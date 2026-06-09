//! Pure-Rust parser for ESP-IDF C register headers → CMSIS-SVD XML.
//!
//! # Supported C-header formats
//!
//! **Legacy** (ESP32, ESP32-C3, ESP32-C2, ESP32-S2, ESP32-S3):
//! ```c
//! #define UART_CLKDIV_REG  (DR_REG_UART_BASE + 0x14)
//! /* UART_CLKDIV : R/W ;bitpos:[19:0] ;default: 20'h1b0 ; */
//! /*description: UART clock divider register */
//! #define UART_CLKDIV  0x000FFFFF
//! #define UART_CLKDIV_M  ((UART_CLKDIV_V)<<(UART_CLKDIV_S))
//! #define UART_CLKDIV_V  0xFFFFF
//! #define UART_CLKDIV_S  0
//! ```
//!
//! **New** (ESP32-C6, ESP32-H2, ESP32-P4, ESP32-C5):
//! ```c
//! /** I2C_SCL_LOW_PERIOD_REG register
//!  *  Configures the SCL low level width.
//!  */
//! #define I2C_SCL_LOW_PERIOD_REG(i) (REG_I2C_BASE(i) + 0x0)
//! /** I2C_SCL_LOW_PERIOD : R/W; bitpos: [8:0]; default: 0;
//!  *  Description of the field.
//!  */
//! #define I2C_SCL_LOW_PERIOD    0x000001FFU
//! #define I2C_SCL_LOW_PERIOD_M  (I2C_SCL_LOW_PERIOD_V << I2C_SCL_LOW_PERIOD_S)
//! #define I2C_SCL_LOW_PERIOD_V  0x000001FFU
//! #define I2C_SCL_LOW_PERIOD_S  0
//! ```

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Data structures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum AccessType {
    ReadWrite,
    ReadOnly,
    WriteOnly,
}

impl AccessType {
    pub fn as_svd_str(&self) -> &'static str {
        match self {
            AccessType::ReadWrite => "read-write",
            AccessType::ReadOnly => "read-only",
            AccessType::WriteOnly => "write-only",
        }
    }

    /// Parse from C header access string (both legacy and new format).
    fn from_str(s: &str) -> Self {
        match s.trim() {
            "R/W" | "R/W/SS" | "R/W/SC" | "R/W/WTC" | "R/W/SC/WTC" => AccessType::ReadWrite,
            "RO" | "R" | "RC" | "RC_W0" | "RC_W1" | "HRO" => AccessType::ReadOnly,
            "WO" | "W" | "WT" | "WTC" | "WTS" => AccessType::WriteOnly,
            _ => AccessType::ReadWrite,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedField {
    pub name: String,
    pub description: String,
    pub bit_offset: u32,
    pub bit_width: u32,
    pub access: AccessType,
}

#[derive(Debug, Clone)]
pub struct ParsedRegister {
    pub name: String,
    pub description: String,
    pub offset: u32,
    pub size: u32,
    pub fields: Vec<ParsedField>,
    /// SVD dim array size (number of elements). None means not an array.
    pub dim: Option<u32>,
    /// SVD dimIncrement in bytes between array elements.
    pub dim_increment: Option<u32>,
    /// SVD dimIndex (e.g. "0-21" or "0,1,2"). None means auto 0..dim-1.
    pub dim_index: Option<String>,
}

// ---------------------------------------------------------------------------
// YAML structures for config files
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct PeripheralsYml {
    pub peripheral_instances: Vec<PeripheralInstance>,
}

#[derive(Debug, Deserialize)]
pub struct PeripheralInstance {
    pub name: String,
    pub peripheral: String,
    pub base_addr: serde_yaml::Value,
}

impl PeripheralInstance {
    pub fn base_address(&self) -> u64 {
        parse_yaml_int(&self.base_addr)
    }
}

#[derive(Debug, Deserialize)]
pub struct InterruptsYml {
    pub peripheral_interrupts: Vec<PeripheralInterrupt>,
}

#[derive(Debug, Deserialize)]
pub struct PeripheralInterrupt {
    pub name: String,
    pub instance: String,
    pub value: u32,
}

#[derive(Debug, Deserialize)]
pub struct DescriptionsYml {
    pub peripheral_descriptions: Vec<PeripheralDescription>,
}

#[derive(Debug, Deserialize)]
pub struct PeripheralDescription {
    pub peripheral: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct HeaderMapYml {
    pub peripherals: HashMap<String, HeaderEntry>,
}

#[derive(Debug, Deserialize, Default)]
pub struct HeaderEntry {
    /// IDF register header file (relative to <idf>/components/soc/<chip>/register/soc/
    /// or <idf>/components/soc/<chip>/include/soc/)
    pub file: Option<String>,
    /// C macro prefix to strip from register/field names.
    /// If absent, derived from the peripheral name in the map key.
    pub prefix: Option<String>,
    /// Additional longer prefixes to try FIRST (longest-match wins).
    /// Useful for headers where some registers have a longer vendor prefix,
    /// e.g. APB_SARADC_APB_ADC_* vs APB_SARADC_*.
    #[serde(default)]
    pub alt_prefixes: Vec<String>,
    /// Explicit register renames applied after prefix stripping.
    /// Key = stripped register name, value = desired SVD register name.
    #[serde(default)]
    pub renames: HashMap<String, String>,
    /// If true, fall back to the yml/ directory instead of parsing from headers.
    #[serde(default)]
    pub skip: bool,
    /// If set, use this yml file in yml/ for fallback (default: <key_lowercase>.yml)
    pub yml: Option<String>,
}

// ---------------------------------------------------------------------------
// Fallback YAML (idf-pipeline/yml/*.yml)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct FallbackYml {
    peripherals: Vec<FallbackPeripheral>,
}

#[derive(Debug, Deserialize)]
struct FallbackPeripheral {
    name: String,
    register_groups: Vec<FallbackRegisterGroup>,
}

#[derive(Debug, Deserialize)]
struct FallbackRegisterGroup {
    #[serde(default)]
    description: String,
    registers: Vec<FallbackRegister>,
}

#[derive(Debug, Deserialize)]
struct FallbackRegister {
    name: String,
    #[serde(default)]
    description: String,
    addr: serde_yaml::Value,
    #[serde(default = "default_reg_size")]
    size: u32,
    #[serde(default)]
    dim: Option<u32>,
    #[serde(default)]
    dim_increment: Option<u32>,
    #[serde(default)]
    dim_index: Option<String>,
    #[serde(default)]
    fields: Vec<FallbackField>,
}

fn default_reg_size() -> u32 {
    32
}

#[derive(Debug, Deserialize)]
struct FallbackField {
    name: String,
    #[serde(default)]
    description: String,
    shift: u32,
    mask: serde_yaml::Value,
    #[serde(default)]
    access: String,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_yaml_int(v: &serde_yaml::Value) -> u64 {
    match v {
        serde_yaml::Value::Number(n) => n.as_u64().unwrap_or(0),
        serde_yaml::Value::String(s) => parse_hex_or_dec(s).unwrap_or(0) as u64,
        _ => 0,
    }
}

fn parse_hex_or_dec(s: &str) -> Option<u32> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        s.parse().ok()
    }
}

/// Count the number of set bits in a mask to determine bit width.
fn mask_to_width(mask: u64) -> u32 {
    mask.count_ones()
}

/// Find the lowest set bit position in a mask.
fn mask_to_offset(mask: u64) -> u32 {
    if mask == 0 {
        return 0;
    }
    mask.trailing_zeros()
}

// ---------------------------------------------------------------------------
// Header location resolution
// ---------------------------------------------------------------------------

/// Try to find an IDF register header file, checking both the new `register/soc/`
/// path (newer chips) and the legacy `include/soc/` path.
fn find_reg_header(idf_path: &Path, chip: &str, filename: &str) -> Option<PathBuf> {
    let candidates = [
        idf_path
            .join("components")
            .join("soc")
            .join(chip)
            .join("register")
            .join("soc")
            .join(filename),
        idf_path
            .join("components")
            .join("soc")
            .join(chip)
            .join("include")
            .join("soc")
            .join(filename),
    ];
    candidates.into_iter().find(|p| p.is_file())
}

// ---------------------------------------------------------------------------
// C-header format detection
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
enum HeaderFormat {
    /// Older style: `/* FIELD : ACCESS ;bitpos:[N:M] ; */`
    Legacy,
    /// Newer style: `/** FIELD : ACCESS; bitpos: [N:M]; */`
    New,
}

fn detect_format(content: &str) -> HeaderFormat {
    // The new format uses `/** ` (doc-comment style) for register/field docs.
    if content.contains("/** ") || content.contains("/**\n") {
        HeaderFormat::New
    } else {
        HeaderFormat::Legacy
    }
}

// ---------------------------------------------------------------------------
// Register/field name stripping
// ---------------------------------------------------------------------------

/// Strip the C-macro peripheral prefix (e.g. `GPIO_`) from a macro name.
/// Also strips `_REG` suffix if `strip_reg` is true.
fn strip_prefix(name: &str, prefix: &str, strip_reg: bool) -> String {
    strip_prefix_multi(name, &[prefix], strip_reg)
}

/// Like `strip_prefix` but tries multiple prefixes in order (longest first).
/// The first matching prefix is used; if none match the name is returned as-is.
fn strip_prefix_multi(name: &str, prefixes: &[&str], strip_reg: bool) -> String {
    let name_upper = name.to_uppercase();
    // Try each candidate prefix (caller should order longest-first)
    let stripped = prefixes
        .iter()
        .find_map(|prefix| {
            let upper_prefix = format!("{}_", prefix.to_uppercase());
            if name_upper.starts_with(&upper_prefix) {
                Some(&name[upper_prefix.len()..])
            } else {
                None
            }
        })
        .unwrap_or(name);

    if strip_reg {
        stripped
            .trim_end_matches("_REG")
            .to_string()
    } else {
        stripped.to_string()
    }
}

// ---------------------------------------------------------------------------
// Bitpos parser
// ---------------------------------------------------------------------------

/// Parse `[MSB:LSB]` or `[N]` → `(bit_offset, bit_width)`.
fn parse_bitpos(s: &str) -> Option<(u32, u32)> {
    let s = s.trim();
    // Find '[' ... ']'
    let start = s.find('[')?;
    let end = s.find(']')?;
    if end <= start {
        return None;
    }
    let inner = &s[start + 1..end];

    if let Some(colon) = inner.find(':') {
        let msb: u32 = inner[..colon].trim().parse().ok()?;
        let lsb: u32 = inner[colon + 1..].trim().parse().ok()?;
        // Ensure MSB >= LSB
        let (hi, lo) = if msb >= lsb { (msb, lsb) } else { (lsb, msb) };
        Some((lo, hi - lo + 1))
    } else {
        let n: u32 = inner.trim().parse().ok()?;
        Some((n, 1))
    }
}

// ---------------------------------------------------------------------------
// Legacy format parser
// ---------------------------------------------------------------------------

/// Try to parse a register address define (non-parameterized or parameterized).
/// Pattern: `#define NAME_REG[optional_params]  (BASE_EXPR + OFFSET)`
/// Returns `(register_name_without_REG_suffix, offset)` or None.
fn try_parse_reg_define_line(line: &str) -> Option<(String, u32)> {
    let trimmed = line.trim();
    if !trimmed.starts_with("#define ") {
        return None;
    }
    // Must contain '+' to distinguish from simple value defines
    if !trimmed.contains('+') {
        return None;
    }

    let rest = trimmed[8..].trim_start();

    // Extract the macro name (identifier chars only)
    let name_end = rest
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(rest.len());
    let name = &rest[..name_end];

    // Accept names ending with `_REG` (standard registers) or `_MEM` (memory regions).
    if !name.ends_with("_REG") && !name.ends_with("_MEM") {
        return None;
    }

    // Extract offset: rightmost `+ HEXVAL` in the line
    let offset = extract_offset_from_expr(&rest[name_end..])?;

    Some((name.to_string(), offset))
}

/// Extract the numeric offset from an expression like `(BASE + 0x4)` or
/// `(REG_BASE(i) + 0x004)`.
fn extract_offset_from_expr(s: &str) -> Option<u32> {
    // Find the LAST '+' character
    let plus_pos = s.rfind('+')?;
    let after = &s[plus_pos + 1..];
    // Clean up trailing characters
    let clean = after.trim().trim_end_matches(')').trim_end_matches(';').trim();
    parse_hex_or_dec(clean)
}

/// Parse a legacy-format field comment:
/// `/* FIELDNAME : ACCESS ;bitpos:[MSB:LSB] ;default: val ; */`
fn try_parse_legacy_field_comment(line: &str, c_prefix: &str) -> Option<ParsedField> {
    try_parse_legacy_field_comment_multi(line, &[c_prefix])
}

fn try_parse_legacy_field_comment_multi(line: &str, prefixes: &[&str]) -> Option<ParsedField> {
    let trimmed = line.trim();
    if !trimmed.starts_with("/*") {
        return None;
    }
    // Strip the leading `/*` and optional trailing `*/`
    let content = &trimmed[2..];
    let inner = if content.ends_with("*/") {
        content[..content.len() - 2].trim()
    } else {
        content.trim_end_matches(';').trim()
    };
    // Skip description comments, and pure block comments without bitpos
    if inner.starts_with("description:") || inner.starts_with('*') || inner.starts_with('!') {
        return None;
    }
    // Must contain "bitpos:" to be a field comment
    if !inner.contains("bitpos:") {
        return None;
    }

    // Format: `FIELDNAME : ACCESS ;bitpos:[...] ;...`
    let colon_pos = inner.find(" :")?;
    let raw_name = inner[..colon_pos].trim();
    let rest = inner[colon_pos + 2..].trim();

    // Access: up to the first ';'
    let semi_pos = rest.find(';').unwrap_or(rest.len());
    let access_str = rest[..semi_pos].trim();
    let access = AccessType::from_str(access_str);
    let rest = if semi_pos < rest.len() {
        rest[semi_pos + 1..].trim()
    } else {
        ""
    };

    // Bitpos
    let bp_pos = rest.find("bitpos:")?;
    let (bit_offset, bit_width) = parse_bitpos(&rest[bp_pos + 7..])?;

    let field_name = strip_prefix_multi(raw_name, prefixes, false);

    Some(ParsedField {
        name: field_name,
        description: String::new(),
        bit_offset,
        bit_width,
        access,
    })
}

pub fn parse_legacy(content: &str, c_prefix: &str) -> Vec<ParsedRegister> {
    parse_legacy_multi(content, &[c_prefix])
}

/// Like `parse_legacy` but tries multiple prefixes for each register (longest first).
pub fn parse_legacy_multi(content: &str, prefixes: &[&str]) -> Vec<ParsedRegister> {
    // Pre-scan for `#define NAME_SIZE_BYTES N` to handle memory-region dim arrays.
    let mut mem_size_bytes: HashMap<String, u32> = HashMap::new();
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("#define ") {
            let rest = t[8..].trim_start();
            let name_end = rest
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(rest.len());
            let macro_name = &rest[..name_end];
            if macro_name.ends_with("_SIZE_BYTES") {
                let val_str = rest[name_end..].trim();
                if let Some(n) = parse_hex_or_dec(val_str) {
                    let base = macro_name[..macro_name.len() - "_SIZE_BYTES".len()].to_string();
                    mem_size_bytes.insert(base, n);
                }
            }
        }
    }

    let mut registers: Vec<ParsedRegister> = Vec::new();
    let mut current_reg: Option<ParsedRegister> = None;
    let mut pending_field: Option<ParsedField> = None;

    let finalize_field = |pending: Option<ParsedField>, reg: &mut Option<ParsedRegister>| {
        if let (Some(f), Some(r)) = (pending, reg.as_mut()) {
            if f.bit_width > 0 && f.bit_offset <= 31 {
                r.fields.push(f);
            }
        }
    };

    for line in content.lines() {
        let trimmed = line.trim();

        // Register declaration?
        if let Some((raw_name, offset)) = try_parse_reg_define_line(line) {
            // Finalize pending field into current register
            finalize_field(pending_field.take(), &mut current_reg);
            // Save completed register
            if let Some(r) = current_reg.take() {
                if !r.name.is_empty() {
                    registers.push(r);
                }
            }
            let reg_name = strip_prefix_multi(&raw_name, prefixes, true);
            let (dim, dim_increment) = if raw_name.ends_with("_MEM") {
                if let Some(&sz) = mem_size_bytes.get(&raw_name) {
                    let words = sz / 4;
                    if words > 0 { (Some(words), Some(4u32)) } else { (None, None) }
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };
            let svd_name = if dim.is_some() { format!("{reg_name}[%s]") } else { reg_name };
            current_reg = Some(ParsedRegister {
                name: svd_name,
                description: String::new(),
                offset,
                size: 32,
                fields: Vec::new(),
                dim,
                dim_increment,
                dim_index: None,
            });
        }
        // Field comment (starts with `/*`, not a description)?
        else if trimmed.starts_with("/*") && current_reg.is_some() {
            if trimmed.starts_with("/*description:") || trimmed.starts_with("/* description:") {
                // Description line for the previous field
                let desc = trimmed
                    .trim_start_matches("/*description:")
                    .trim_start_matches("/* description:")
                    .trim()
                    .trim_end_matches("*/")
                    .trim()
                    .to_string();
                if let Some(ref mut f) = pending_field {
                    f.description = desc;
                }
            } else if let Some(field) = try_parse_legacy_field_comment_multi(line, prefixes) {
                // Complete previous pending field
                finalize_field(pending_field.replace(field), &mut current_reg);
            }
        }
    }

    // Finalize
    finalize_field(pending_field.take(), &mut current_reg);
    if let Some(r) = current_reg.take() {
        if !r.name.is_empty() {
            registers.push(r);
        }
    }

    registers
}

// ---------------------------------------------------------------------------
// New format parser
// ---------------------------------------------------------------------------

/// Parse a new-format field comment block into a `ParsedField`.
/// The block is a vec of lines from `/**` to `*/`.
fn parse_new_field_comment(lines: &[String], c_prefix: &str) -> Option<ParsedField> {
    parse_new_field_comment_multi(lines, &[c_prefix])
}

fn parse_new_field_comment_multi(lines: &[String], prefixes: &[&str]) -> Option<ParsedField> {
    // First line: `/** FIELDNAME : ACCESS; bitpos: [MSB:LSB]; default: N;`
    //          or `/* FIELDNAME : ACCESS; bitpos: [MSB:LSB]; default: N;`
    let first = lines.first()?;
    let trimmed_first = first.trim();
    let inner = if trimmed_first.starts_with("/**") {
        trimmed_first.trim_start_matches("/**")
    } else {
        trimmed_first.trim_start_matches("/*")
    }
    .trim()
    .trim_end_matches("*/")
    .trim();

    // Detect field comment: must contain "bitpos:"
    if !inner.contains("bitpos:") {
        return None;
    }

    // Split on ':'
    let colon_pos = inner.find(" :")?;
    let raw_name = inner[..colon_pos].trim();
    let rest = inner[colon_pos + 2..].trim();

    // Access: up to ';'
    let semi_pos = rest.find(';').unwrap_or(rest.len());
    let access_str = rest[..semi_pos].trim();
    let access = AccessType::from_str(access_str);
    let rest = &rest[semi_pos..];

    // Find bitpos:
    let bp_pos = rest.find("bitpos:")?;
    let (bit_offset, bit_width) = parse_bitpos(&rest[bp_pos + 7..])?;

    // Description from subsequent lines
    let description = lines[1..]
        .iter()
        .filter_map(|l| {
            let s = l
                .trim()
                .trim_start_matches('*')
                .trim()
                .trim_end_matches("*/")
                .trim();
            if s.is_empty() { None } else { Some(s.to_string()) }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let field_name = strip_prefix_multi(raw_name, prefixes, false);

    Some(ParsedField {
        name: field_name,
        description,
        bit_offset,
        bit_width,
        access,
    })
}

/// Detect whether a `/** ... */` comment block describes a register (ends with
/// "register" or "registers") or a field.
fn is_reg_comment(lines: &[String]) -> bool {
    let first = lines.first().map(|s| s.as_str()).unwrap_or("");
    let text = first
        .trim()
        .trim_start_matches("/**")
        .trim()
        .trim_end_matches("*/")
        .trim()
        .to_lowercase();
    text.ends_with("register") || text.ends_with("registers") || text.ends_with("reg")
}

/// Extract register description from a register comment block.
fn reg_comment_description(lines: &[String]) -> String {
    lines[1..]
        .iter()
        .filter_map(|l| {
            let s = l
                .trim()
                .trim_start_matches('*')
                .trim()
                .trim_end_matches("*/")
                .trim();
            if s.is_empty() { None } else { Some(s.to_string()) }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn parse_new(content: &str, c_prefix: &str) -> Vec<ParsedRegister> {
    parse_new_multi(content, &[c_prefix])
}

/// Like `parse_new` but tries multiple prefixes (longest first).
pub fn parse_new_multi(content: &str, prefixes: &[&str]) -> Vec<ParsedRegister> {
    // Pre-scan for `#define NAME_SIZE_BYTES N` to know which _MEM registers are arrays.
    let mut mem_size_bytes: HashMap<String, u32> = HashMap::new();
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("#define ") {
            let rest = t[8..].trim_start();
            let name_end = rest
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(rest.len());
            let macro_name = &rest[..name_end];
            if macro_name.ends_with("_SIZE_BYTES") {
                let val_str = rest[name_end..].trim();
                if let Some(n) = parse_hex_or_dec(val_str) {
                    // Strip the _SIZE_BYTES suffix to get the base memory register name.
                    let base = macro_name[..macro_name.len() - "_SIZE_BYTES".len()].to_string();
                    mem_size_bytes.insert(base, n);
                }
            }
        }
    }

    let mut registers: Vec<ParsedRegister> = Vec::new();
    let mut current_reg: Option<ParsedRegister> = None;
    let mut pending_field: Option<ParsedField> = None;
    let mut comment_buf: Vec<String> = Vec::new();
    let mut in_comment = false;
    let mut pending_reg_desc: Option<String> = None;

    let finalize_field = |pending: Option<ParsedField>, reg: &mut Option<ParsedRegister>| {
        if let (Some(f), Some(r)) = (pending, reg.as_mut()) {
            if f.bit_width > 0 && f.bit_offset <= 31 {
                r.fields.push(f);
            }
        }
    };

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("/**") || (trimmed.starts_with("/*") && !in_comment) {
            in_comment = true;
            comment_buf.clear();
            comment_buf.push(trimmed.to_string());
            // Single-line `/** ... */` or `/* ... */`
            if trimmed.ends_with("*/") && trimmed.len() > 4 {
                in_comment = false;
                if is_reg_comment(&comment_buf) {
                    pending_reg_desc = Some(reg_comment_description(&comment_buf));
                } else if let Some(f) = parse_new_field_comment_multi(&comment_buf, prefixes) {
                    finalize_field(pending_field.replace(f), &mut current_reg);
                } else if current_reg.is_some() {
                    // Try legacy single-line field comment format
                    if let Some(f) = try_parse_legacy_field_comment_multi(trimmed, prefixes) {
                        finalize_field(pending_field.replace(f), &mut current_reg);
                    }
                }
                comment_buf.clear();
            }
        } else if in_comment {
            comment_buf.push(trimmed.to_string());
            if trimmed.ends_with("*/") {
                in_comment = false;
                if is_reg_comment(&comment_buf) {
                    pending_reg_desc = Some(reg_comment_description(&comment_buf));
                } else if let Some(f) = parse_new_field_comment_multi(&comment_buf, prefixes) {
                    finalize_field(pending_field.replace(f), &mut current_reg);
                } else if current_reg.is_some() {
                    // Hybrid format: /* FIELDNAME : ACCESS; bitpos:[N]; ... \n * desc\n */
                    // Try the first line as a legacy-style field comment.
                    if let Some(first) = comment_buf.first() {
                        if let Some(f) = try_parse_legacy_field_comment_multi(first, prefixes) {
                            finalize_field(pending_field.replace(f), &mut current_reg);
                        }
                    }
                }
                comment_buf.clear();
            }
        } else if let Some((raw_name, offset)) = try_parse_reg_define_line(line) {
            finalize_field(pending_field.take(), &mut current_reg);
            if let Some(r) = current_reg.take() {
                if !r.name.is_empty() {
                    registers.push(r);
                }
            }
            let reg_name = strip_prefix_multi(&raw_name, prefixes, true);
            let desc = pending_reg_desc.take().unwrap_or_default();
            // For memory-region defines (raw name ends with `_MEM`), check if a
            // corresponding `_SIZE_BYTES` define sets the byte-length of the region.
            // If so, emit it as a dim array of 32-bit words.
            let (dim, dim_increment) = if raw_name.ends_with("_MEM") {
                if let Some(&sz) = mem_size_bytes.get(&raw_name) {
                    let words = sz / 4;
                    if words > 0 {
                        (Some(words), Some(4u32))
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };
            // Dim array registers use `%s` placeholder suffix in SVD names.
            let svd_name = if dim.is_some() {
                format!("{reg_name}[%s]")
            } else {
                reg_name
            };
            current_reg = Some(ParsedRegister {
                name: svd_name,
                description: desc,
                offset,
                size: 32,
                fields: Vec::new(),
                dim,
                dim_increment,
                dim_index: None,
            });
        }
    }

    finalize_field(pending_field.take(), &mut current_reg);
    if let Some(r) = current_reg.take() {
        if !r.name.is_empty() {
            registers.push(r);
        }
    }

    registers
}

// ---------------------------------------------------------------------------
// Public entry: parse a single C header file
// ---------------------------------------------------------------------------

/// Parse an IDF register header file and return a flat list of registers.
/// `c_prefix` is the C macro prefix to strip (e.g. `"GPIO"` for `gpio_reg.h`).
pub fn parse_header(path: &Path, c_prefix: &str) -> Result<Vec<ParsedRegister>> {
    parse_header_multi(path, &[c_prefix])
}

/// Like `parse_header` but tries multiple prefixes (longest first).
pub fn parse_header_multi(path: &Path, prefixes: &[&str]) -> Result<Vec<ParsedRegister>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("reading header {}", path.display()))?;
    let regs = match detect_format(&content) {
        HeaderFormat::Legacy => parse_legacy_multi(&content, prefixes),
        HeaderFormat::New => parse_new_multi(&content, prefixes),
    };
    log::debug!(
        "parsed {:?} from '{}': {} registers",
        prefixes,
        path.display(),
        regs.len()
    );
    Ok(regs)
}

// ---------------------------------------------------------------------------
// Fallback YAML reader
// ---------------------------------------------------------------------------

/// Load the fallback `idf-pipeline/yml/<name>.yml` file and return its registers.
fn load_fallback_yml(yml_path: &Path, periph_type: &str) -> Result<Vec<ParsedRegister>> {
    let content = fs::read_to_string(yml_path)
        .with_context(|| format!("reading fallback yml {}", yml_path.display()))?;

    let parsed: FallbackYml = serde_yaml::from_str(&content)
        .with_context(|| format!("parsing fallback yml {}", yml_path.display()))?;

    // Prefer an entry whose name matches the peripheral type; fall back to first.
    let mut peripherals = parsed.peripherals;
    let periph = if let Some(pos) = peripherals
        .iter()
        .position(|p| p.name.eq_ignore_ascii_case(periph_type))
    {
        peripherals.remove(pos)
    } else if !peripherals.is_empty() {
        peripherals.remove(0)
    } else {
        return Ok(Vec::new());
    };

    let c_prefix = periph_type;
    let mut registers = Vec::new();

    for group in periph.register_groups {
        for reg in group.registers {
            let raw_name = &reg.name;
            let reg_name = strip_prefix(raw_name, c_prefix, true);
            let offset = parse_yaml_int(&reg.addr) as u32;

            let mut fields = Vec::new();
            for f in reg.fields {
                let mask_val = parse_yaml_int(&f.mask);
                if mask_val == 0 {
                    continue;
                }
                let bit_offset = mask_to_offset(mask_val);
                let bit_width = mask_to_width(mask_val);
                let field_name = strip_prefix(&f.name, c_prefix, false);
                let access = if f.access.is_empty() {
                    AccessType::ReadWrite
                } else {
                    AccessType::from_str(&f.access)
                };
                fields.push(ParsedField {
                    name: field_name,
                    description: f.description,
                    bit_offset,
                    bit_width,
                    access,
                });
            }

            registers.push(ParsedRegister {
                name: reg_name,
                description: reg.description,
                offset,
                size: reg.size,
                fields,
                dim: reg.dim,
                dim_increment: reg.dim_increment,
                dim_index: reg.dim_index,
            });
        }
    }

    Ok(registers)
}

// ---------------------------------------------------------------------------
// SVD XML generation
// ---------------------------------------------------------------------------

/// Escape text for inclusion in XML.
fn xml_escape(s: &str) -> String {
    // Strip characters not legal in XML 1.0 (e.g. vertical tab \u{b} from C comments)
    let cleaned: String = s
        .chars()
        .filter(|&c| {
            matches!(c, '\t' | '\n' | '\r')
                || (c >= '\u{20}' && c <= '\u{D7FF}')
                || (c >= '\u{E000}' && c <= '\u{FFFD}')
                || (c >= '\u{10000}' && c <= '\u{10FFFF}')
        })
        .collect();
    cleaned
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Compute the `addressBlock.size` for a peripheral as the next multiple of
/// `0x100` above `max_register_offset + 4`, with a minimum of `0x100`.
fn compute_addr_block_size(registers: &[ParsedRegister]) -> u32 {
    let max_off = registers.iter().map(|r| r.offset).max().unwrap_or(0);
    let raw = max_off + 4;
    let aligned = ((raw + 0xFF) / 0x100) * 0x100;
    aligned.max(0x100)
}

struct SvdPeripheral<'a> {
    instance_name: &'a str,
    periph_type: &'a str,
    base_addr: u64,
    description: &'a str,
    interrupts: Vec<&'a PeripheralInterrupt>,
    registers: &'a [ParsedRegister],
    derived_from: Option<&'a str>,
}

fn render_peripheral(p: &SvdPeripheral<'_>, out: &mut String) {
    if let Some(base) = p.derived_from {
        out.push_str(&format!(
            "    <peripheral derivedFrom=\"{}\">\n",
            xml_escape(base)
        ));
    } else {
        out.push_str("    <peripheral>\n");
    }

    out.push_str(&format!(
        "      <name>{}</name>\n",
        xml_escape(p.instance_name)
    ));
    out.push_str(&format!(
        "      <description>{}</description>\n",
        xml_escape(p.description)
    ));

    if p.derived_from.is_none() {
        out.push_str(&format!(
            "      <groupName>{}</groupName>\n",
            xml_escape(p.periph_type)
        ));
    }

    out.push_str(&format!(
        "      <baseAddress>0x{:08X}</baseAddress>\n",
        p.base_addr
    ));

    if p.derived_from.is_none() && !p.registers.is_empty() {
        let block_size = compute_addr_block_size(p.registers);
        out.push_str(&format!(
            "      <addressBlock>\n        <offset>0x0</offset>\n        <size>0x{:X}</size>\n        <usage>registers</usage>\n      </addressBlock>\n",
            block_size
        ));
    } else if p.derived_from.is_none() {
        // No registers — still emit a minimal address block
        out.push_str(
            "      <addressBlock>\n        <offset>0x0</offset>\n        <size>0x100</size>\n        <usage>registers</usage>\n      </addressBlock>\n",
        );
    }

    for intr in &p.interrupts {
        out.push_str(&format!(
            "      <interrupt>\n        <name>{}</name>\n        <value>{}</value>\n      </interrupt>\n",
            xml_escape(&intr.name),
            intr.value
        ));
    }

    if p.derived_from.is_none() && !p.registers.is_empty() {
        out.push_str("      <registers>\n");
        for reg in p.registers {
            render_register(reg, out);
        }
        out.push_str("      </registers>\n");
    }

    out.push_str("    </peripheral>\n");
}

fn render_register(reg: &ParsedRegister, out: &mut String) {
    out.push_str("        <register>\n");
    if let Some(dim) = reg.dim {
        out.push_str(&format!("          <dim>{dim}</dim>\n"));
        let inc = reg.dim_increment.unwrap_or(4);
        out.push_str(&format!("          <dimIncrement>0x{inc:X}</dimIncrement>\n"));
        if let Some(ref idx) = reg.dim_index {
            out.push_str(&format!("          <dimIndex>{idx}</dimIndex>\n"));
        }
    }
    out.push_str(&format!(
        "          <name>{}</name>\n",
        xml_escape(&reg.name)
    ));
    if !reg.description.is_empty() {
        out.push_str(&format!(
            "          <description>{}</description>\n",
            xml_escape(&reg.description)
        ));
    }
    out.push_str(&format!(
        "          <addressOffset>0x{:X}</addressOffset>\n",
        reg.offset
    ));
    out.push_str(&format!(
        "          <size>0x{:X}</size>\n",
        reg.size
    ));
    if !reg.fields.is_empty() {
        out.push_str("          <fields>\n");
        for field in &reg.fields {
            render_field(field, out);
        }
        out.push_str("          </fields>\n");
    }
    out.push_str("        </register>\n");
}

fn render_field(field: &ParsedField, out: &mut String) {
    out.push_str("            <field>\n");
    out.push_str(&format!(
        "              <name>{}</name>\n",
        xml_escape(&field.name)
    ));
    if !field.description.is_empty() {
        out.push_str(&format!(
            "              <description>{}</description>\n",
            xml_escape(&field.description)
        ));
    }
    out.push_str(&format!(
        "              <bitOffset>{}</bitOffset>\n",
        field.bit_offset
    ));
    out.push_str(&format!(
        "              <bitWidth>{}</bitWidth>\n",
        field.bit_width
    ));
    out.push_str(&format!(
        "              <access>{}</access>\n",
        field.access.as_svd_str()
    ));
    out.push_str("            </field>\n");
}

// ---------------------------------------------------------------------------
// Main entry point
// ---------------------------------------------------------------------------

/// Chip metadata for SVD generation.
struct ChipMeta {
    /// Pretty name: e.g. `"ESP32-C3"`
    name: String,
    /// Series: e.g. `"ESP32 C-Series"`
    series: String,
    /// Short description
    description: String,
    /// CPU name for SVD
    cpu_name: String,
}

fn chip_meta(chip: &str) -> ChipMeta {
    // Derive a pretty name like "ESP32-C3" from "esp32c3"
    let upper = chip.to_uppercase();
    let pretty = upper.replacen("ESP32", "ESP32-", 1).trim_end_matches('-').to_string();

    let (series, description, cpu_name) = match chip {
        c if c.starts_with("esp32c") || c.starts_with("esp32h") => (
            "ESP32 C/H-Series".to_string(),
            format!("{} MCU", pretty),
            "RV32IMC".to_string(),
        ),
        c if c.starts_with("esp32s") => (
            "ESP32 S-Series".to_string(),
            format!("{} MCU", pretty),
            "LX7".to_string(),
        ),
        "esp32" => (
            "ESP32".to_string(),
            "ESP32 dual-core MCU with Wi-Fi and Bluetooth".to_string(),
            "LX6".to_string(),
        ),
        _ => (
            "ESP32".to_string(),
            format!("{} MCU", pretty),
            "RV32IMC".to_string(),
        ),
    };

    ChipMeta { name: pretty, series, description, cpu_name }
}

// ---------------------------------------------------------------------------
// Auto-discovery helpers
// ---------------------------------------------------------------------------

/// Extract peripheral instances (name, type, base_addr) from an existing SVD file.
/// Peripherals that use `derivedFrom` are treated as additional instances of the same type.
fn peripherals_from_svd(svd_path: &Path) -> Vec<PeripheralInstance> {
    use std::str::FromStr;
    let content = match fs::read_to_string(svd_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    // Quick-and-dirty XML walk without a full DOM – we only need <peripheral> elements.
    // Use xml_rs or quick-xml is not available; do it with the stdlib XML parser via
    // minidom-like linear scan. But since we already have xml.etree via Python or
    // Rust's quick-xml is not linked, just use a regex-free line scan approach.
    // Actually use roxmltree which is already indirectly available via svdtools – but
    // let's use a simpler approach: parse with the standard library's xml::reader.
    //
    // We use the xmlparser approach: scan for <peripheral> … </peripheral> blocks.
    let mut result = Vec::new();
    // Build a minimal state machine over the raw XML text.
    let mut pos = 0;
    let bytes = content.as_bytes();
    while pos < bytes.len() {
        // Find next '<peripheral' (could be '<peripheral>' or '<peripheral derivedFrom=...')
        if let Some(start) = content[pos..].find("<peripheral") {
            let abs = pos + start;
            // Find the closing '>' of the opening tag
            let tag_end = match content[abs..].find('>') {
                Some(e) => abs + e,
                None => break,
            };
            let open_tag = &content[abs..=tag_end];
            // Get derivedFrom attribute if present
            let derived_from: Option<String> = if let Some(df) = open_tag.find("derivedFrom=\"") {
                let rest = &open_tag[df + 13..];
                rest.find('"').map(|end| rest[..end].to_string())
            } else {
                None
            };
            // Find </peripheral>
            let close = match content[tag_end..].find("</peripheral>") {
                Some(c) => tag_end + c + 13,
                None => break,
            };
            let block = &content[abs..close];
            // Extract <name>
            let name = extract_xml_text(block, "name").unwrap_or_default();
            // Extract <baseAddress>
            let base_str = extract_xml_text(block, "baseAddress").unwrap_or_default();
            let base_addr: u64 = u64::from_str_radix(base_str.trim_start_matches("0x").trim_start_matches("0X"), 16)
                .or_else(|_| u64::from_str(&base_str).map_err(|_| ()))
                .unwrap_or(0);
            if !name.is_empty() {
                // The peripheral "type" for derivedFrom instances is the derivedFrom name.
                // For the primary instance, the type = name.
                // We don't know the type yet (there's no explicit type in SVD) –
                // but derivedFrom gives us the first instance name.
                let periph_type = derived_from.clone().unwrap_or_else(|| name.to_string());
                result.push(PeripheralInstance {
                    name: name.to_string(),
                    peripheral: periph_type,
                    base_addr: serde_yaml::Value::Number(serde_yaml::Number::from(base_addr)),
                });
            }
            pos = close;
        } else {
            break;
        }
    }
    result
}

/// Extract all text content of the first `<tag>...</tag>` pair found in `block`.
fn extract_xml_text<'a>(block: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = block.find(&open)? + open.len();
    let end = block[start..].find(&close)?;
    Some(block[start..start + end].trim())
}

/// Extract register data for a specific peripheral type from an existing SVD file.
/// Returns parsed registers so they can be used as fallback data.
fn registers_from_svd(svd_path: &Path, periph_name: &str) -> Vec<ParsedRegister> {
    let content = match fs::read_to_string(svd_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    // Find the peripheral block for `periph_name` (the first non-derivedFrom instance)
    let mut pos = 0;
    while pos < content.len() {
        if let Some(start) = content[pos..].find("<peripheral") {
            let abs = pos + start;
            let tag_end = match content[abs..].find('>') {
                Some(e) => abs + e,
                None => break,
            };
            let open_tag = &content[abs..=tag_end];
            // Skip derivedFrom instances (they have no registers)
            if open_tag.contains("derivedFrom") {
                pos = tag_end + 1;
                continue;
            }
            let close = match content[tag_end..].find("</peripheral>") {
                Some(c) => tag_end + c + 13,
                None => break,
            };
            let block = &content[abs..close];
            let name = extract_xml_text(block, "name").unwrap_or_default();
            if name.eq_ignore_ascii_case(periph_name) {
                return parse_registers_from_svd_block(block);
            }
            pos = close;
        } else {
            break;
        }
    }
    Vec::new()
}

/// Parse `<register>` elements from an SVD peripheral block into `ParsedRegister`s.
fn parse_registers_from_svd_block(block: &str) -> Vec<ParsedRegister> {
    let mut result = Vec::new();
    let mut pos = 0;
    while pos < block.len() {
        if let Some(start) = block[pos..].find("<register>").or_else(|| {
            // also handle <register> that has no dim but starts with whitespace
            block[pos..].find("<register>")
        }) {
            let abs = pos + start;
            let close = match block[abs..].find("</register>") {
                Some(c) => abs + c + 11,
                None => break,
            };
            let reg_block = &block[abs..close];
            let name = extract_xml_text(reg_block, "name")
                .unwrap_or_default()
                .to_string();
            let desc = extract_xml_text(reg_block, "description")
                .unwrap_or_default()
                .to_string();
            let offset_str = extract_xml_text(reg_block, "addressOffset").unwrap_or("0x0");
            let offset = u32::from_str_radix(
                offset_str.trim().trim_start_matches("0x").trim_start_matches("0X"),
                16,
            )
            .or_else(|_| offset_str.trim().parse())
            .unwrap_or(0);

            let dim = extract_xml_text(reg_block, "dim")
                .and_then(|s| s.parse::<u32>().ok());
            let dim_increment = extract_xml_text(reg_block, "dimIncrement")
                .and_then(|s| {
                    u32::from_str_radix(s.trim().trim_start_matches("0x"), 16)
                        .or_else(|_| s.trim().parse())
                        .ok()
                });
            let dim_index = extract_xml_text(reg_block, "dimIndex").map(str::to_string);

            // Parse fields
            let mut fields = Vec::new();
            let mut fpos = 0;
            while fpos < reg_block.len() {
                if let Some(fs) = reg_block[fpos..].find("<field>") {
                    let fabs = fpos + fs;
                    let fclose = match reg_block[fabs..].find("</field>") {
                        Some(c) => fabs + c + 8,
                        None => break,
                    };
                    let fb = &reg_block[fabs..fclose];
                    let fname = extract_xml_text(fb, "name").unwrap_or_default().to_string();
                    let fdesc = extract_xml_text(fb, "description").unwrap_or_default().to_string();
                    let bo = extract_xml_text(fb, "bitOffset")
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0);
                    let bw = extract_xml_text(fb, "bitWidth")
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(1);
                    let acc_str = extract_xml_text(fb, "access").unwrap_or("read-write");
                    let access = AccessType::from_str(acc_str);
                    if !fname.is_empty() {
                        fields.push(ParsedField {
                            name: fname,
                            description: fdesc,
                            bit_offset: bo,
                            bit_width: bw,
                            access,
                        });
                    }
                    fpos = fclose;
                } else {
                    break;
                }
            }

            if !name.is_empty() {
                result.push(ParsedRegister {
                    name,
                    description: desc,
                    offset,
                    size: 32,
                    fields,
                    dim,
                    dim_increment,
                    dim_index,
                });
            }
            pos = close;
        } else {
            break;
        }
    }
    result
}

/// Try to auto-discover a register header file for a peripheral type in the IDF tree.
/// Checks `register/soc/<ptype_lowercase>_reg.h` and a few other patterns.
fn auto_discover_header(idf_path: &Path, chip: &str, ptype: &str) -> Option<PathBuf> {
    let candidates = [
        format!("{}_reg.h", ptype.to_lowercase()),
        format!("{}_reg.h", ptype.to_lowercase().replace("_", "")),
    ];
    let search_dirs = [
        idf_path.join(format!("components/soc/{}/register/soc", chip)),
        idf_path.join(format!("components/soc/{}/include/soc", chip)),
    ];
    for dir in &search_dirs {
        for name in &candidates {
            let p = dir.join(name);
            if p.is_file() {
                return Some(p);
            }
        }
    }
    None
}

/// Generate a CMSIS-SVD XML string from IDF headers.
///
/// All configuration files in `chip_idf_pipeline_dir` are **optional**:
///
/// | File | Fallback when absent |
/// |------|----------------------|
/// | `peripherals.yml` | Extracted from `<chip>.base.svd` (peripheral names + base addrs) |
/// | `interrupts.yml` | No interrupts in SVD (empty) |
/// | `peripheral_descriptions.yml` | Empty descriptions |
/// | `header_map.yml` | Auto-discover `<ptype_lowercase>_reg.h` in IDF headers |
/// | `yml/<ptype>.yml` | Registers extracted from existing `<chip>.base.svd` |
///
/// Peripherals that end up with **zero registers** (no parseable header, no fallback yml,
/// not present in the existing SVD) are listed as warnings at the end.
pub fn generate_base_svd(
    chip: &str,
    chip_idf_pipeline_dir: &Path,
    idf_path: &Path,
) -> Result<String> {
    // path to the chip's SVD directory (for fallback extraction)
    // chip_idf_pipeline_dir is e.g. esp32c3/idf-pipeline; SVD lives at esp32c3/svd/
    let chip_dir = chip_idf_pipeline_dir.parent().unwrap_or(chip_idf_pipeline_dir);
    let existing_svd = chip_dir.join(format!("svd/{}.base.svd", chip));

    // -----------------------------------------------------------------------
    // Load metadata YAMLs  (all optional – fall back to auto-derived data)
    // -----------------------------------------------------------------------
    let periph_yml: PeripheralsYml = load_yaml_opt(chip_idf_pipeline_dir.join("peripherals.yml"))
        .unwrap_or_else(|| {
            // Derive from existing base SVD
            let instances = if existing_svd.is_file() {
                log::info!("peripherals.yml absent – deriving peripheral list from {}", existing_svd.display());
                peripherals_from_svd(&existing_svd)
            } else {
                log::warn!("peripherals.yml absent and no existing base SVD – peripheral list will be empty");
                Vec::new()
            };
            PeripheralsYml { peripheral_instances: instances }
        });

    let intr_yml: InterruptsYml = load_yaml_opt(chip_idf_pipeline_dir.join("interrupts.yml"))
        .unwrap_or_else(|| {
            log::debug!("interrupts.yml absent – SVD will have no interrupt entries");
            InterruptsYml { peripheral_interrupts: Vec::new() }
        });

    let desc_yml: DescriptionsYml =
        load_yaml_opt(chip_idf_pipeline_dir.join("peripheral_descriptions.yml"))
        .unwrap_or_else(|| {
            log::debug!("peripheral_descriptions.yml absent – using empty descriptions");
            DescriptionsYml { peripheral_descriptions: Vec::new() }
        });

    let header_map: HeaderMapYml =
        load_yaml_opt(chip_idf_pipeline_dir.join("header_map.yml"))
        .unwrap_or_else(|| {
            log::info!("header_map.yml absent – will auto-discover header files");
            HeaderMapYml { peripherals: HashMap::new() }
        });

    // Build lookup maps
    let desc_map: HashMap<String, String> = desc_yml
        .peripheral_descriptions
        .into_iter()
        .map(|d| (d.peripheral, d.description))
        .collect();

    let mut intr_by_instance: HashMap<String, Vec<&PeripheralInterrupt>> = HashMap::new();
    for intr in &intr_yml.peripheral_interrupts {
        intr_by_instance
            .entry(intr.instance.clone())
            .or_default()
            .push(intr);
    }

    // -----------------------------------------------------------------------
    // Parse registers for each peripheral TYPE
    // -----------------------------------------------------------------------
    // Track which peripheral types have been parsed (type → registers)
    let mut type_registers: HashMap<String, Vec<ParsedRegister>> = HashMap::new();

    // Collect unique peripheral types from the instance list
    let mut periph_types: Vec<String> = periph_yml
        .peripheral_instances
        .iter()
        .map(|p| p.peripheral.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    periph_types.sort();

    let yml_dir = chip_idf_pipeline_dir.join("yml");
    let common_yml_dir = chip_idf_pipeline_dir.join("common-yml");

    for ptype in &periph_types {
        let entry = header_map.peripherals.get(ptype.as_str());
        let default_entry = HeaderEntry::default();
        let e = entry.unwrap_or(&default_entry);

        let registers = if e.skip {
            // Explicit skip: try yml fallback, then existing SVD
            let from_yml = load_fallback_for_type(ptype, e, &yml_dir, &common_yml_dir);
            if from_yml.is_empty() && existing_svd.is_file() {
                let from_svd = registers_from_svd(&existing_svd, ptype);
                if !from_svd.is_empty() {
                    log::info!("extracted {} registers for {} from existing SVD", from_svd.len(), ptype);
                }
                from_svd
            } else {
                from_yml
            }
        } else {
            // Determine header filename: explicit > auto-discover
            let explicit_file = e.file.as_deref();
            let auto_path = if explicit_file.is_none() && !e.skip {
                auto_discover_header(idf_path, chip, ptype)
            } else {
                None
            };

            let header_path = if let Some(fname) = explicit_file {
                find_reg_header(idf_path, chip, fname)
            } else {
                auto_path
            };

            match header_path {
                Some(path) => {
                    let main_prefix = e.prefix.as_deref().unwrap_or(ptype.as_str());
                    let mut all_prefixes: Vec<&str> =
                        e.alt_prefixes.iter().map(|s| s.as_str()).collect();
                    all_prefixes.push(main_prefix);

                    match parse_header_multi(&path, &all_prefixes) {
                        Ok(mut regs) => {
                            for r in &mut regs {
                                if let Some(new_name) = e.renames.get(&r.name) {
                                    r.name = new_name.clone();
                                }
                            }
                            log::info!("parsed {} registers for {}", regs.len(), ptype);
                            regs
                        }
                        Err(err) => {
                            log::warn!("failed to parse header for {}: {}", ptype, err);
                            load_fallback_for_type(ptype, e, &yml_dir, &common_yml_dir)
                        }
                    }
                }
                None => {
                    // No header found: yml fallback, then existing SVD
                    if explicit_file.is_some() {
                        log::warn!(
                            "header file '{}' not found for {} in {}",
                            explicit_file.unwrap_or("?"),
                            ptype,
                            chip
                        );
                    }
                    let from_yml = load_fallback_for_type(ptype, e, &yml_dir, &common_yml_dir);
                    if from_yml.is_empty() && existing_svd.is_file() {
                        let from_svd = registers_from_svd(&existing_svd, ptype);
                        if !from_svd.is_empty() {
                            log::info!(
                                "extracted {} registers for {} from existing SVD",
                                from_svd.len(),
                                ptype
                            );
                        }
                        from_svd
                    } else {
                        from_yml
                    }
                }
            }
        };

        type_registers.insert(ptype.clone(), registers);
    }

    // Warn about peripherals with zero registers (likely need header_map.yml entries or fallbacks)
    let empty: Vec<&str> = periph_types
        .iter()
        .filter(|pt| type_registers.get(pt.as_str()).map_or(true, |r| r.is_empty()))
        .map(|s| s.as_str())
        .collect();
    if !empty.is_empty() {
        log::warn!(
            "The following peripheral types ended up with 0 registers – \
             add entries to header_map.yml or provide yml/{{}}.yml fallbacks:\n  {}",
            empty.join(", ")
        );
    }

    // -----------------------------------------------------------------------
    // Build SVD XML
    // -----------------------------------------------------------------------
    let meta = chip_meta(chip);
    let mut xml = String::with_capacity(1024 * 512);

    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<device schemaVersion=\"1.1\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema-instance\" xs:noNamespaceSchemaLocation=\"CMSIS-SVD_Schema_1_1.xsd\">\n");
    xml.push_str("  <vendor>ESPRESSIF SYSTEMS (SHANGHAI) CO., LTD.</vendor>\n");
    xml.push_str("  <vendorID>ESPRESSIF</vendorID>\n");
    xml.push_str(&format!("  <name>{}</name>\n", xml_escape(&meta.name)));
    xml.push_str(&format!("  <series>{}</series>\n", xml_escape(&meta.series)));
    xml.push_str("  <version>1</version>\n");
    xml.push_str(&format!(
        "  <description>{}</description>\n",
        xml_escape(&meta.description)
    ));
    xml.push_str("  <licenseText>Copyright 2025 Espressif Systems (Shanghai) PTE LTD\n\n    Licensed under the Apache License, Version 2.0 (the &quot;License&quot;);\n    you may not use this file except in compliance with the License.\n    You may obtain a copy of the License at\n\n        http://www.apache.org/licenses/LICENSE-2.0\n\n    Unless required by applicable law or agreed to in writing, software\n    distributed under the License is distributed on an &quot;AS IS&quot; BASIS,\n    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n    See the License for the specific language governing permissions and\n    limitations under the License.</licenseText>\n");
    xml.push_str("  <cpu>\n");
    xml.push_str(&format!(
        "    <name>{}</name>\n",
        xml_escape(&meta.cpu_name)
    ));
    xml.push_str("    <revision>r0p0</revision>\n");
    xml.push_str("    <endian>little</endian>\n");
    xml.push_str("    <mpuPresent>false</mpuPresent>\n");
    xml.push_str("    <fpuPresent>false</fpuPresent>\n");
    xml.push_str("    <nvicPrioBits>0</nvicPrioBits>\n");
    xml.push_str("    <vendorSystickConfig>false</vendorSystickConfig>\n");
    xml.push_str("  </cpu>\n");
    xml.push_str("  <addressUnitBits>32</addressUnitBits>\n");
    xml.push_str("  <width>32</width>\n");
    xml.push_str("  <resetValue>0x00000000</resetValue>\n");
    xml.push_str("  <resetMask>0xFFFFFFFF</resetMask>\n");
    xml.push_str("  <peripherals>\n");

    // Track which peripheral types have been emitted (for derivedFrom)
    let mut rendered_types: HashMap<String, String> = HashMap::new(); // type → first instance name

    for inst in &periph_yml.peripheral_instances {
        let ptype = &inst.peripheral;
        let instance_name = &inst.name;
        let base_addr = inst.base_address();

        let description = desc_map
            .get(instance_name.as_str())
            .or_else(|| desc_map.get(ptype.as_str()))
            .map(|s| s.as_str())
            .unwrap_or(instance_name.as_str());

        let interrupts = intr_by_instance
            .get(instance_name.as_str())
            .cloned()
            .unwrap_or_default();

        let regs = type_registers.get(ptype).map(|v| v.as_slice()).unwrap_or(&[]);

        let derived_from = if let Some(first_inst) = rendered_types.get(ptype) {
            Some(first_inst.as_str())
        } else {
            rendered_types.insert(ptype.clone(), instance_name.clone());
            None
        };

        let svd_periph = SvdPeripheral {
            instance_name,
            periph_type: ptype,
            base_addr,
            description,
            interrupts,
            registers: regs,
            derived_from,
        };

        render_peripheral(&svd_periph, &mut xml);
    }

    xml.push_str("  </peripherals>\n");
    xml.push_str("</device>\n");

    Ok(xml)
}

// ---------------------------------------------------------------------------
// Helpers for fallback loading
// ---------------------------------------------------------------------------

fn load_fallback_for_type(
    ptype: &str,
    entry: &HeaderEntry,
    yml_dir: &Path,
    common_yml_dir: &Path,
) -> Vec<ParsedRegister> {
    // Determine yml filename: explicit > ptype_lowercase.yml
    let yml_name = entry
        .yml
        .as_deref()
        .unwrap_or("")
        .to_string();
    let yml_name = if yml_name.is_empty() {
        format!("{}.yml", ptype.to_lowercase())
    } else {
        yml_name
    };

    // Search yml_dir first, then common_yml_dir
    for dir in &[yml_dir, common_yml_dir] {
        let path = dir.join(&yml_name);
        if path.is_file() {
            match load_fallback_yml(&path, ptype) {
                Ok(regs) => {
                    log::info!(
                        "loaded {} fallback registers for {} from {}",
                        regs.len(),
                        ptype,
                        path.display()
                    );
                    return regs;
                }
                Err(e) => {
                    // Show full error chain so the root serde_yaml error is visible
                    let chain: Vec<String> = e.chain().map(|c| c.to_string()).collect();
                    log::warn!("failed to read fallback yml {}:\n    {}", path.display(), chain.join("\n    caused by: "));
                }
            }
        }
    }

    log::debug!("no fallback found for {}, using empty register set", ptype);
    Vec::new()
}

fn load_yaml<T: for<'de> Deserialize<'de>>(path: PathBuf) -> Result<T> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    serde_yaml::from_str(&content)
        .with_context(|| format!("parsing {}", path.display()))
}

/// Like `load_yaml` but returns `None` if the file does not exist.
fn load_yaml_opt<T: for<'de> Deserialize<'de>>(path: PathBuf) -> Option<T> {
    if !path.is_file() {
        return None;
    }
    match load_yaml(path) {
        Ok(v) => Some(v),
        Err(e) => {
            log::warn!("failed to load optional config: {}", e);
            None
        }
    }
}
