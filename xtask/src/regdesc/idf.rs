use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use regex::Regex;
use std::sync::LazyLock;

use super::model::{Field, Peripheral, Register, RegisterGroup};
use super::util::parse_verilog_number;

static REG_VALUE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\(\(?([A-Z_0-9]+_BASE)(?:\(i\))?\)?\s*\+\s*([0-9A-Fa-fx]+)(?:\s*\+\s*\(i\)\s*\*\s*([0-9A-Fa-fx]+))?\)",
    )
    .unwrap()
});

static FIELD_DESC_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"/\*\*?\s*([A-Z_0-9]+)\s*:\s*([A-Z0-9_/]+)\s*;\s*bitpos:\s*\[?[0-9:]+\]\s*;\s*default\s*:\s*([x0-9a-hA-F']+)",
    )
    .unwrap()
});

static FIELD_DESC_SHORT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"/\*\*?\s*([A-Z_0-9]+)\s*:\s*([A-Z0-9_/]+)\s*;\s*bitpos:\s*\[?[0-9:]+\]\s*;?",
    )
    .unwrap()
});

static DEFAULT_CONT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"default\s*:\s*([x0-9a-hA-F']+)").unwrap());

static BIT_FIELD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#define\s*([^\s(]+)\s*\(BIT\((\d+)\)\)").unwrap());

static DESC_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"/\*\s*description:\s*(.*)\*/").unwrap());

static VERILOG_RADIX_FIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+)'([hdb])([0-9a-fA-F]+)").unwrap());

/// regdesc.yml entry keys whose IDF header name differs from `{stem}_reg.h`.
static HEADER_ALIASES: &[(&str, &str)] = &[
    ("core0_interrupt_reg.csv", "interrupt_core0_reg.h"),
    ("core1_interrupt_reg.csv", "interrupt_core1_reg.h"),
    ("efuse_mem_reg.csv", "efuse_reg.h"),
    ("hp_sys_reg.csv", "hp_system_reg.h"),
    ("i2c_ext_reg.csv", "i2c_reg.h"),
    ("iomux_reg.csv", "io_mux_reg.h"),
    ("lcdcam_reg.csv", "lcd_cam_reg.h"),
    ("lp_aonclkrst_reg.csv", "lp_clkrst_reg.h"),
    ("lp_i2c_ext_reg.csv", "lp_i2c_reg.h"),
    ("lp_periclkrst_reg.csv", "lp_peri_clkrst_reg.h"),
    ("lp_sys_reg.csv", "lp_system_reg.h"),
    ("pwm_reg.csv", "mcpwm_reg.h"),
    ("spi1_mem_reg_c.csv", "spi1_mem_c_reg.h"),
    // The configured SPI1 instance is FLASH_SPI1, not the PSRAM slave block.
    ("spi1_mem_reg_s.csv", "spi1_mem_c_reg.h"),
    ("sdio_host_reg.csv", "sdmmc_reg.h"),
    ("spi2_reg_s.csv", "spi_reg.h"),
    ("spi3_reg.csv", "spi_reg.h"),
    ("spi_mem_reg_c.csv", "spi_mem_c_reg.h"),
    // The configured SPI0 instance is FLASH_SPI0, not the PSRAM slave block.
    ("spi_mem_reg_s.csv", "spi_mem_c_reg.h"),
    ("sys_timer_reg.csv", "systimer_reg.h"),
    ("timers_reg.csv", "timer_group_reg.h"),
];

#[derive(Debug, Default)]
pub struct IdfParseStats {
    pub registers: usize,
    pub unknowns: usize,
}

/// Maps a `regdesc.yml` entry key to the corresponding ESP-IDF register header.
pub fn header_for_entry(entry_key: &str, override_name: Option<&str>) -> String {
    if let Some(name) = override_name {
        return name.to_owned();
    }

    HEADER_ALIASES
        .iter()
        .find_map(|(key, header)| (*key == entry_key).then(|| (*header).to_owned()))
        .unwrap_or_else(|| entry_key.replace(".csv", ".h"))
}

/// Locates a register header under an ESP-IDF checkout for the given chip.
pub fn resolve_header_path(idf_path: &Path, chip: &str, header_file: &str) -> Option<PathBuf> {
    let soc_root = idf_path.join("components").join("soc").join(chip);
    let candidates = [
        soc_root.join("register").join("soc").join(header_file),
        soc_root.join("include").join("soc").join(header_file),
        soc_root.join("include").join("modem").join(header_file),
    ];

    candidates.into_iter().find(|path| path.is_file())
}

/// Parses one ESP-IDF legacy `#define` register header into a `Peripheral`.
pub fn read_peripheral_header(
    content: &str,
    peripheral_name: Option<&str>,
) -> Result<(Peripheral, IdfParseStats)> {
    let (registers, unknowns) = read_reg_header_legacy(content)?;
    let stats = IdfParseStats {
        registers: registers.len(),
        unknowns: unknowns.len(),
    };

    let name = match peripheral_name {
        Some(name) if !name.is_empty() => name.to_owned(),
        _ => probe_peripheral_name(&registers)
            .context("Failed to determine peripheral name from IDF header")?,
    };

    Ok((
        Peripheral {
            name: name.clone(),
            name_prefixes: vec![name],
            register_groups: vec![RegisterGroup {
                name: None,
                description: String::new(),
                registers,
                visible: true,
                repeat: None,
                offset: 0,
            }],
            description: String::new(),
        },
        stats,
    ))
}

/// Parses one ESP-IDF legacy `#define` register header from disk.
pub fn read_peripheral_header_file(
    path: &Path,
    peripheral_name: Option<&str>,
) -> Result<(Peripheral, IdfParseStats)> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("reading IDF header {}", path.display()))?;
    read_peripheral_header(&content, peripheral_name)
}

/// Guesses the peripheral name from register name prefixes in a header file.
fn probe_peripheral_name(registers: &[Register]) -> Option<String> {
    let names: Vec<&str> = registers.iter().map(|reg| reg.name.as_str()).collect();
    if names.len() < 2 {
        return names.first().and_then(|name| name.rsplit_once('_').map(|(prefix, _)| prefix.to_owned()));
    }

    let common_prefix = common_prefix(&names);
    let pos = common_prefix.rfind('_')?;
    Some(common_prefix[..pos].to_owned())
}

/// Finds the shared start of a bunch of strings.
fn common_prefix(values: &[&str]) -> String {
    if values.is_empty() {
        return String::new();
    }

    let mut prefix = values[0].to_owned();
    for value in &values[1..] {
        while !value.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix
}

/// Core legacy header parser — port of regdesc's `read_header_legacy.py` with
/// support for `/**` comments, multi-line field descriptions, and macro aliases.
pub fn read_reg_header_legacy(content: &str) -> Result<(Vec<Register>, HashMap<String, UnknownDefine>)> {
    let raw_defines = collect_raw_defines(content);
    let mut regs = Vec::new();
    let mut unknowns: HashMap<String, UnknownDefine> = HashMap::new();
    let mut last_reg: Option<usize> = None;
    let mut fields: HashMap<String, FieldState> = HashMap::new();
    let mut template_fields: HashMap<String, FieldState> = HashMap::new();
    let mut values: HashMap<String, u32> = HashMap::new();
    let mut desc: Option<String> = None;
    let mut last_line_desc = false;
    let mut last_desc: Option<String> = None;
    let mut pending_field_desc: Option<Vec<String>> = None;
    let mut pending_reg_desc: Option<String> = None;
    let mut pending_reg_desc_lines: Option<Vec<String>> = None;
    let mut line_counter = 0usize;

    for line in content.lines() {
        line_counter += 1;
        let mut line = line.trim_end().to_owned();
        let mut desc_line = false;

        if let Some(ref mut block) = pending_reg_desc_lines {
            block.push(line.clone());
            if line.contains("*/") {
                pending_reg_desc = Some(register_comment_description(block));
                pending_reg_desc_lines = None;
            }
            continue;
        }

        if let Some(ref mut block) = pending_field_desc {
            block.push(line.clone());
            if line.contains("*/") {
                apply_field_desc_block(
                    block,
                    &mut fields,
                    &mut unknowns,
                    last_reg.as_ref().map(|idx| &mut regs[*idx]),
                )?;
                pending_field_desc = None;
            }
            continue;
        }

        if line.contains("#define") {
            line = line.replace("( i )", "(i)");
            let items: Vec<&str> = line.split_whitespace().collect();
            if items.len() < 3 {
                continue;
            }

            let define = items[1];
            let value = items[2..].join(" ");

            if define.ends_with("_H_") || define.ends_with("_H__") {
                continue;
            }

            if define.ends_with("_MEM_SIZE_BYTES") {
                if let Some(mem_name) = define.strip_suffix("_SIZE_BYTES") {
                    if let Ok(size) = parse_int(strip_comment(&value).trim_end_matches('U')) {
                        if let Some(reg) = regs.iter_mut().find(|reg| reg.name == mem_name) {
                            reg.size = size;
                            reg.is_mem_region = true;
                        }
                    }
                }
                continue;
            }

            if is_register_define(define) || is_mem_region_define(define) {
                if let Some(addr) = resolve_register_offset(&value, &raw_defines) {
                    let mut name = define.to_owned();
                    if name.ends_with("(i)") {
                        name.truncate(name.len() - 3);
                    }

                    let is_mem_region = is_mem_region_define(define);
                    last_reg = Some(regs.len());
                    fields.clear();
                    regs.push(Register {
                        name,
                        addr,
                        fields: Vec::new(),
                        description: pending_reg_desc.take().unwrap_or_default(),
                        visible: true,
                        size: if is_mem_region { 1 } else { 4 },
                        repeat: None,
                        repeat_name_hint: None,
                        repeat_index_hint: None,
                        is_mem_region,
                        expand_context: Default::default(),
                    });
                } else if value.contains("_BASE") {
                    bail!("At line {line_counter}: invalid register define `{line}`");
                } else {
                    unknowns.insert(
                        define.to_owned(),
                        UnknownDefine {
                            name: define.to_owned(),
                            value,
                        },
                    );
                }
            } else if is_legacy_field_define(define, &raw_defines) {
                let shift_name = format!("{define}_S");
                let Some(shift) = raw_defines
                    .get(&shift_name)
                    .and_then(|value| resolve_numeric_define(value, &raw_defines, 0))
                else {
                    continue;
                };
                let Some(mask) = parse_legacy_field_mask(&value, &raw_defines) else {
                    unknowns.insert(
                        define.to_owned(),
                        UnknownDefine {
                            name: define.to_owned(),
                            value,
                        },
                    );
                    continue;
                };

                if let Some(reg_idx) = last_reg {
                    let mut field = FieldState::new(define);
                    field.shift = shift;
                    field.mask = Some(mask);
                    if let Some(description) = last_desc.take() {
                        field.description = description;
                    }
                    regs[reg_idx].fields.push(field.to_field());
                    fields.insert(define.to_owned(), field);
                }
            } else if is_field_metadata_define(define, &raw_defines) {
                let field_info_type = define.chars().last().unwrap();
                let field_name = &define[..define.len() - 2];

                if field_info_type == 'S' {
                    let shift_value = strip_comment(&value).trim_end_matches('U');
                    let Ok(shift) = parse_int(shift_value) else {
                        unknowns.insert(
                            define.to_owned(),
                            UnknownDefine {
                                name: define.to_owned(),
                                value,
                            },
                        );
                        continue;
                    };
                    if shift > 31 {
                        unknowns.insert(
                            define.to_owned(),
                            UnknownDefine {
                                name: define.to_owned(),
                                value,
                            },
                        );
                        continue;
                    }
                }

                unknowns.remove(field_name);

                let target = if last_reg.is_some() {
                    FieldTarget::Register
                } else {
                    FieldTarget::Template
                };

                let field = match target {
                    FieldTarget::Register => fields.entry(field_name.to_owned()).or_insert_with(|| {
                        let field = FieldState::new(field_name);
                        if let Some(reg_idx) = last_reg {
                            regs[reg_idx].fields.push(field.to_field());
                        }
                        field
                    }),
                    FieldTarget::Template => template_fields
                        .entry(field_name.to_owned())
                        .or_insert_with(|| FieldState::new(field_name)),
                };

                let mut value = value.as_str();
                if value.starts_with('(') {
                    value = &value[1..];
                }
                if value.ends_with(')') {
                    value = &value[..value.len() - 1];
                }

                match field_info_type {
                    'S' => {
                        let shift_value = strip_comment(value).trim_end_matches('U');
                        let resolved = if let Ok(parsed) = parse_int(shift_value) {
                            parsed
                        } else if let Some(resolved) = values.get(shift_value) {
                            *resolved
                        } else if let Some(resolved) =
                            resolve_numeric_define(shift_value, &raw_defines, 0)
                        {
                            resolved
                        } else {
                            bail!("At line {line_counter}: invalid shift `{line}`");
                        };
                        field.shift = resolved;
                    }
                    'V' => {
                        let mut mask_value = strip_comment(value).to_owned();
                        if mask_value.contains("BIT(") {
                            if let Some(bit) = Regex::new(r"BIT\((\d+)\)")
                                .unwrap()
                                .captures(&mask_value)
                            {
                                let shift: u32 = bit[1].parse().unwrap_or(0);
                                mask_value = format!("{:#x}", 1u32 << shift);
                            }
                        }
                        mask_value = mask_value.trim_end_matches('U').to_owned();
                        field.mask = Some(parse_int(&mask_value).or_else(|_| {
                            values
                                .get(mask_value.as_str())
                                .copied()
                                .or_else(|| {
                                    resolve_numeric_define(&mask_value, &raw_defines, 0)
                                })
                                .ok_or_else(|| anyhow::anyhow!("unknown mask value `{mask_value}`"))
                        })?);
                    }
                    _ => {}
                }

                if let Some(description) = last_desc.take() {
                    field.description = description;
                }
                if last_reg.is_some() {
                    sync_field(&mut regs, field);
                }

                if let Some(mask) = field.mask {
                    values.insert(define.to_owned(), mask);
                }
            } else if define.ends_with("BASE(i)") {
                continue;
            } else if let Some(caps) = BIT_FIELD_REGEX.captures(&line) {
                let name = caps[1].to_owned();
                let shift: u32 = caps[2].parse().unwrap_or(0);

                if !name.to_ascii_uppercase().contains("MASK")
                    && last_reg.is_some()
                    && !fields.contains_key(&name)
                {
                    let field = FieldState {
                        name: name.clone(),
                        shift,
                        mask: Some(1 << shift),
                        ..FieldState::new(&name)
                    };
                    if let Some(reg_idx) = last_reg {
                        regs[reg_idx].fields.push(field.to_field());
                    }
                    fields.insert(name, field);
                }
            } else {
                unknowns.insert(
                    define.to_owned(),
                    UnknownDefine {
                        name: define.to_owned(),
                        value,
                    },
                );
            }
        } else if line.contains("/**") && line.contains(" register") && !line.contains("bitpos") {
            let block = vec![line.clone()];
            if line.contains("*/") {
                pending_reg_desc = Some(register_comment_description(&block));
            } else {
                pending_reg_desc_lines = Some(block);
            }
        } else if line.contains("bitpos") && (line.contains("/*") || line.contains("/**")) {
            pending_field_desc = Some(vec![line.clone()]);
            if line.contains("*/") {
                apply_field_desc_block(
                    pending_field_desc.as_ref().unwrap(),
                    &mut fields,
                    &mut unknowns,
                    last_reg.as_ref().map(|idx| &mut regs[*idx]),
                )?;
                pending_field_desc = None;
            }
        } else if line.contains("/*description:") {
            desc_line = true;
            desc = Some(line);
        } else if last_line_desc {
            desc_line = true;
            let Some(mut current) = desc.take() else {
                bail!("At line {line_counter}: malformed description block `{line}`");
            };
            current.push(' ');
            current.push_str(&line);
            desc = Some(current);
        }

        if last_line_desc && !desc_line {
            if let Some(current) = desc.take() {
                let Some(caps) = DESC_REGEX.captures(&current) else {
                    bail!("At line {line_counter}: malformed description `{current}`");
                };
                last_desc = Some(caps[1].trim().to_owned());
            }
        }

        last_line_desc = desc_line;
    }

    if pending_field_desc.is_some() {
        bail!("Unterminated field description block at end of file");
    }

    apply_template_fields(&mut regs, &template_fields);

    Ok((regs, unknowns))
}

enum FieldTarget {
    Register,
    Template,
}

/// Returns true for register `#define` names (`*_REG`, `*_REG(i)`).
fn is_register_define(define: &str) -> bool {
    define.ends_with("_REG") || define.ends_with("_REG(i)")
}

/// Returns true for crypto/memory buffer `#define` names (`*_MEM`, not `*_REG`).
fn is_mem_region_define(define: &str) -> bool {
    define.ends_with("_MEM") && !define.ends_with("_REG")
}

/// Distinguishes generated field metadata (`FOO_M`, `FOO_V`, `FOO_S`) from
/// real fields whose names happen to end in those letters (`FRAC_M`).
///
/// IDF emits all three metadata macros for a field. Requiring the sibling
/// `_V` and `_S` definitions avoids creating phantom zero-width fields.
fn is_field_metadata_define(define: &str, raw_defines: &HashMap<String, String>) -> bool {
    let Some((field_name, suffix)) = define.rsplit_once('_') else {
        return false;
    };
    if !matches!(suffix, "M" | "V" | "S") {
        return false;
    }
    raw_defines.contains_key(&format!("{field_name}_V"))
        && raw_defines.contains_key(&format!("{field_name}_S"))
}

fn is_legacy_field_define(define: &str, raw_defines: &HashMap<String, String>) -> bool {
    !define.ends_with("_S")
        && !define.ends_with("_V")
        && !define.ends_with("_M")
        && raw_defines.contains_key(&format!("{define}_S"))
        && !raw_defines.contains_key(&format!("{define}_V"))
}

fn parse_legacy_field_mask(
    value: &str,
    raw_defines: &HashMap<String, String>,
) -> Option<u32> {
    static BIT: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\(?BIT\(\d+\)\)?$").unwrap());
    let value = strip_comment(value).trim();
    if BIT.is_match(value) {
        Some(1)
    } else {
        resolve_numeric_define(value, raw_defines, 0)
    }
}

/// Pulls the human-readable text out of a `/** FOO register ... */` block.
fn register_comment_description(lines: &[String]) -> String {
    static REGISTER_NAME: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"/\*\*?\s*[A-Z0-9_]+\s+register\s*").unwrap());

    let text = lines.join("\n");
    let without_opener = REGISTER_NAME.replace(&text, "");
    let without_closer = without_opener.replace("*/", "");
    without_closer
        .lines()
        .map(|line| line.trim().trim_start_matches('*').trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Collects all `#define` values for alias / address resolution.
fn collect_raw_defines(content: &str) -> HashMap<String, String> {
    let mut raw = HashMap::new();
    for line in content.lines() {
        if !line.contains("#define") {
            continue;
        }
        let items: Vec<&str> = line.split_whitespace().collect();
        if items.len() >= 3 {
            raw.insert(items[1].to_owned(), items[2..].join(" "));
        }
    }
    raw
}

/// Resolves a register offset from a direct `(BASE + offset)` or a macro alias chain.
fn resolve_register_offset(value: &str, raw_defines: &HashMap<String, String>) -> Option<u32> {
    resolve_register_offset_inner(value, raw_defines, 0)
}

fn resolve_register_offset_inner(
    value: &str,
    raw_defines: &HashMap<String, String>,
    depth: u32,
) -> Option<u32> {
    if depth > 16 {
        return None;
    }

    let value = strip_comment(value).trim();
    if let Some(caps) = REG_VALUE_REGEX.captures(value) {
        return parse_offset(&caps[2]);
    }

    let token = value
        .split_whitespace()
        .next()
        .unwrap_or(value)
        .trim_matches(|c: char| c == '(' || c == ')');

    raw_defines
        .get(token)
        .and_then(|next| resolve_register_offset_inner(next, raw_defines, depth + 1))
}

fn parse_offset(offset: &str) -> Option<u32> {
    let offset = offset.trim();
    if let Some(hex) = offset.strip_prefix("0x").or_else(|| offset.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        offset.parse().ok()
    }
}

fn resolve_numeric_define(
    value: &str,
    raw_defines: &HashMap<String, String>,
    depth: u32,
) -> Option<u32> {
    if depth > 16 {
        return None;
    }
    let value = strip_comment(value)
        .trim()
        .trim_matches(|c: char| c == '(' || c == ')')
        .trim_end_matches(['U', 'L']);
    if let Ok(value) = parse_int(value) {
        return Some(value);
    }
    raw_defines
        .get(value)
        .and_then(|next| resolve_numeric_define(next, raw_defines, depth + 1))
}

/// Applies shared field templates (e.g. IO_MUX pin fields) to registers that lack fields.
fn apply_template_fields(regs: &mut [Register], template_fields: &HashMap<String, FieldState>) {
    if template_fields.is_empty() {
        return;
    }

    let template: Vec<Field> = template_fields
        .values()
        .map(FieldState::to_field)
        .collect();

    for reg in regs {
        if reg.fields.is_empty()
            && (reg.name.starts_with("IO_MUX_GPIO") || reg.name.starts_with("PERIPHS_IO_MUX"))
        {
            reg.fields = template.clone();
        }
    }
}

#[derive(Debug, Clone)]
struct FieldState {
    name: String,
    shift: u32,
    mask: Option<u32>,
    access: String,
    default: u64,
    description: String,
}

impl FieldState {
    /// Creates a field placeholder before shift/mask defines are seen.
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            shift: 0,
            mask: None,
            access: String::new(),
            default: 0,
            description: String::new(),
        }
    }

    /// Converts this mutable parser state into an output `Field`.
    fn to_field(&self) -> Field {
        Field {
            name: self.name.clone(),
            shift: self.shift,
            mask: self.mask.unwrap_or(0),
            access: self.access.clone(),
            default: self.default,
            description: self.description.clone(),
            visible: true,
            min_val: None,
            max_val: None,
            repeat: None,
            repeat_name_hint: None,
            repeat_index_hint: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnknownDefine {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub value: String,
}

/// Applies a collected field-description comment block to the in-progress model.
fn apply_field_desc_block(
    lines: &[String],
    fields: &mut HashMap<String, FieldState>,
    unknowns: &mut HashMap<String, UnknownDefine>,
    last_reg: Option<&mut Register>,
) -> Result<()> {
    let text = lines.join(" ");
    let (field_name, access, default_spec) = if let Some(caps) = FIELD_DESC_REGEX.captures(&text) {
        (
            caps[1].to_owned(),
            normalize_access(&caps[2]),
            caps[3].to_owned(),
        )
    } else if let Some(caps) = FIELD_DESC_SHORT_REGEX.captures(&text) {
        let default_spec = DEFAULT_CONT_REGEX
            .captures(&text)
            .map(|c| c[1].to_owned())
            .unwrap_or_else(|| "0".to_owned());
        (caps[1].to_owned(), normalize_access(&caps[2]), default_spec)
    } else {
        bail!("Could not parse field description `{text}`");
    };

    let default = parse_default_spec(&default_spec)?;
    unknowns.remove(&field_name);

    if let Some(field) = fields.get_mut(&field_name) {
        field.access = access;
        field.default = default;
        if let Some(reg) = last_reg {
            sync_field(std::slice::from_mut(reg), field);
        }
        return Ok(());
    }

    let field = FieldState {
        name: field_name.clone(),
        shift: 0,
        mask: None,
        access,
        default,
        description: String::new(),
    };

    if let Some(reg) = last_reg {
        reg.fields.push(field.to_field());
    }
    fields.insert(field_name, field);

    Ok(())
}

/// Copies the latest field state back into the active register's field list.
fn sync_field(regs: &mut [Register], field: &FieldState) {
    let Some(reg) = regs.last_mut() else {
        return;
    };
    if let Some(existing) = reg.fields.iter_mut().find(|f| f.name == field.name) {
        *existing = field.to_field();
    }
}

/// Strips inline `/* ... */` comments from a `#define` value.
fn strip_comment(value: &str) -> &str {
    value.split("/*").next().unwrap_or(value).trim()
}

/// Parses integer literals in decimal or hex.
fn parse_int(value: &str) -> Result<u32> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).map_err(Into::into)
    } else {
        value.parse().map_err(Into::into)
    }
}

/// Normalizes access strings from IDF headers (`R/W`, `RO`, `HRO`, …).
fn normalize_access(access: &str) -> String {
    let access = access.replace('_', "/");
    if access == "HRO" {
        return "HRO".to_owned();
    }
    if matches!(access.as_str(), "RO" | "R" | "RC") {
        return access;
    }
    if access.contains("WTC") || access.contains("W1C") {
        return "RW1C".to_owned();
    }
    if access.contains("WTS") || access.contains("W1S") {
        return "RW1S".to_owned();
    }
    if access.contains("R/W") || access.starts_with("RW") || access == "RO/WT" {
        return "RW".to_owned();
    }
    match access.as_str() {
        "WD" | "W" | "WO" | "WS" | "WT" => "W".to_owned(),
        other => other.replace('/', ""),
    }
}

/// Parses a field default value, including Verilog-style literals.
fn parse_default_spec(def_spec: &str) -> Result<u64> {
    let def_spec = if def_spec == "x" { "0" } else { def_spec };

    if VERILOG_RADIX_FIX.is_match(def_spec) {
        if let Some(caps) = VERILOG_RADIX_FIX.captures(def_spec) {
            let mut radix = caps[2].to_string();
            if radix == "b" && caps[3].parse::<u32>().unwrap_or(0) > 1 {
                radix = "d".to_owned();
            }
            let fixed = format!("{}'{}{}", &caps[1], radix, &caps[3]);
            let (default, _, _) = parse_verilog_number(&fixed);
            return Ok(default);
        }
    }

    let (default, width, _) = parse_verilog_number(def_spec);
    if width > 0 || def_spec.chars().all(|c| c.is_ascii_digit()) {
        return Ok(default);
    }

    def_spec.parse().map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mem_region_with_size_bytes() {
        let content = r#"
#define DR_REG_ECDSA_BASE 0x60000000
#define ECDSA_Z_MEM (DR_REG_ECDSA_BASE + 0x440)
#define ECDSA_Z_MEM_SIZE_BYTES 48
"#;
        let (regs, _unknowns) = read_reg_header_legacy(content).unwrap();
        assert_eq!(regs.len(), 1);
        assert!(regs[0].is_mem_region);
        assert_eq!(regs[0].name, "ECDSA_Z_MEM");
        assert_eq!(regs[0].size, 48);
    }

    #[test]
    fn parses_multiline_field_default() {
        let content = r#"
#define FOO_BAR_REG (DR_REG_FOO_BASE + 0x0)
/** FOO_BAR_BAZ : R/W; bitpos: [9:8];
 *  default: 0;
 */
#define FOO_BAR_BAZ 0x3U
#define FOO_BAR_BAZ_M (FOO_BAR_BAZ_V << FOO_BAR_BAZ_S)
#define FOO_BAR_BAZ_V 0x3U
#define FOO_BAR_BAZ_S 8
"#;
        let (regs, unknowns) = read_reg_header_legacy(content).unwrap();
        assert!(unknowns.is_empty());
        assert_eq!(regs.len(), 1);
        assert_eq!(regs[0].fields.len(), 1);
        assert_eq!(regs[0].fields[0].name, "FOO_BAR_BAZ");
        assert_eq!(regs[0].fields[0].shift, 8);
        assert_eq!(regs[0].fields[0].mask, 0x3);
        assert_eq!(regs[0].fields[0].access, "RW");
    }

    #[test]
    fn field_name_ending_in_m_is_not_parsed_as_metadata() {
        let content = r#"
#define FOO_CFG_REG (DR_REG_FOO_BASE + 0x0)
/** FOO_FRAC_M : R/W; bitpos: [9:0]; default: 0; */
#define FOO_FRAC_M 0x3ffU
#define FOO_FRAC_M_M (FOO_FRAC_M_V << FOO_FRAC_M_S)
#define FOO_FRAC_M_V 0x3ffU
#define FOO_FRAC_M_S 0
"#;
        let (regs, _) = read_reg_header_legacy(content).unwrap();
        assert_eq!(regs[0].fields.len(), 1);
        assert_eq!(regs[0].fields[0].name, "FOO_FRAC_M");
        assert_eq!(regs[0].fields[0].mask, 0x3ff);
    }

    #[test]
    fn resolves_numeric_field_aliases() {
        let content = r#"
#define FOO_CFG_REG (DR_REG_FOO_BASE + 0x0)
#define FOO_WIDTH 0x1f
/** FOO_VALUE : R/W; bitpos: [4:0]; default: 0; */
#define FOO_VALUE FOO_WIDTH
#define FOO_VALUE_M (FOO_VALUE_V << FOO_VALUE_S)
#define FOO_VALUE_V FOO_WIDTH
#define FOO_VALUE_S 0
"#;
        let (regs, _) = read_reg_header_legacy(content).unwrap();
        assert_eq!(regs[0].fields[0].mask, 0x1f);
    }

    #[test]
    fn parses_legacy_fields_with_only_shift_metadata() {
        let content = r#"
#define FOO_CFG_REG (DR_REG_FOO_BASE + 0x0)
#define FOO_LOW 0xff
#define FOO_LOW_S 0
#define FOO_ENABLE (BIT(31))
#define FOO_ENABLE_S 31
"#;
        let (regs, _) = read_reg_header_legacy(content).unwrap();
        assert_eq!(regs[0].fields.len(), 2);
        assert_eq!((regs[0].fields[0].shift, regs[0].fields[0].mask), (0, 0xff));
        assert_eq!((regs[0].fields[1].shift, regs[0].fields[1].mask), (31, 1));
    }
}
