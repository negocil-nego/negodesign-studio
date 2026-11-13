use std::{fs, path::Path};

/// Detecta o pacote base de um projecto Spring Boot através de:
/// 1. Anotação `@SpringBootApplication` em ficheiros .java ou .kt
/// 2. Definições no `pom.xml` (<groupId> e <artifactId> após <parent>)
/// 3. Declarações em `build.gradle` ou `build.gradle.kts`
/// 4. Qualquer ficheiro Java/Kotlin existente sob `src/main/java` ou `src/main/kotlin`
pub fn detect_spring_base_package(project_root: &Path) -> Option<String> {
    if let Some(pkg) = scan_main_application_package(project_root) {
        return Some(pkg);
    }

    let pom_path = project_root.join("pom.xml");
    if pom_path.is_file() {
        if let Ok(content) = fs::read_to_string(&pom_path) {
            if let Some(pkg) = extract_pom_package(&content) {
                return Some(pkg);
            }
        }
    }

    for gradle_file in ["build.gradle", "build.gradle.kts"] {
        let path = project_root.join(gradle_file);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(pkg) = extract_gradle_group(&content) {
                    return Some(pkg);
                }
            }
        }
    }

    scan_any_package(project_root)
}

fn scan_main_application_package(root: &Path) -> Option<String> {
    for src_dir in ["src/main/java", "src/main/kotlin"] {
        let dir = root.join(src_dir);
        if dir.is_dir() {
            if let Some(pkg) = scan_dir_for_spring_boot_app(&dir) {
                return Some(pkg);
            }
        }
    }
    None
}

fn scan_dir_for_spring_boot_app(dir: &Path) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(pkg) = scan_dir_for_spring_boot_app(&path) {
                return Some(pkg);
            }
        } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext == "java" || ext == "kt" {
                if let Ok(content) = fs::read_to_string(&path) {
                    if content.contains("@SpringBootApplication") {
                        if let Some(pkg) = extract_package_statement(&content) {
                            return Some(pkg);
                        }
                    }
                }
            }
        }
    }
    None
}

fn extract_package_statement(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("package ") {
            let without_kw = trimmed.trim_start_matches("package ").trim();
            let clean = without_kw.trim_end_matches(';').trim();
            if !clean.is_empty() {
                return Some(clean.to_string());
            }
        }
    }
    None
}

fn extract_pom_package(pom_xml: &str) -> Option<String> {
    // Procura o groupId e artifactId após a secção </parent> para não pegar org.springframework.boot
    let content_after_parent = match pom_xml.split_once("</parent>") {
        Some((_, rest)) => rest,
        None => pom_xml,
    };

    let group_id = extract_xml_tag(content_after_parent, "groupId")
        .or_else(|| extract_xml_tag(pom_xml, "groupId"))?;
    let artifact_id = extract_xml_tag(content_after_parent, "artifactId")
        .or_else(|| extract_xml_tag(pom_xml, "artifactId"));

    if group_id == "org.springframework.boot" {
        return None;
    }

    if let Some(art) = artifact_id {
        let clean_art = art.replace('-', "").replace('_', "").to_lowercase();
        if !clean_art.is_empty() && !group_id.ends_with(&clean_art) && !group_id.contains(&format!(".{clean_art}")) {
            return Some(format!("{group_id}.{clean_art}"));
        }
    }

    Some(group_id)
}

fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let open_tag = format!("<{tag}>");
    let close_tag = format!("</{tag}>");
    let (_, after) = xml.split_once(&open_tag)?;
    let (val, _) = after.split_once(&close_tag)?;
    let trimmed = val.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn extract_gradle_group(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("group") {
            let parts: Vec<&str> = trimmed.split('=').collect();
            if parts.len() == 2 {
                let clean = parts[1].trim().trim_matches('"').trim_matches('\'').trim();
                if !clean.is_empty() && clean != "org.springframework.boot" {
                    return Some(clean.to_string());
                }
            }
        }
    }
    None
}

fn scan_any_package(root: &Path) -> Option<String> {
    for src_dir in ["src/main/java", "src/main/kotlin"] {
        let dir = root.join(src_dir);
        if dir.is_dir() {
            if let Some(pkg) = scan_dir_for_any_package(&dir) {
                return Some(pkg);
            }
        }
    }
    None
}

fn scan_dir_for_any_package(dir: &Path) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(pkg) = scan_dir_for_any_package(&path) {
                return Some(pkg);
            }
        } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext == "java" || ext == "kt" {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(pkg) = extract_package_statement(&content) {
                        // Limpa sufixos conhecidos de sub-pacotes
                        let root_pkg = pkg
                            .split(".feature.")
                            .next()
                            .unwrap_or(&pkg)
                            .split(".controller")
                            .next()
                            .unwrap_or(&pkg)
                            .split(".service")
                            .next()
                            .unwrap_or(&pkg)
                            .split(".repository")
                            .next()
                            .unwrap_or(&pkg)
                            .split(".dto")
                            .next()
                            .unwrap_or(&pkg)
                            .split(".entity")
                            .next()
                            .unwrap_or(&pkg);
                        return Some(root_pkg.to_string());
                    }
                }
            }
        }
    }
    None
}

