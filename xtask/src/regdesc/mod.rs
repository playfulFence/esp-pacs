mod config;
mod idf;
mod idf_struct;
mod merge;
mod model;
mod repeat_infer;
mod svd;
mod util;

use std::path::Path;

use anyhow::{Context, Result};

use config::{chip_config_dir, load_descriptions, load_metadata, load_regdesc_config, peripheral_descriptions_path};
use model::RegdescFragment;

pub use config::default_output_path;

/// Main entry: load ESP-IDF headers + chip config, build the model, write an SVD file.
pub fn generate_base_svd(chip: &str, idf_path: &Path, output: &Path, version: u32) -> Result<()> {
    log::info!(
        "Generating base SVD for {chip} (version {version}) from IDF headers in {}",
        idf_path.display()
    );
    log::info!("Using chip config {}", chip_config_dir(chip).display());

    let fragment = load_fragment(chip, idf_path)?;
    write_svd(output, &fragment, version)
}

/// Parses one ESP-IDF register header and writes a minimal SVD containing that peripheral.
pub fn generate_from_idf_header(
    chip: &str,
    idf_path: &Path,
    header: &str,
    peripheral_name: Option<&str>,
    output: &Path,
    version: u32,
) -> Result<()> {
    let header_path = idf::resolve_header_path(idf_path, chip, header)
        .with_context(|| format!("could not find IDF header {header} for chip {chip}"))?;

    let (mut peripheral, stats) =
        idf::read_peripheral_header_file(&header_path, peripheral_name)?;

    apply_struct_hints(&header_path, &mut peripheral)?;
    repeat_infer::infer_repeats(&mut peripheral.register_groups[0].registers);
    for err in peripheral.merge_registers_fields() {
        log::warn!("merge warning in {header}: {}", err.0);
    }

    log::info!(
        "parsed IDF {header} -> {} ({} registers, {} unknown defines)",
        peripheral.name,
        stats.registers,
        stats.unknowns
    );

    let instance = model::PeripheralInstance {
        name: peripheral.name.clone(),
        peripheral: peripheral.name.clone(),
        base_addr: 0,
        description: Some(format!("Peripheral {}", peripheral.name)),
    };
    let fragment = config::build_fragment(chip, vec![peripheral], vec![instance], Vec::new());
    write_svd(output, &fragment, version)
}

/// Reads YAML config and ESP-IDF headers and assembles one in-memory chip description.
fn load_fragment(chip: &str, idf_path: &Path) -> Result<RegdescFragment> {
    let config_dir = chip_config_dir(chip);
    let regdesc_map = load_regdesc_config(&config_dir.join("regdesc.yml"))?;
    let (mut instances, interrupts) = load_metadata(&config_dir)?;
    let descriptions = load_descriptions(&peripheral_descriptions_path())?;

    config::apply_instance_descriptions(&mut instances, &descriptions);

    let mut entries: Vec<_> = regdesc_map.keys().cloned().collect();
    entries.sort();

    let mut peripherals = Vec::with_capacity(entries.len());
    for entry in entries {
        let options = &regdesc_map[&entry];
        if options.skip {
            log::info!("skipping {entry} (marked skip in regdesc.yml)");
            continue;
        }

        let header_file = idf::header_for_entry(&entry, options.idf.as_deref());
        let Some(header_path) = idf::resolve_header_path(idf_path, chip, &header_file) else {
            log::warn!("IDF header {header_file} not found for {entry}, skipping");
            continue;
        };

        let (mut peripheral, stats) = match idf::read_peripheral_header_file(&header_path, None) {
            Ok(parsed) => parsed,
            Err(_) if options.name.is_some() => {
                idf::read_peripheral_header_file(&header_path, options.name.as_deref())
                    .with_context(|| format!("parsing IDF header {}", header_path.display()))?
            }
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("parsing IDF header {}", header_path.display()));
            }
        };

        let idf_name = peripheral.name.clone();
        if let Some(name) = &options.name {
            peripheral.name = name.clone();
            peripheral.name_prefixes.clear();
            if idf_name == *name || idf_name.ends_with(name) {
                peripheral.name_prefixes.push(idf_name);
            }
        }
        peripheral.name_prefixes.push(peripheral.name.clone());
        peripheral
            .name_prefixes
            .extend(options.strip_prefixes.iter().cloned());
        peripheral.name_prefixes.sort_by_key(|prefix| std::cmp::Reverse(prefix.len()));
        peripheral.name_prefixes.dedup();

        apply_prefix_replacements(&mut peripheral, &options.prefix_replacements);
        if options.struct_arrays {
            apply_struct_hints(&header_path, &mut peripheral)?;
        }
        if options.infer_arrays {
            for group in &mut peripheral.register_groups {
                repeat_infer::infer_repeats(&mut group.registers);
            }
        }
        normalize_array_layouts(&mut peripheral, &options.normalize_array_layouts);
        preserve_flat_layouts(&mut peripheral, &options.preserve_flat_layouts);
        for err in peripheral.merge_registers_fields() {
            log::warn!("merge warning in {header_file}: {}", err.0);
        }

        if stats.unknowns > 0 {
            log::warn!(
                "IDF header {header_file} has {} unknown #define entries",
                stats.unknowns
            );
        }

        log::info!(
            "parsed IDF {header_file} -> {} ({} registers after merge)",
            peripheral.name,
            peripheral.register_count()
        );
        peripherals.push(peripheral);
    }

    Ok(config::build_fragment(chip, peripherals, instances, interrupts))
}

fn apply_struct_hints(header_path: &Path, peripheral: &mut model::Peripheral) -> Result<()> {
    let Some(struct_path) = idf_struct::companion_path(header_path) else {
        return Ok(());
    };
    let mut applied = 0usize;
    for group in &mut peripheral.register_groups {
        applied += idf_struct::apply_struct_layout_file(&struct_path, &mut group.registers)?;
    }
    if applied > 0 {
        log::debug!(
            "applied {applied} explicit array hints from {}",
            struct_path.display()
        );
    }
    Ok(())
}

fn apply_prefix_replacements(
    peripheral: &mut model::Peripheral,
    replacements: &std::collections::HashMap<String, String>,
) {
    let mut replacements: Vec<_> = replacements.iter().collect();
    replacements.sort_by_key(|(from, _)| std::cmp::Reverse(from.len()));

    for group in &mut peripheral.register_groups {
        for register in &mut group.registers {
            replace_prefix(&mut register.name, &replacements);
            for field in &mut register.fields {
                replace_prefix(&mut field.name, &replacements);
            }
        }
    }
}

fn replace_prefix(value: &mut String, replacements: &[(&String, &String)]) {
    if let Some((from, to)) = replacements
        .iter()
        .find(|(from, _)| value.starts_with(from.as_str()))
    {
        *value = format!("{to}{}", &value[from.len()..]);
    }
}

fn preserve_flat_layouts(peripheral: &mut model::Peripheral, templates: &[String]) {
    for group in &mut peripheral.register_groups {
        for register in &mut group.registers {
            if register
                .repeat_name_hint
                .as_ref()
                .is_some_and(|template| templates.contains(template))
            {
                register.repeat_name_hint = None;
                register.repeat_index_hint = None;
            }
        }
    }
}

fn normalize_array_layouts(peripheral: &mut model::Peripheral, templates: &[String]) {
    for group in &mut peripheral.register_groups {
        for template in templates {
            let indices: Vec<_> = group
                .registers
                .iter()
                .enumerate()
                .filter(|(_, register)| register.repeat_name_hint.as_ref() == Some(template))
                .map(|(index, _)| index)
                .collect();
            let Some(&prototype_index) = indices.first() else {
                continue;
            };
            let prototype = group.registers[prototype_index].clone();
            let prototype_name = prototype.name.trim_end_matches("_REG");
            for index in indices {
                let register_name = group.registers[index]
                    .name
                    .trim_end_matches("_REG")
                    .to_owned();
                group.registers[index].fields = prototype
                    .fields
                    .iter()
                    .cloned()
                    .map(|mut field| {
                        if field.name.starts_with(prototype_name) {
                            field.name =
                                field.name.replacen(prototype_name, &register_name, 1);
                        }
                        field
                    })
                    .collect();
            }
        }
    }
}

/// Turns the chip model into XML and saves it (creates parent dirs if needed).
fn write_svd(output: &Path, fragment: &RegdescFragment, version: u32) -> Result<()> {
    let xml = svd::write_svd(fragment, version);

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(output, xml).with_context(|| format!("writing SVD to {}", output.display()))?;

    log::info!("Wrote {}", output.display());
    Ok(())
}
