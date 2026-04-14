//! Experimental extraction of register metadata from ESP-IDF `components/soc/<chip>/register/soc/*_reg.h`.
//!
//! ESP-IDF headers typically follow one of these shapes:
//! - **Legacy block** (`gpio_reg.h`): `/* FIELD : R/W ;bitpos:[msb:lsb] ;default: ... */` plus `#define FIELD ...` / `_S` lines.
//! - **Doxygen block** (`gdma_reg.h`): `/** NAME : ACCESS; bitpos: [n] or [hi:lo]; ...` with multi-line descriptions.
//!
//! Output is written under `target/` by default so tracked PAC / SVD sources are untouched.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;

use crate::Chip;

const IDF_RAW_BASE: &str =
    "https://raw.githubusercontent.com/espressif/esp-idf/master/components/soc";

/// `*_reg.h` files for ESP32-C3 (from ESP-IDF `register/soc/`), plus `reg_base.h` for address bases.
const ESP32C3_SOC_HEADERS: &[&str] = &[
    "reg_base.h",
    "apb_ctrl_reg.h",
    "apb_saradc_reg.h",
    "assist_debug_reg.h",
    "efuse_reg.h",
    "extmem_reg.h",
    "gdma_reg.h",
    "gpio_reg.h",
    "gpio_sd_reg.h",
    "i2c_reg.h",
    "i2s_reg.h",
    "interrupt_core0_reg.h",
    "io_mux_reg.h",
    "ledc_reg.h",
    "nrx_reg.h",
    "rmt_reg.h",
    "rtc_cntl_reg.h",
    "rtc_i2c_reg.h",
    "sensitive_reg.h",
    "spi_mem_reg.h",
    "spi_reg.h",
    "syscon_reg.h",
    "system_reg.h",
    "systimer_reg.h",
    "timer_group_reg.h",
    "uart_reg.h",
    "uhci_reg.h",
    "usb_serial_jtag_reg.h",
    "xts_aes_reg.h",
];

#[derive(Debug, Clone)]
pub struct IdfSocOptions {
    pub chip: Chip,
    pub out_dir: PathBuf,
    pub idf_path: Option<PathBuf>,
    pub offline: bool,
}

#[derive(Debug, Clone)]
struct ParsedField {
    name: String,
    bit_low: u32,
    bit_high: u32,
    access: String,
    description: String,
}

#[derive(Debug, Clone)]
struct ParsedRegister {
    c_name: String,
    address: u32,
    offset: u32,
    base_macro: String,
    base_addr: u32,
    source_file: String,
    fields: Vec<ParsedField>,
}

pub fn run_idf_soc_experiment(opts: IdfSocOptions) -> Result<()> {
    let chip_dir = chip_soc_dir(&opts.chip).ok_or_else(|| {
        anyhow!(
            "idf-soc experiment is only wired for esp32c3 right now (got {:?})",
            opts.chip
        )
    })?;

    fs::create_dir_all(&opts.out_dir)
        .with_context(|| format!("create {}", opts.out_dir.display()))?;

    let headers: Vec<(String, String)> = if let Some(root) = &opts.idf_path {
        load_headers_from_disk(root, chip_dir)?
    } else if opts.offline {
        bail!("--offline requires --idf-path");
    } else {
        fetch_headers_from_github(chip_dir)?
    };

    let reg_base_content = headers
        .iter()
        .find(|(n, _)| n == "reg_base.h")
        .map(|(_, c)| c.as_str())
        .ok_or_else(|| anyhow!("reg_base.h missing"))?;

    let mut bases = parse_reg_bases(reg_base_content);
    if matches!(opts.chip, Chip::Esp32c3) {
        augment_esp32c3_bases(&mut bases);
    }
    log::info!(
        "parsed {} DR_REG_*_BASE entries (after aliases) from reg_base.h",
        bases.len()
    );

    let mut all_regs = Vec::new();
    for (name, content) in &headers {
        if name == "reg_base.h" || !name.ends_with("_reg.h") {
            continue;
        }
        let mut regs = parse_reg_header(content, name, &bases);
        log::info!("{}: {} registers", name, regs.len());
        all_regs.append(&mut regs);
    }

    all_regs.sort_by(|a, b| a.address.cmp(&b.address).then_with(|| a.c_name.cmp(&b.c_name)));

    write_csv(&opts.out_dir.join("idf_soc_registers.csv"), &all_regs)?;
    write_svd(&opts.out_dir.join("idf_soc_from_headers.svd"), &all_regs)?;

    log::info!(
        "wrote {} registers ({} fields) under {}",
        all_regs.len(),
        all_regs.iter().map(|r| r.fields.len()).sum::<usize>(),
        opts.out_dir.display()
    );

    Ok(())
}

fn chip_soc_dir(chip: &Chip) -> Option<&'static str> {
    match chip {
        Chip::Esp32c3 => Some("esp32c3"),
        _ => None,
    }
}

fn load_headers_from_disk(idf_root: &Path, chip: &str) -> Result<Vec<(String, String)>> {
    let dir = idf_root
        .join("components/soc")
        .join(chip)
        .join("register/soc");
    let mut out = Vec::new();
    for &h in ESP32C3_SOC_HEADERS {
        let p = dir.join(h);
        let s = fs::read_to_string(&p).with_context(|| format!("read {}", p.display()))?;
        out.push((h.to_string(), s));
    }
    Ok(out)
}

fn fetch_headers_from_github(chip: &str) -> Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    for &h in ESP32C3_SOC_HEADERS {
        let url = format!("{IDF_RAW_BASE}/{chip}/register/soc/{h}");
        log::info!("fetch {url}");
        let body = ureq::get(&url)
            .call()
            .with_context(|| format!("GET {url}"))?
            .into_string()
            .with_context(|| format!("decode body {url}"))?;
        out.push((h.to_string(), body));
    }
    Ok(out)
}

fn parse_reg_bases(src: &str) -> HashMap<String, u32> {
    let re = Regex::new(r#"#define\s+(DR_REG_[A-Z0-9_]+_BASE)\s+(0x[0-9a-fA-F]+)\s*"#).unwrap();
    let mut m = HashMap::new();
    for cap in re.captures_iter(src) {
        let name = cap[1].to_string();
        let v = u32::from_str_radix(&cap[2][2..], 16).unwrap_or(0);
        m.insert(name, v);
    }
    m
}

/// IDF sometimes uses different macro names in `*_reg.h` than in `reg_base.h`.
fn augment_esp32c3_bases(bases: &mut HashMap<String, u32>) {
    if let Some(v) = bases.get("DR_REG_INTERRUPT_BASE").copied() {
        bases
            .entry("DR_REG_INTERRUPT_CORE0_BASE".into())
            .or_insert(v);
    }
    // From `components/soc/esp32c3/ld/esp32c3.peripherals.ld` (`SDM`); missing in C3 `reg_base.h`.
    bases
        .entry("DR_REG_GPIO_SD_BASE".into())
        .or_insert(0x6000_4f00);
}

fn parse_reg_header(
    src: &str,
    filename: &str,
    bases: &HashMap<String, u32>,
) -> Vec<ParsedRegister> {
    let mut out = Vec::new();
    let lines: Vec<&str> = src.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let Some((reg_name, expr_inner)) = parse_register_define_line(line) else {
            continue;
        };

        let instances: Vec<(String, String)> = if let Some((param, stem)) = split_indexed_reg_name(&reg_name) {
            (0u32..4)
                .filter_map(|idx| {
                    let inst_name = format!("{stem}({idx})");
                    let sub = replace_ident_param(&expr_inner, &param, idx);
                    Some((inst_name, sub))
                })
                .collect()
        } else {
            vec![(reg_name, expr_inner)]
        };

        for (reg_name, expr_inner) in instances {
            let expanded = match expand_esp32c3_reg_helpers(&expr_inner, bases) {
                Some(e) => e,
                None => continue,
            };
            let wrapped = format!("({expanded})");
            let Some(address) = eval_full_address(&wrapped, bases) else {
                continue;
            };
            if !is_plausible_peripheral_addr(address) {
                continue;
            }

            let (base_macro, base_addr) = match find_dr_reg_base(&wrapped) {
                Some(bm) => {
                    let Some(&ba) = bases.get(&bm) else {
                        log::warn!(
                            "{filename}: unknown base `{bm}` for `{reg_name}` ({wrapped})"
                        );
                        continue;
                    };
                    (bm, ba)
                }
                None => match infer_base_for_address(address, bases) {
                    Some(x) => x,
                    None => {
                        log::warn!("{filename}: could not infer base for `{reg_name}` @ {address:#x}");
                        continue;
                    }
                },
            };

            let offset = address.wrapping_sub(base_addr);

            let block = collect_block_after_define(&lines, i + 1);
            let fields = parse_fields_from_comments(&block);

            out.push(ParsedRegister {
                c_name: reg_name,
                address,
                offset,
                base_macro,
                base_addr,
                source_file: filename.to_string(),
                fields,
            });
        }
    }

    out
}

fn is_plausible_peripheral_addr(a: u32) -> bool {
    (0x6000_0000..0x6010_0000).contains(&a)
}

fn infer_base_for_address(addr: u32, bases: &HashMap<String, u32>) -> Option<(String, u32)> {
    bases
        .iter()
        .filter(|(_, b)| addr >= **b && addr - **b < 0x10_0000)
        .min_by_key(|(_, b)| addr - **b)
        .map(|(n, b)| (n.clone(), *b))
}

fn strip_line_comment(line: &str) -> &str {
    if let Some((before, _)) = line.split_once("//") {
        before
    } else {
        line
    }
}

/// `#define NAME ( ... )` or `#define NAME(i) ( ... )` for peripheral register macros.
fn parse_register_define_line(line: &str) -> Option<(String, String)> {
    let line = strip_line_comment(line).trim();
    let rest = line.strip_prefix("#define")?.trim_start();

    let bytes = rest.as_bytes();
    let mut pos = 0usize;
    while pos < bytes.len()
        && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_' || bytes[pos] == b'(')
    {
        if bytes[pos] == b'(' {
            let mut d = 1u32;
            pos += 1;
            while pos < bytes.len() && d > 0 {
                match bytes[pos] {
                    b'(' => d += 1,
                    b')' => d -= 1,
                    _ => {}
                }
                pos += 1;
            }
            break;
        }
        pos += 1;
    }

    let name = rest.get(..pos)?.trim();
    if !name.contains("_REG") {
        return None;
    }

    while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    let tail = rest.get(pos..)?;
    let (inner, _) = extract_balanced_paren_expr(tail, 0)?;
    Some((name.to_string(), inner))
}

fn extract_balanced_paren_expr(s: &str, start: usize) -> Option<(String, usize)> {
    let b = s.as_bytes().get(start)?;
    if *b != b'(' {
        return None;
    }
    let mut depth = 0u32;
    let mut i = start;
    while i < s.len() {
        match s.as_bytes()[i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    let inner = s.get(start + 1..i)?.to_string();
                    return Some((inner, i + 1));
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn split_indexed_reg_name(name: &str) -> Option<(&str, &str)> {
    let open = name.rfind('(')?;
    let close = name.rfind(')')?;
    if close != name.len().saturating_sub(1) || open >= close {
        return None;
    }
    let stem = name.get(..open)?;
    let inner = name.get(open + 1..close)?;
    if !stem.ends_with("_REG") {
        return None;
    }
    if !inner.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some((inner, stem))
}

fn replace_ident_param(expr: &str, param: &str, value: u32) -> String {
    let re = Regex::new(&format!(r"\b{}\b", regex::escape(param))).unwrap();
    re.replace_all(expr, value.to_string()).to_string()
}

/// Expand `REG_UART_BASE(0)`-style helpers from ESP32-C3 `soc/soc.h`.
fn expand_esp32c3_reg_helpers(expr: &str, bases: &HashMap<String, u32>) -> Option<String> {
    let mut s = expr.to_string();
    // `timer_group_reg.h` uses `DR_REG_TIMG_BASE(i)` as an alias of `REG_TIMG_BASE(i)`.
    let re_timg_alias = Regex::new(r"DR_REG_TIMG_BASE\((\d+)\)").unwrap();
    s = re_timg_alias
        .replace_all(&s, |cap: &regex::Captures| {
            expand_reg_timg_base(&cap[1], bases)
                .map(|v| format!("0x{v:x}"))
                .unwrap_or_else(|| cap[0].to_string())
        })
        .to_string();

    let replacers: &[(&str, fn(&str, &HashMap<String, u32>) -> Option<u32>)] = &[
        ("REG_UART_BASE", expand_reg_uart_base),
        ("REG_I2C_BASE", expand_reg_i2c_base),
        ("REG_TIMG_BASE", expand_reg_timg_base),
        ("REG_UHCI_BASE", expand_reg_uhci_base),
        ("REG_SPI_MEM_BASE", expand_reg_spi_mem_base),
        ("REG_SPI_BASE", expand_reg_spi_base),
        ("REG_UART_AHB_BASE", expand_reg_uart_ahb_base),
        ("REG_I2S_BASE", expand_reg_i2s_base),
    ];

    for _ in 0..64 {
        let prev = s.clone();
        for (prefix, f) in replacers {
            let pat = format!(r"{}\((\d+)\)", regex::escape(prefix));
            let re = Regex::new(&pat).unwrap();
            s = re
                .replace_all(&s, |cap: &regex::Captures| {
                    match f(cap.get(1).map(|m| m.as_str()).unwrap_or("0"), bases) {
                        Some(v) => format!("0x{v:x}"),
                        None => cap[0].to_string(),
                    }
                })
                .to_string();
        }
        if s == prev {
            break;
        }
    }

    if has_unexpanded_soc_reg_macro(&s) {
        return None;
    }
    Some(s)
}

fn has_unexpanded_soc_reg_macro(s: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"\bREG_(?:UART_AHB|UART|I2C|TIMG|UHCI|SPI_MEM|SPI|I2S)_BASE\s*\(|DR_REG_TIMG_BASE\s*\(")
            .unwrap()
    });
    re.is_match(s)
}

fn expand_reg_uart_base(arg: &str, bases: &HashMap<String, u32>) -> Option<u32> {
    let i: u32 = arg.parse().ok()?;
    Some(bases.get("DR_REG_UART_BASE")? + i * 0x1_0000)
}

fn expand_reg_uart_ahb_base(arg: &str, _bases: &HashMap<String, u32>) -> Option<u32> {
    let i: u32 = arg.parse().ok()?;
    Some(0x6000_0000 + i * 0x1_0000)
}

fn expand_reg_i2c_base(arg: &str, bases: &HashMap<String, u32>) -> Option<u32> {
    let i: u32 = arg.parse().ok()?;
    Some(bases.get("DR_REG_I2C_EXT_BASE")? + i * 0x1_4000)
}

fn expand_reg_timg_base(arg: &str, bases: &HashMap<String, u32>) -> Option<u32> {
    let i: u32 = arg.parse().ok()?;
    Some(bases.get("DR_REG_TIMERGROUP0_BASE")? + i * 0x1000)
}

fn expand_reg_uhci_base(arg: &str, bases: &HashMap<String, u32>) -> Option<u32> {
    let i: u32 = arg.parse().ok()?;
    Some(bases.get("DR_REG_UHCI0_BASE")? - i * 0x8000)
}

fn expand_reg_spi_mem_base(arg: &str, bases: &HashMap<String, u32>) -> Option<u32> {
    let i: u32 = arg.parse().ok()?;
    Some(bases.get("DR_REG_SPI0_BASE")? - i * 0x1000)
}

fn expand_reg_spi_base(arg: &str, bases: &HashMap<String, u32>) -> Option<u32> {
    let i: u32 = arg.parse().ok()?;
    if i == 2 {
        Some(*bases.get("DR_REG_SPI2_BASE")?)
    } else {
        Some(0)
    }
}

fn expand_reg_i2s_base(_arg: &str, bases: &HashMap<String, u32>) -> Option<u32> {
    bases.get("DR_REG_I2S_BASE").copied()
}

fn find_dr_reg_base(expr: &str) -> Option<String> {
    let re = Regex::new(r"DR_REG_[A-Z0-9_]+_BASE").unwrap();
    re.find(expr).map(|m| m.as_str().to_string())
}

/// Evaluate `(DR_REG_FOO_BASE + 0x10 + …)` as base + sum of terms (hex or decimal literals only after substitution).
fn eval_full_address(expr: &str, bases: &HashMap<String, u32>) -> Option<u32> {
    let inner = peel_outer_parentheses(expr)?;
    let mut sum: i128 = 0;
    let mut sign = 1i128;
    let mut cur = String::new();

    let flush = |tok: &str, sign: i128, sum: &mut i128, bases: &HashMap<String, u32>| -> Option<()> {
        let t = tok.trim();
        if t.is_empty() {
            return Some(());
        }
        if let Some(v) = bases.get(t) {
            *sum += sign * (*v as i128);
        } else if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
            *sum += sign * i128::from(u32::from_str_radix(hex, 16).ok()?);
        } else if let Ok(v) = t.parse::<u32>() {
            *sum += sign * i128::from(v);
        } else {
            return None;
        }
        Some(())
    };

    for ch in inner.chars() {
        match ch {
            '+' => {
                flush(&cur, sign, &mut sum, bases)?;
                cur.clear();
                sign = 1;
            }
            '-' => {
                flush(&cur, sign, &mut sum, bases)?;
                cur.clear();
                sign = -1;
            }
            _ => cur.push(ch),
        }
    }
    flush(&cur, sign, &mut sum, bases)?;

    u32::try_from(sum).ok()
}

fn peel_outer_parentheses(s: &str) -> Option<&str> {
    let mut t = s.trim();
    while t.starts_with('(') && t.ends_with(')') && outer_parens_balanced(t) {
        t = t.get(1..t.len().saturating_sub(1))?;
    }
    Some(t)
}

fn outer_parens_balanced(s: &str) -> bool {
    let mut depth = 0u32;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    return false;
                }
                depth -= 1;
                if depth == 0 && i != s.len().saturating_sub(1) {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}

fn collect_block_after_define(lines: &[&str], start: usize) -> String {
    let mut buf = String::new();
    for line in lines.iter().skip(start) {
        if parse_register_define_line(line).is_some() {
            break;
        }
        buf.push_str(line);
        buf.push('\n');
    }
    buf
}

fn parse_fields_from_comments(block: &str) -> Vec<ParsedField> {
    let field_line = field_metadata_regex();
    let mut out = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = block.chars().collect();

    while i < chars.len() {
        if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
            i += 2;
            let mut depth = 1usize;
            let mut j = i;
            while j < chars.len() && depth > 0 {
                if j + 1 < chars.len() && chars[j] == '/' && chars[j + 1] == '*' {
                    depth += 1;
                    j += 2;
                    continue;
                }
                if j + 1 < chars.len() && chars[j] == '*' && chars[j + 1] == '/' {
                    depth -= 1;
                    if depth == 0 {
                        j += 2;
                        break;
                    }
                    j += 2;
                    continue;
                }
                j += 1;
            }
            if depth != 0 {
                break;
            }
            let end_content = j.saturating_sub(2).max(i).min(chars.len());
            let inner: String = chars[i..end_content].iter().collect();
            i = j;

            let normalized = normalize_comment_inner(&inner);
            let first_line = normalized.lines().next().unwrap_or("").trim();

            if let Some(cap) = field_line.captures(first_line) {
                let name = cap["name"].to_string();
                let access = cap["acc"].trim().to_string();
                let msb: u32 = cap["msb"].parse().unwrap_or(0);
                let lsb: u32 = cap
                    .name("lsb")
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(msb);
                let (bit_low, bit_high) = if lsb <= msb { (lsb, msb) } else { (msb, lsb) };

                let desc = normalized
                    .lines()
                    .skip(1)
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .take(6)
                    .collect::<Vec<_>>()
                    .join(" ");

                out.push(ParsedField {
                    name,
                    bit_low,
                    bit_high,
                    access,
                    description: desc,
                });
            }
            continue;
        }
        i += 1;
    }

    out
}

fn normalize_comment_inner(inner: &str) -> String {
    inner
        .lines()
        .map(|l| {
            let t = l.trim();
            t.strip_prefix("/**")
                .or_else(|| t.strip_prefix("/*!"))
                .or_else(|| t.strip_prefix("/*"))
                .unwrap_or(t)
                .trim_start_matches('*')
                .trim()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn field_metadata_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?x)
            (?P<name>[A-Za-z0-9_]+)
            \s*:\s*
            (?P<acc>[^;]+?)
            ;
            \s*bitpos:\s*
            \[\s*(?P<msb>\d+)
            (?:\s*:\s*(?P<lsb>\d+)\s*)?
            \]
            ",
        )
        .expect("valid regex")
    })
}

fn write_csv(path: &Path, regs: &[ParsedRegister]) -> Result<()> {
    let mut w = String::from("source_file,base_macro,base_addr_hex,register_cname,address_hex,offset_hex,field_name,bit_low,bit_high,access,description\n");
    for r in regs {
        if r.fields.is_empty() {
            w.push_str(&format!(
                "{},{},{:#x},{},{:#x},{:#x},,,,,\n",
                r.source_file,
                r.base_macro,
                r.base_addr,
                r.c_name,
                r.address,
                r.offset
            ));
        } else {
            for f in &r.fields {
                w.push_str(&format!(
                    "{},{},{:#x},{},{:#x},{:#x},{},{},{},\"{}\",\"{}\"\n",
                    r.source_file,
                    r.base_macro,
                    r.base_addr,
                    r.c_name,
                    r.address,
                    r.offset,
                    f.name,
                    f.bit_low,
                    f.bit_high,
                    f.access.replace('\"', "'"),
                    f.description.replace('\"', "'")
                ));
            }
        }
    }
    fs::write(path, w).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

fn write_svd(path: &Path, regs: &[ParsedRegister]) -> Result<()> {
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct PeripheralBuild {
        base_addr: u32,
        base_macro: String,
        regs: Vec<ParsedRegister>,
    }

    let mut groups: BTreeMap<u32, PeripheralBuild> = BTreeMap::new();
    for r in regs {
        let e = groups.entry(r.base_addr).or_default();
        if e.base_macro.is_empty() {
            e.base_macro = r.base_macro.clone();
        }
        e.base_addr = r.base_addr;
        e.regs.push(r.clone());
    }

    let mut xml = String::new();
    xml.push_str(r#"<?xml version="1.0" encoding="utf-8"?>"#);
    xml.push('\n');
    xml.push_str(
        r#"<device schemaVersion="1.1" xmlns:xs="http://www.w3.org/2001/XMLSchema-instance" xs:noNamespaceSchemaLocation="CMSIS-SVD_Schema_1_1.xsd">"#,
    );
    xml.push('\n');
    xml.push_str("  <name>ESP32C3_IDF_SOC_HEADERS</name>\n");
    xml.push_str("  <version>0.0</version>\n");
    xml.push_str("  <description>Experimental: ESP-IDF soc *_reg.h extract (not a complete SoC description).</description>\n");
    xml.push_str("  <addressUnitBits>8</addressUnitBits>\n");
    xml.push_str("  <width>32</width>\n");
    xml.push_str("  <peripherals>\n");

    for (_base, p) in groups {
        let pname = peripheral_svd_name(&p.base_macro);
        xml.push_str("    <peripheral>\n");
        xml.push_str(&format!("      <name>{pname}</name>\n"));
        xml.push_str(&format!(
            "      <description>{}</description>\n",
            xml_escape(&p.base_macro)
        ));
        xml.push_str(&format!("      <baseAddress>{:#x}</baseAddress>\n", p.base_addr));
        xml.push_str("      <registers>\n");

        let mut rlist = p.regs;
        rlist.sort_by(|a, b| a.offset.cmp(&b.offset).then_with(|| a.c_name.cmp(&b.c_name)));

        for r in rlist {
            let rname = register_svd_name(&pname, &r.c_name);
            xml.push_str("        <register>\n");
            xml.push_str(&format!("          <name>{rname}</name>\n"));
            xml.push_str(&format!(
                "          <description>{}</description>\n",
                xml_escape(&r.c_name)
            ));
            xml.push_str("          <addressOffset>");
            xml.push_str(&format!("{:#x}", r.offset));
            xml.push_str("</addressOffset>\n");
            xml.push_str("          <size>32</size>\n");
            xml.push_str("          <access>read-write</access>\n");
            if !r.fields.is_empty() {
                xml.push_str("          <fields>\n");
                for f in &r.fields {
                    let width = f.bit_high - f.bit_low + 1;
                    let acc = map_access(&f.access);
                    xml.push_str("            <field>\n");
                    xml.push_str(&format!("              <name>{}</name>\n", xml_escape(&f.name)));
                    xml.push_str(&format!(
                        "              <description>{}</description>\n",
                        xml_escape(&f.description)
                    ));
                    xml.push_str(&format!("              <bitOffset>{}</bitOffset>\n", f.bit_low));
                    xml.push_str(&format!("              <bitWidth>{width}</bitWidth>\n"));
                    xml.push_str(&format!("              <access>{acc}</access>\n"));
                    xml.push_str("            </field>\n");
                }
                xml.push_str("          </fields>\n");
            }
            xml.push_str("        </register>\n");
        }

        xml.push_str("      </registers>\n");
        xml.push_str("    </peripheral>\n");
    }

    xml.push_str("  </peripherals>\n");
    xml.push_str("</device>\n");

    fs::write(path, xml).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

fn peripheral_svd_name(base_macro: &str) -> String {
    let s = base_macro
        .strip_prefix("DR_REG_")
        .and_then(|s| s.strip_suffix("_BASE"))
        .unwrap_or(base_macro);
    sanitize_ident(s)
}

fn register_svd_name(peripheral: &str, c_reg: &str) -> String {
    let prefix = format!("{peripheral}_");
    let body = c_reg
        .strip_suffix("_REG")
        .unwrap_or(c_reg)
        .strip_prefix(&prefix)
        .unwrap_or(
            c_reg
                .strip_suffix("_REG")
                .unwrap_or(c_reg),
        );
    sanitize_ident(body)
}

fn sanitize_ident(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        out.insert(0, 'R');
    }
    if out.is_empty() {
        out.push('X');
    }
    out
}

fn map_access(s: &str) -> &'static str {
    let head = s
        .split(';')
        .next()
        .unwrap_or(s)
        .trim()
        .to_ascii_uppercase();
    if head.contains("R/W") || head == "RW" {
        return "read-write";
    }
    if head.contains("WO") {
        return "write-only";
    }
    if head.contains("RO") {
        return "read-only";
    }
    "read-write"
}

fn xml_escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpio_header_excerpt() {
        let bases_src = r#"
#define DR_REG_GPIO_BASE 0x60004000
"#;
        let bases = parse_reg_bases(bases_src);
        let hdr = r#"
#define GPIO_OUT_REG (DR_REG_GPIO_BASE + 0x4)
/* GPIO_OUT_DATA : R/W ;bitpos:[25:0] ;default: 26'h0 ; */
/*description: */
#define GPIO_OUT_DATA 0x03FFFFFF
#define GPIO_OUT_W1TS_REG (DR_REG_GPIO_BASE + 0x8)
"#;
        let regs = parse_reg_header(hdr, "gpio_reg.h", &bases);
        assert_eq!(regs.len(), 2);
        assert_eq!(regs[0].c_name, "GPIO_OUT_REG");
        assert_eq!(regs[0].address, 0x6000_4004);
        assert_eq!(regs[0].fields.len(), 1);
        assert_eq!(regs[0].fields[0].name, "GPIO_OUT_DATA");
        assert_eq!(regs[0].fields[0].bit_low, 0);
        assert_eq!(regs[0].fields[0].bit_high, 25);
    }

    #[test]
    fn gdma_doxygen_field() {
        let bases_src = r#"
#define DR_REG_GDMA_BASE 0x6003f000
"#;
        let bases = parse_reg_bases(bases_src);
        let hdr = r#"
#define GDMA_INT_RAW_CH0_REG (DR_REG_GDMA_BASE + 0x0)
/** GDMA_IN_DONE_CH0_INT_RAW : R/WTC/SS; bitpos: [0]; default: 0;
 * hello
 */
#define GDMA_IN_DONE_CH0_INT_RAW (BIT(0))
"#;
        let regs = parse_reg_header(hdr, "gdma_reg.h", &bases);
        assert_eq!(regs.len(), 1);
        assert_eq!(regs[0].fields.len(), 1);
        assert_eq!(regs[0].fields[0].name, "GDMA_IN_DONE_CH0_INT_RAW");
    }
}