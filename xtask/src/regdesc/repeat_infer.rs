use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

use super::model::{Field, Register};

static CH_INDEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<prefix>.*?)CH(?P<index>\d+)(?P<suffix>.*)$").unwrap());
static PIN_INDEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<prefix>.*?)PIN(?P<index>\d+)(?P<suffix>.*)$").unwrap());
static FUNC_INDEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<prefix>.*?)FUNC(?P<index>\d+)_(?P<suffix>.+)$").unwrap()
});
static TIMER_INDEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<prefix>.*?)TIMER(?P<index>\d+)_(?P<suffix>.+)$").unwrap()
});
static UNIT_INDEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<prefix>.*?)U(?P<index>\d+)_(?P<suffix>.+)$").unwrap());
static DATA_INDEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<prefix>.*?)DATA(?P<index>\d+)$").unwrap());
static CHECK_VALUE_INDEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<prefix>.*?)CHECK_VALUE(?P<index>\d+)$").unwrap()
});
static NAMED_UNDERSCORE_INDEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<prefix>.*(?:KEY|TEXT_IN|TEXT_OUT))_(?P<index>\d+)$").unwrap()
});
static GPIO_INDEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<prefix>.*?)GPIO(?P<index>\d+)$").unwrap());
static SIGMADELTA_INDEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<prefix>.*?)SIGMADELTA(?P<index>\d+)$").unwrap());
static PMS_INDEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<prefix>.*PMS)(?P<index>\d+)(?P<suffix>_.*)$").unwrap()
});
static ETM_TASK_P_INDEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<prefix>ETM_TASK_)P(?P<index>\d+)_CFG$").unwrap()
});
static W_INDEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<prefix>.*)W(?P<index>\d+)$").unwrap());

/// Detects numbered register/field names and sets merge hints before `merge_*` runs.
pub fn infer_repeats(registers: &mut [Register]) {
    infer_register_repeats(registers);
    for register in registers.iter_mut() {
        infer_field_repeats(&mut register.fields);
    }
}

/// Groups registers whose names differ only by a numeric index.
fn infer_register_repeats(registers: &mut [Register]) {
    let mut groups: HashMap<String, Vec<(usize, i32)>> = HashMap::new();

    for (idx, register) in registers.iter().enumerate() {
        if register.repeat.is_some() {
            continue;
        }
        let Some((template, index)) = template_and_index(&register.name) else {
            continue;
        };
        groups.entry(template).or_default().push((idx, index));
    }

    for (template, mut items) in groups {
        items.sort_by_key(|(_, index)| *index);
        for run in contiguous_runs(&items) {
            if run.len() < 2 {
                continue;
            }

            let stride = match register_stride(&run, registers) {
                Some(stride) => stride,
                None => continue,
            };
            if stride == 0 {
                continue;
            }

            let repeat_name = format!("{template}_REG");
            for (idx, index) in run {
                registers[idx].set_repeat_hint(repeat_name.clone(), index);
                let _ = stride;
            }
        }
    }
}

/// Groups fields whose names differ only by a numeric index.
fn infer_field_repeats(fields: &mut [Field]) {
    let mut groups: HashMap<String, Vec<(usize, i32)>> = HashMap::new();

    for (idx, field) in fields.iter().enumerate() {
        if field.repeat.is_some() {
            continue;
        }
        let Some((template, index)) = template_and_index(&field.name) else {
            continue;
        };
        groups.entry(template).or_default().push((idx, index));
    }

    for (template, mut items) in groups {
        items.sort_by_key(|(_, index)| *index);
        for run in contiguous_runs(&items) {
            if run.len() < 2 {
                continue;
            }

            let stride = match field_stride(&run, fields) {
                Some(stride) => stride,
                None => continue,
            };
            if stride == 0 {
                continue;
            }

            for (idx, index) in run {
                fields[idx].set_repeat_hint(template.clone(), index);
                let _ = stride;
            }
        }
    }
}

/// Splits a symbol into `(template with $n, index)` using common IDF naming patterns.
fn template_and_index(name: &str) -> Option<(String, i32)> {
    let stem = name.strip_suffix("_REG").unwrap_or(name);

    if let Some(caps) = CH_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}CH$n{}", &caps["prefix"], &caps["suffix"]), index));
    }
    if let Some(caps) = PIN_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}PIN$n{}", &caps["prefix"], &caps["suffix"]), index));
    }
    if let Some(caps) = FUNC_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((
            format!(
                "{}FUNC$n_{}",
                &caps["prefix"],
                &caps["suffix"]
            ),
            index,
        ));
    }
    if let Some(caps) = TIMER_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((
            format!(
                "{}TIMER$n_{}",
                &caps["prefix"],
                &caps["suffix"]
            ),
            index,
        ));
    }
    if let Some(caps) = UNIT_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((
            format!("{}U$n_{}", &caps["prefix"], &caps["suffix"]),
            index,
        ));
    }
    if let Some(caps) = DATA_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}DATA$n", &caps["prefix"]), index));
    }
    if let Some(caps) = NAMED_UNDERSCORE_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}_$n", &caps["prefix"]), index));
    }
    if let Some(caps) = CHECK_VALUE_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}CHECK_VALUE$n", &caps["prefix"]), index));
    }
    if let Some(caps) = GPIO_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}GPIO$n", &caps["prefix"]), index));
    }
    if let Some(caps) = SIGMADELTA_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}SIGMADELTA$n", &caps["prefix"]), index));
    }
    if let Some(caps) = PMS_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}$n{}", &caps["prefix"], &caps["suffix"]), index));
    }
    if let Some(caps) = ETM_TASK_P_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}P$n_CFG", &caps["prefix"]), index));
    }
    if let Some(caps) = W_INDEX.captures(stem) {
        let index: i32 = caps["index"].parse().ok()?;
        return Some((format!("{}W$n", &caps["prefix"]), index));
    }

    None
}

/// Splits `(index, value)` items into maximal runs of consecutive indices.
fn contiguous_runs(items: &[(usize, i32)]) -> Vec<Vec<(usize, i32)>> {
    if items.is_empty() {
        return Vec::new();
    }

    let mut runs = Vec::new();
    let mut current = vec![items[0]];

    for &(idx, index) in &items[1..] {
        let (_, prev_index) = *current.last().unwrap();
        if index == prev_index + 1 {
            current.push((idx, index));
        } else {
            runs.push(current);
            current = vec![(idx, index)];
        }
    }

    runs.push(current);
    runs
}

/// Returns address spacing between repeated registers when it is uniform.
fn register_stride(items: &[(usize, i32)], registers: &[Register]) -> Option<u32> {
    if items.len() <= 1 {
        return Some(0);
    }
    let base = registers[items[0].0].addr;
    let next = registers[items[1].0].addr;
    let stride = next.checked_sub(base)?;
    if stride == 0 {
        return None;
    }
    if items.iter().enumerate().all(|(idx, (reg_idx, _))| {
        registers[*reg_idx]
            .addr
            .checked_sub(base)
            .is_some_and(|delta| delta == idx as u32 * stride)
    }) {
        Some(stride)
    } else {
        None
    }
}

/// Returns bit spacing between repeated fields when it is uniform.
fn field_stride(items: &[(usize, i32)], fields: &[Field]) -> Option<u32> {
    if items.len() <= 1 {
        return Some(0);
    }
    let base = fields[items[0].0].shift;
    let next = fields[items[1].0].shift;
    let stride = next.checked_sub(base)?;
    if stride == 0 {
        return None;
    }
    if items.iter().enumerate().all(|(idx, (field_idx, _))| {
        fields[*field_idx]
            .shift
            .checked_sub(base)
            .is_some_and(|delta| delta == idx as u32 * stride)
    }) {
        Some(stride)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::model::Register;

    #[test]
    fn infers_gpio_pin_repeats() {
        let mut registers = vec![
            Register {
                name: "GPIO_PIN0_REG".into(),
                addr: 0xf4,
                ..default_register()
            },
            Register {
                name: "GPIO_PIN1_REG".into(),
                addr: 0xf8,
                ..default_register()
            },
        ];
        infer_register_repeats(&mut registers);
        assert_eq!(registers[0].repeat_name_hint.as_deref(), Some("GPIO_PIN$n_REG"));
        assert_eq!(registers[0].repeat_index_hint, Some(0));
        assert_eq!(registers[1].repeat_index_hint, Some(1));
    }

    #[test]
    fn infers_gpio_func_repeats() {
        let mut registers = vec![
            Register {
                name: "GPIO_FUNC0_OUT_SEL_CFG_REG".into(),
                addr: 0x500,
                ..default_register()
            },
            Register {
                name: "GPIO_FUNC1_OUT_SEL_CFG_REG".into(),
                addr: 0x504,
                ..default_register()
            },
        ];
        infer_register_repeats(&mut registers);
        assert_eq!(
            registers[0].repeat_name_hint.as_deref(),
            Some("GPIO_FUNC$n_OUT_SEL_CFG_REG")
        );
    }

    #[test]
    fn does_not_merge_gpio_out_banks() {
        let mut registers = vec![
            Register {
                name: "GPIO_OUT1_REG".into(),
                addr: 0x10,
                ..default_register()
            },
            Register {
                name: "GPIO_OUT2_REG".into(),
                addr: 0x14,
                ..default_register()
            },
        ];
        infer_register_repeats(&mut registers);
        assert!(registers[0].repeat_name_hint.is_none());
        assert!(registers[1].repeat_name_hint.is_none());
    }

    #[test]
    fn preserves_prefix_for_w_style_arrays() {
        assert_eq!(
            template_and_index("SPI_MEM_W7_REG"),
            Some(("SPI_MEM_W$n".into(), 7))
        );
        assert_eq!(
            template_and_index("PVT_PMUP_BITMAP_LOW0_REG"),
            Some(("PVT_PMUP_BITMAP_LOW$n".into(), 0))
        );
    }

    #[test]
    fn infers_unit_style_arrays() {
        assert_eq!(
            template_and_index("PCNT_U3_STATUS_REG"),
            Some(("PCNT_U$n_STATUS".into(), 3))
        );
    }

    #[test]
    fn infers_permission_monitor_arrays() {
        assert_eq!(
            template_and_index("SPI_MEM_SPI_FMEM_PMS3_ATTR_REG"),
            Some(("SPI_MEM_SPI_FMEM_PMS$n_ATTR".into(), 3))
        );
    }

    #[test]
    fn infers_contiguous_func_in_runs() {
        let mut registers = vec![
            Register {
                name: "GPIO_FUNC0_IN_SEL_CFG_REG".into(),
                addr: 0x2f4,
                ..default_register()
            },
            Register {
                name: "GPIO_FUNC1_IN_SEL_CFG_REG".into(),
                addr: 0x2f8,
                ..default_register()
            },
            Register {
                name: "GPIO_FUNC10_IN_SEL_CFG_REG".into(),
                addr: 0x31c,
                ..default_register()
            },
            Register {
                name: "GPIO_FUNC11_IN_SEL_CFG_REG".into(),
                addr: 0x320,
                ..default_register()
            },
        ];
        infer_register_repeats(&mut registers);
        assert_eq!(
            registers[0].repeat_name_hint.as_deref(),
            Some("GPIO_FUNC$n_IN_SEL_CFG_REG")
        );
        assert_eq!(registers[2].repeat_index_hint, Some(10));
        assert_eq!(registers[2].repeat_name_hint.as_deref(), Some("GPIO_FUNC$n_IN_SEL_CFG_REG"));
    }

    #[test]
    fn merges_contiguous_func_in_runs() {
        use super::super::merge::merge_registers;

        let registers = (0..8)
            .map(|i| Register {
                name: format!("GPIO_FUNC{i}_IN_SEL_CFG_REG"),
                addr: 0x2f4 + i * 4,
                repeat_name_hint: Some("GPIO_FUNC$n_IN_SEL_CFG_REG".into()),
                repeat_index_hint: Some(i as i32),
                ..default_register()
            })
            .collect::<Vec<_>>();
        let (merged, errors) = merge_registers(registers);
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].repeat.as_ref().map(|r| r.count), Some(8));
    }

    #[test]
    fn gpio_header_merges_func_in_registers() {
        use std::path::Path;

        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../esp-idf/components/soc/esp32s31/register/soc/gpio_reg.h"
        ));
        if !path.is_file() {
            return;
        }

        let content = std::fs::read_to_string(path).unwrap();
        let (mut peripheral, _) =
            super::super::idf::read_peripheral_header(&content, Some("GPIO")).unwrap();
        infer_repeats(&mut peripheral.register_groups[0].registers);
        let hinted = peripheral.register_groups[0]
            .registers
            .iter()
            .filter(|register| {
                register.name.contains("IN_SEL_CFG") && register.repeat_name_hint.is_some()
            })
            .count();
        assert!(hinted >= 200, "expected FUNC IN repeat hints, got {hinted}");

        let errors = peripheral.merge_registers_fields();
        assert!(
            errors.is_empty(),
            "merge errors: {errors:?}; merged_count={}; hinted={hinted}",
            peripheral.register_count()
        );
        assert!(
            peripheral.register_count() < 100,
            "expected merged GPIO register count, got {} (hinted={hinted})",
            peripheral.register_count()
        );
    }

    fn default_register() -> Register {
        Register {
            name: String::new(),
            addr: 0,
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
