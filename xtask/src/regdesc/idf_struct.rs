use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result};
use regex::Regex;
use std::sync::LazyLock;

use super::model::Register;

static TYPEDEF_START: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*typedef\s+(struct|union)(?:\s+[A-Za-z_][A-Za-z0-9_]*)?\s*\{").unwrap()
});
static TYPEDEF_END: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*}\s*([A-Za-z_][A-Za-z0-9_]*)\s*;").unwrap());
static MEMBER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^\s*(?:volatile\s+)?([A-Za-z_][A-Za-z0-9_]*)\s+([A-Za-z_][A-Za-z0-9_]*)(?:\[(\d+)\])?\s*;",
    )
    .unwrap()
});
static EXTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*extern\s+([A-Za-z_][A-Za-z0-9_]*)\s+[A-Za-z_][A-Za-z0-9_]*\s*;")
        .unwrap()
});

#[derive(Debug, Clone)]
struct Member {
    ty: String,
    name: String,
    count: u32,
}

#[derive(Debug, Default)]
struct CLayout {
    structs: HashMap<String, Vec<Member>>,
    scalar_types: HashSet<String>,
    roots: Vec<String>,
}

#[derive(Debug, Clone)]
struct Leaf {
    offset: u32,
    ty: String,
    path: String,
    arrays: Vec<ArrayPosition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ArrayPosition {
    path: String,
    index: u32,
    count: u32,
}

/// Applies array information from the IDF `*_struct.h` companion to parsed
/// registers. The C structure is authoritative for array membership and
/// stride; name-based repeat inference remains a fallback for old headers.
pub fn apply_struct_layout_file(path: &Path, registers: &mut Vec<Register>) -> Result<usize> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("reading IDF structure header {}", path.display()))?;
    let layout = parse_layout(&content);
    let leaves = flatten_layout(&layout);
    let materialized = materialize_direct_arrays(&leaves, registers);
    Ok(materialized + apply_array_hints(&leaves, registers))
}

/// Returns the conventional structure-header path beside a register header.
pub fn companion_path(register_header: &Path) -> Option<std::path::PathBuf> {
    let file_name = register_header.file_name()?.to_str()?;
    let struct_name = file_name.strip_suffix("_reg.h")?.to_owned() + "_struct.h";
    let path = register_header.with_file_name(struct_name);
    path.is_file().then_some(path)
}

fn parse_layout(content: &str) -> CLayout {
    let mut layout = CLayout::default();
    let mut current_kind: Option<&str> = None;
    let mut current_members = Vec::new();
    let mut brace_depth = 0i32;

    for line in content.lines() {
        if current_kind.is_none() {
            if let Some(caps) = TYPEDEF_START.captures(line) {
                current_kind = Some(if &caps[1] == "struct" {
                    "struct"
                } else {
                    "union"
                });
                current_members.clear();
                brace_depth = brace_delta(line);
                continue;
            }
            if let Some(caps) = EXTERN.captures(line) {
                layout.roots.push(caps[1].to_owned());
            }
            continue;
        }

        brace_depth += brace_delta(line);
        if brace_depth == 0 {
            if let Some(caps) = TYPEDEF_END.captures(line) {
                let name = caps[1].to_owned();
                if current_kind == Some("struct") {
                    layout
                        .structs
                        .insert(name, std::mem::take(&mut current_members));
                } else {
                    layout.scalar_types.insert(name);
                }
            }
            current_kind = None;
            continue;
        }

        // Only direct members of a typedef struct describe address layout.
        // Bitfields nested inside register unions are intentionally ignored.
        if current_kind == Some("struct") && brace_depth == 1 {
            if let Some(caps) = MEMBER.captures(line) {
                current_members.push(Member {
                    ty: caps[1].to_owned(),
                    name: caps[2].to_owned(),
                    count: caps
                        .get(3)
                        .and_then(|m| m.as_str().parse().ok())
                        .unwrap_or(1),
                });
            }
        }
    }

    layout.roots.sort();
    layout.roots.dedup();
    layout
}

fn brace_delta(line: &str) -> i32 {
    line.bytes().filter(|&b| b == b'{').count() as i32
        - line.bytes().filter(|&b| b == b'}').count() as i32
}

fn flatten_layout(layout: &CLayout) -> Vec<Leaf> {
    let mut sizes = HashMap::new();
    for root in &layout.roots {
        let mut visiting = HashSet::new();
        let _ = type_size(root, layout, &mut sizes, &mut visiting);
    }

    let mut leaves = Vec::new();
    for root in &layout.roots {
        let mut arrays = Vec::new();
        flatten_type(root, 0, "", &mut arrays, layout, &sizes, &mut leaves);
    }
    leaves.sort_by_key(|leaf| leaf.offset);
    leaves.dedup_by_key(|leaf| leaf.offset);
    leaves
}

fn type_size(
    ty: &str,
    layout: &CLayout,
    cache: &mut HashMap<String, u32>,
    visiting: &mut HashSet<String>,
) -> Option<u32> {
    if ty == "uint32_t" || layout.scalar_types.contains(ty) {
        return Some(4);
    }
    if let Some(size) = cache.get(ty) {
        return Some(*size);
    }
    if !visiting.insert(ty.to_owned()) {
        return None;
    }
    let members = layout.structs.get(ty)?;
    let mut size = 0u32;
    for member in members {
        size = size.checked_add(
            type_size(&member.ty, layout, cache, visiting)?.checked_mul(member.count)?,
        )?;
    }
    visiting.remove(ty);
    cache.insert(ty.to_owned(), size);
    Some(size)
}

fn flatten_type(
    ty: &str,
    base: u32,
    path: &str,
    arrays: &mut Vec<ArrayPosition>,
    layout: &CLayout,
    sizes: &HashMap<String, u32>,
    leaves: &mut Vec<Leaf>,
) {
    if layout.scalar_types.contains(ty) {
        leaves.push(Leaf {
            offset: base,
            ty: ty.to_owned(),
            path: path.to_owned(),
            arrays: arrays.clone(),
        });
        return;
    }
    let Some(members) = layout.structs.get(ty) else {
        return;
    };
    let mut offset = base;
    for member in members {
        let Some(member_size) = sizes
            .get(&member.ty)
            .copied()
            .or_else(|| {
                (member.ty == "uint32_t" || layout.scalar_types.contains(&member.ty))
                    .then_some(4)
            })
        else {
            continue;
        };
        for index in 0..member.count {
            let child_path = if path.is_empty() {
                member.name.clone()
            } else {
                format!("{path}.{}", member.name)
            };
            if member.count > 1 && member.ty != "uint32_t" {
                arrays.push(ArrayPosition {
                    path: child_path.clone(),
                    index,
                    count: member.count,
                });
            }
            flatten_type(
                &member.ty,
                offset + index * member_size,
                &child_path,
                arrays,
                layout,
                sizes,
                leaves,
            );
            if member.count > 1 && member.ty != "uint32_t" {
                arrays.pop();
            }
        }
        offset += member_size * member.count;
    }
}

fn apply_array_hints(leaves: &[Leaf], registers: &mut [Register]) -> usize {
    let by_offset: HashMap<u32, usize> = registers
        .iter()
        .enumerate()
        .map(|(index, register)| (register.addr, index))
        .collect();
    let mut candidates: HashMap<(String, u32), Vec<(u32, u32)>> = HashMap::new();

    for leaf in leaves {
        if !by_offset.contains_key(&leaf.offset) {
            continue;
        }
        for array in &leaf.arrays {
            candidates
                .entry((array.path.clone(), array.count))
                .or_default()
                .push((array.index, leaf.offset));
        }
    }

    let mut applied = 0usize;
    for ((_path, count), mut items) in candidates {
        items.sort_unstable();
        items.dedup();
        if items.len() != count as usize {
            continue;
        }
        let Some(indices): Option<Vec<usize>> = items
            .iter()
            .map(|(_, offset)| by_offset.get(offset).copied())
            .collect()
        else {
            continue;
        };
        let names: Vec<&str> = indices
            .iter()
            .map(|&index| registers[index].name.as_str())
            .collect();
        let item_indices: Vec<u32> = items.iter().map(|(index, _)| *index).collect();
        let Some(template) = indexed_name_template(&names, &item_indices) else {
            continue;
        };
        let repeat_name = if template.ends_with("_REG") {
            template
        } else {
            format!("{template}_REG")
        };
        for (&register_index, &item_index) in indices.iter().zip(&item_indices) {
            if registers[register_index].repeat.is_none()
                && registers[register_index].repeat_name_hint.is_none()
            {
                registers[register_index]
                    .set_repeat_hint(repeat_name.clone(), item_index as i32);
                applied += 1;
            }
        }
    }
    applied
}

fn materialize_direct_arrays(leaves: &[Leaf], registers: &mut Vec<Register>) -> usize {
    let mut groups: HashMap<(String, String, u32), Vec<&Leaf>> = HashMap::new();
    for leaf in leaves {
        let Some(array) = leaf.arrays.last() else {
            continue;
        };
        if array.path != leaf.path || leaf.arrays.len() != 1 {
            continue;
        }
        groups
            .entry((leaf.path.clone(), leaf.ty.clone(), array.count))
            .or_default()
            .push(leaf);
    }

    let mut added = 0usize;
    for ((_path, _ty, count), mut array_leaves) in groups {
        array_leaves.sort_by_key(|leaf| leaf.arrays[0].index);
        if array_leaves.len() != count as usize {
            continue;
        }

        let by_offset: HashMap<u32, usize> = registers
            .iter()
            .enumerate()
            .map(|(index, register)| (register.addr, index))
            .collect();
        let existing: Vec<_> = array_leaves
            .iter()
            .filter_map(|leaf| {
                by_offset
                    .get(&leaf.offset)
                    .map(|&register_index| (leaf.arrays[0].index, register_index))
            })
            .collect();
        if existing.len() < 2 {
            continue;
        }

        // A shared C register type is authoritative for bit layout. IDF may
        // still emit pin-specific reset/access metadata that cannot be
        // represented by one SVD array element; normalize only those
        // attributes when every member has the same physical fields.
        let prototype_fields = registers[existing[0].1].fields.clone();
        let same_physical_layout = existing.iter().all(|(_, register_index)| {
            let fields = &registers[*register_index].fields;
            fields.len() == prototype_fields.len()
                && prototype_fields.iter().all(|prototype| {
                    fields.iter().any(|field| {
                        field.shift == prototype.shift && field.mask == prototype.mask
                    })
                })
        });
        if same_physical_layout {
            for (_, register_index) in &existing {
                for field in &mut registers[*register_index].fields {
                    if let Some(prototype) = prototype_fields.iter().find(|prototype| {
                        prototype.shift == field.shift && prototype.mask == field.mask
                    }) {
                        field.access = prototype.access.clone();
                        field.default = prototype.default;
                    }
                }
            }
        } else {
            let prototype_name = registers[existing[0].1]
                .name
                .trim_end_matches("_REG")
                .to_owned();
            for (_, register_index) in &existing {
                let register_name = registers[*register_index]
                    .name
                    .trim_end_matches("_REG")
                    .to_owned();
                registers[*register_index].fields = prototype_fields
                    .iter()
                    .cloned()
                    .map(|mut field| {
                        if field.name.starts_with(&prototype_name) {
                            field.name = field.name.replacen(
                                &prototype_name,
                                &register_name,
                                1,
                            );
                        }
                        field
                    })
                    .collect();
            }
        }
        if existing.len() == array_leaves.len() {
            continue;
        }

        let names: Vec<&str> = existing
            .iter()
            .map(|(_, register_index)| registers[*register_index].name.as_str())
            .collect();
        let indices: Vec<u32> = existing.iter().map(|(index, _)| *index).collect();
        let Some(template) = indexed_name_template(&names, &indices) else {
            continue;
        };
        let prototype = registers[existing[0].1].clone();

        for leaf in array_leaves {
            if by_offset.contains_key(&leaf.offset) {
                continue;
            }
            let mut register = prototype.clone();
            register.addr = leaf.offset;
            register.name = template.replace("$n", &leaf.arrays[0].index.to_string());
            register.repeat = None;
            register.repeat_name_hint = None;
            register.repeat_index_hint = None;
            registers.push(register);
            added += 1;
        }
    }
    registers.sort_by_key(|register| register.addr);
    added
}

fn indexed_name_template(names: &[&str], indices: &[u32]) -> Option<String> {
    let first = *names.first()?;
    let first_index = indices.first()?.to_string();

    for (start, _) in first.match_indices(&first_index) {
        let end = start + first_index.len();
        if start > 0 && first.as_bytes()[start - 1].is_ascii_digit() {
            continue;
        }
        if end < first.len() && first.as_bytes()[end].is_ascii_digit() {
            continue;
        }
        let prefix = &first[..start];
        let suffix = &first[end..];
        if names
            .iter()
            .zip(indices)
            .all(|(name, index)| *name == format!("{prefix}{index}{suffix}"))
        {
            return Some(format!("{prefix}$n{suffix}"));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::model::Register;

    #[test]
    fn derives_arrays_through_nested_structs() {
        let layout = parse_layout(
            r#"
typedef union {
    uint32_t val;
} foo_conf_reg_t;
typedef struct {
    volatile foo_conf_reg_t conf;
    uint32_t reserved[3];
} foo_channel_t;
typedef struct {
    volatile foo_channel_t channel[3];
} foo_dev_t;
extern foo_dev_t FOO;
"#,
        );
        assert_eq!(layout.roots, vec!["foo_dev_t"]);
        assert!(layout.scalar_types.contains("foo_conf_reg_t"));
        assert!(layout.structs.contains_key("foo_channel_t"));
        assert!(layout.structs.contains_key("foo_dev_t"));
        let leaves = flatten_layout(&layout);
        assert_eq!(
            leaves.iter().map(|leaf| leaf.offset).collect::<Vec<_>>(),
            vec![0, 16, 32]
        );
        assert_eq!(leaves[0].arrays[0].count, 3);
    }

    #[test]
    fn applies_array_hints_using_offsets_not_naming_rules() {
        let leaves = vec![
            leaf(0, 0),
            leaf(4, 1),
            leaf(8, 2),
        ];
        let mut registers = vec![
            register("FOO_MAGIC_SLOT0_REG", 0),
            register("FOO_MAGIC_SLOT1_REG", 4),
            register("FOO_MAGIC_SLOT2_REG", 8),
        ];
        assert_eq!(apply_array_hints(&leaves, &mut registers), 3);
        assert_eq!(
            registers[0].repeat_name_hint.as_deref(),
            Some("FOO_MAGIC_SLOT$n_REG")
        );
    }

    #[test]
    fn materializes_sparse_direct_arrays_from_struct_layout() {
        let leaves = (0..4)
            .map(|index| Leaf {
                offset: index * 4,
                ty: "foo_reg_t".into(),
                path: "slot".into(),
                arrays: vec![ArrayPosition {
                    path: "slot".into(),
                    index,
                    count: 4,
                }],
            })
            .collect::<Vec<_>>();
        let mut registers = vec![
            register("FOO_SLOT0_REG", 0),
            register("FOO_SLOT3_REG", 12),
        ];
        assert_eq!(materialize_direct_arrays(&leaves, &mut registers), 2);
        assert_eq!(
            registers.iter().map(|register| register.name.as_str()).collect::<Vec<_>>(),
            vec![
                "FOO_SLOT0_REG",
                "FOO_SLOT1_REG",
                "FOO_SLOT2_REG",
                "FOO_SLOT3_REG"
            ]
        );
    }

    #[test]
    fn chooses_the_index_that_varies() {
        let names = ["DMA_OUT_CONF0_CH0_REG", "DMA_OUT_CONF0_CH1_REG"];
        assert_eq!(
            indexed_name_template(&names, &[0, 1]).as_deref(),
            Some("DMA_OUT_CONF0_CH$n_REG")
        );
    }

    fn leaf(offset: u32, index: u32) -> Leaf {
        Leaf {
            offset,
            ty: "foo_reg_t".into(),
            path: "slot".into(),
            arrays: vec![ArrayPosition {
                path: "slot".into(),
                index,
                count: 3,
            }],
        }
    }

    fn register(name: &str, addr: u32) -> Register {
        Register {
            name: name.into(),
            addr,
            fields: Vec::new(),
            description: String::new(),
            visible: true,
            size: 4,
            repeat: None,
            repeat_name_hint: None,
            repeat_index_hint: None,
            is_mem_region: false,
            expand_context: Default::default(),
        }
    }
}
