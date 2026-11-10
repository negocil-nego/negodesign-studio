use super::paths::{
    encode_base64, normalize_logical_path, resolve_disk_path, transform_react_or_vue_hooks,
};
use std::{collections::BTreeMap, fs, path::Path};

const PUBLIC_ASSET_DIRECTORIES: &[&str] = &["hugeicons", "flags", "styles"];

pub fn add_source_file(
    root: &Path,
    logical_path: &str,
    files_by_path: &mut BTreeMap<String, String>,
    manifest_lang: &str,
    optional: bool,
) -> Result<(), String> {
    let normalized = normalize_logical_path(logical_path);
    let disk_path = resolve_disk_path(root, &normalized);
    if !disk_path.is_file() {
        if optional {
            return Ok(());
        }
        return Err(format!(
            "O ficheiro indicado no dependence.json não existe: {}",
            normalized
        ));
    }

    if normalized.ends_with(".woff2") {
        let bytes = fs::read(&disk_path).map_err(|e| e.to_string())?;
        files_by_path.insert(normalized, format!("base64:{}", encode_base64(&bytes)));
        return Ok(());
    }

    let mut content = fs::read_to_string(&disk_path).map_err(|e| e.to_string())?;
    let mut output_path = normalized;

    if manifest_lang == "React" || manifest_lang == "Vue" {
        if let Some(stripped) = output_path.strip_suffix(".svelte.ts") {
            output_path = format!("{stripped}.ts");
        }
        content = transform_react_or_vue_hooks(&content);
    }

    files_by_path.insert(output_path, content);
    Ok(())
}

pub fn add_asset_pattern(
    root: &Path,
    asset: &str,
    files_by_path: &mut BTreeMap<String, String>,
    manifest_lang: &str,
) -> Result<(), String> {
    let normalized = normalize_logical_path(asset);
    if let Some((parent_logical, pattern_name)) = normalized.rsplit_once('/') {
        if pattern_name.contains('*') {
            let parts: Vec<&str> = pattern_name.split('*').collect();
            let prefix = parts.first().copied().unwrap_or("");
            let suffix = parts.last().copied().unwrap_or("");
            let disk_dir = resolve_disk_path(root, parent_logical);
            if disk_dir.is_dir() {
                for entry in fs::read_dir(disk_dir).map_err(|e| e.to_string())?.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(fname) = path.file_name().and_then(|s| s.to_str()) {
                            if fname.starts_with(prefix) && fname.ends_with(suffix) {
                                let child_logical = format!("{parent_logical}/{fname}");
                                add_source_file(
                                    root,
                                    &child_logical,
                                    files_by_path,
                                    manifest_lang,
                                    false,
                                )?;
                            }
                        }
                    }
                }
            }
            return Ok(());
        }
    }
    add_source_file(root, &normalized, files_by_path, manifest_lang, false)
}

pub fn build_translations_barrel(locales: &[String]) -> Result<String, String> {
    if locales.is_empty() {
        return Err("Seleccione pelo menos uma língua para a internacionalização.".to_string());
    }
    let mut imports = Vec::new();
    let mut entries = Vec::new();
    for (idx, locale) in locales.iter().enumerate() {
        imports.push(format!("import language{idx} from \"./langs/{locale}\";"));
        entries.push(format!("  \"{locale}\": language{idx},"));
    }
    Ok(format!(
        "{}\n\nexport default {{\n{}\n}};\n",
        imports.join("\n"),
        entries.join("\n")
    ))
}

pub fn relocate_static_assets(
    files_by_path: BTreeMap<String, String>,
    directory: &str,
) -> BTreeMap<String, String> {
    let mut relocated = BTreeMap::new();
    for (path, mut content) in files_by_path {
        let target = if let Some(rest) = path.strip_prefix("static/") {
            format!("static/{directory}/{rest}")
        } else {
            path
        };
        if !content.starts_with("base64:") {
            for seg in PUBLIC_ASSET_DIRECTORIES {
                let from = format!("/{seg}/");
                let to = format!("/{directory}/{seg}/");
                content = content.replace(&from, &to);
            }
        }
        relocated.insert(target, content);
    }
    relocated
}

