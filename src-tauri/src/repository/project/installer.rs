use crate::model::project::{ComponentExport, ComponentFile};
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

pub fn validate_relative_path(path: &str) -> Result<PathBuf, String> {
    let relative_path = Path::new(path);
    if relative_path.as_os_str().is_empty()
        || relative_path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err("O caminho do ficheiro de componente é inválido.".to_string());
    }
    Ok(relative_path.to_path_buf())
}

pub fn validate_export_name(name: &str) -> Result<(), String> {
    let mut chars = name.chars();
    let starts_valid = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$');
    if !starts_valid || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$') {
        return Err(format!("O nome de exportação '{name}' não é válido."));
    }
    Ok(())
}

pub fn build_index(language: &str, components: &mut [ComponentExport]) -> Result<String, String> {
    components.sort_by(|l, r| l.name.cmp(&r.name));
    let mut index = String::from("// Gerado pelo NegoDesign Studio.\n");
    for comp in components {
        validate_export_name(&comp.name)?;
        let entry = validate_relative_path(&comp.entry)?
            .to_string_lossy()
            .replace('\\', "/");

        if language == "Flutter" {
            index.push_str(&format!("export '{entry}';\n"));
        } else {
            let clean_entry = if language == "React" || language == "Nextjs" {
                entry.strip_suffix(".tsx").unwrap_or(&entry)
            } else {
                &entry
            };
            index.push_str(&format!(
                "export {{ default as {} }} from \"./{}\";\n",
                comp.name, clean_entry
            ));
        }
    }
    if language != "Flutter" {
        let href = if language == "Svelte" {
            "/nego/hugeicons/css/hugeicons.css"
        } else {
            "/hugeicons/css/hugeicons.css"
        };
        index.push_str(&format!(
            "\nif (typeof document !== \"undefined\" && !document.querySelector('link[data-negodesign-hugeicons]')) {{\n  const link = document.createElement(\"link\");\n  link.rel = \"stylesheet\";\n  link.href = \"{href}\";\n  link.dataset.negodesignHugeicons = \"true\";\n  document.head.append(link);\n}}\n",
        ));
    }
    Ok(index)
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    let mut decoded = Vec::with_capacity(input.len() * 3 / 4);
    let mut buffer = 0_u32;
    let mut bits = 0_u8;
    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => return Err("O conteúdo binário em base64 é inválido.".to_string()),
        };
        buffer = (buffer << 6) | u32::from(value);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            decoded.push((buffer >> bits) as u8);
        }
    }
    Ok(decoded)
}

/// Raiz da pasta `negodesign` dentro do projecto activo.
///
/// - **Svelte**: a pasta vive em `src/negodesign`, porque todos os imports
///   gerados são relativos a `src/` (ex.: `src/lib/features/...` →
///   `../../../negodesign`). Se existir uma instalação anterior na raiz do
///   projecto (`negodesign/`), é migrada para `src/negodesign`.
/// - **Restantes linguagens**: mantém-se na raiz do projecto.
pub fn package_root(project_root: &Path, language: &str) -> PathBuf {
    if language != "Svelte" {
        return project_root.join("negodesign");
    }

    let new_root = project_root.join("src").join("negodesign");
    let legacy_root = project_root.join("negodesign");
    if !new_root.exists() && legacy_root.is_dir() {
        if let Some(parent) = new_root.parent() {
            let _ = fs::create_dir_all(parent);
        }
        // Se a migração falhar (ficheiro em uso), os novos ficheiros são
        // na mesma escritos em src/negodesign — os imports continuam válidos.
        let _ = fs::rename(&legacy_root, &new_root);
    }
    new_root
}

pub fn write_project_files(
    project_root: &Path,
    language: &str,
    files: Vec<ComponentFile>,
) -> Result<usize, String> {
    let package_root = package_root(project_root, language);
    let public_root = match language {
        "Svelte" => project_root.join("static"),
        "Flutter" => project_root.join("assets"),
        _ => project_root.join("public"),
    };
    let mut copied = 0;
    let mut needs_flutter_icons = false;

    for file in files {
        let rel = validate_relative_path(&file.path)?;
        if language == "Flutter" && rel.starts_with("static/hugeicons/svg") {
            needs_flutter_icons = true;
        }
        let destination = if let Ok(asset_path) = rel.strip_prefix("static") {
            if language == "Flutter" && rel.starts_with("static/hugeicons/svg/") {
                let fname = asset_path
                    .file_name()
                    .ok_or_else(|| "O asset SVG não tem um nome de ficheiro.".to_string())?;
                public_root.join("icons").join(fname)
            } else {
                public_root.join(asset_path)
            }
        } else if rel.starts_with("project")
            || rel.starts_with("hooks")
            || rel.starts_with("components/local")
        {
            package_root.join(&rel)
        } else {
            return Err(format!(
                "O ficheiro '{}' não está numa localização permitida.",
                file.path
            ));
        };

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let bytes = match file.content.strip_prefix("base64:") {
            Some(encoded) => decode_base64(encoded)?,
            None => file.content.into_bytes(),
        };
        fs::write(destination, bytes).map_err(|e| e.to_string())?;
        copied += 1;
    }

    if needs_flutter_icons && ensure_flutter_icons(project_root)? {
        copied += 1;
    }
    Ok(copied)
}

fn ensure_flutter_icons(project_root: &Path) -> Result<bool, String> {
    let pubspec_path = project_root.join("pubspec.yaml");
    let source = fs::read_to_string(&pubspec_path).map_err(|e| e.to_string())?;
    if source.contains("assets/icons/") {
        return Ok(false);
    }
    let mut lines: Vec<String> = source.lines().map(str::to_owned).collect();
    if let Some(fl_idx) = lines.iter().position(|l| l.trim() == "flutter:") {
        let end = lines
            .iter()
            .enumerate()
            .skip(fl_idx + 1)
            .find(|(_, l)| !l.trim().is_empty() && !l.starts_with(char::is_whitespace))
            .map(|(i, _)| i)
            .unwrap_or(lines.len());
        if let Some(a_idx) = lines
            .iter()
            .enumerate()
            .take(end)
            .skip(fl_idx + 1)
            .find(|(_, l)| l.trim() == "assets:")
            .map(|(i, _)| i)
        {
            lines.insert(a_idx + 1, "    - assets/icons/".to_string());
        } else {
            lines.insert(end, "  assets:".to_string());
            lines.insert(end + 1, "    - assets/icons/".to_string());
        }
    } else {
        lines.extend([
            String::new(),
            "flutter:".to_string(),
            "  assets:".to_string(),
            "    - assets/icons/".to_string(),
        ]);
    }
    fs::write(pubspec_path, format!("{}\n", lines.join("\n"))).map_err(|e| e.to_string())?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_root_svelte_lives_in_src_and_migrates_legacy_copy() {
        let root = std::env::temp_dir().join(format!("negodesign-installer-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let legacy = root.join("negodesign");
        fs::create_dir_all(&legacy).expect("criar pasta legada");
        fs::write(legacy.join("marker.txt"), "x").expect("escrever marker");

        let resolved = package_root(&root, "Svelte");
        assert_eq!(resolved, root.join("src").join("negodesign"));
        assert!(
            resolved.join("marker.txt").exists(),
            "a cópia legada na raiz deve ser migrada para src/negodesign"
        );

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn package_root_other_languages_stay_at_project_root() {
        let root = std::env::temp_dir().join("negodesign-installer-other");
        assert_eq!(package_root(&root, "React"), root.join("negodesign"));
        assert_eq!(package_root(&root, "Vue"), root.join("negodesign"));
        assert_eq!(package_root(&root, "Nuxt"), root.join("negodesign"));
        assert_eq!(package_root(&root, "Flutter"), root.join("negodesign"));
    }
}
