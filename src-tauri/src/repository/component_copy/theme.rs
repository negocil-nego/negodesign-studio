use super::assets::{add_asset_pattern, add_source_file};
use std::{collections::BTreeMap, fs, path::Path};

const COMPONENT_CSS_IMPORTS: &str = "\
@import \"./button.css\";
@import \"./badge.css\";
@import \"./input.css\";
@import \"./form.css\";
@import \"./select.css\";
@import \"./card.css\";
@import \"./table.css\";
@import \"./menu.css\";
@import \"./sidebar.css\";
@import \"./modal.css\";
@import \"./accordion.css\";
@import \"./carousel.css\";
@import \"./nav.css\";
@import \"./alert.css\";
@import \"./progress.css\";
@import \"./list-group.css\";
@import \"./container.css\";";

fn extract_bootswatch_theme_block(root: &Path, theme_id: &str) -> Option<String> {
    let target_marker = format!("[data-bootswatch=\"{theme_id}\"]");
    for file_name in ["themes-a-m.css", "themes-n-z.css"] {
        let css_path = root.join("src/project/data/styles/themes").join(file_name);
        if let Ok(content) = fs::read_to_string(css_path) {
            for line in content.lines() {
                if line.contains(&target_marker) {
                    if let Some(brace_start) = line.find('{') {
                        let body = &line[brace_start..];
                        return Some(format!(
                            "/* Bootswatch Theme: {theme_id} */\n:root, [data-bootswatch=\"{theme_id}\"] {body}\n"
                        ));
                    }
                }
            }
        }
    }
    None
}

pub fn attach_theme_and_styles(
    root: &Path,
    files_by_path: &mut BTreeMap<String, String>,
    manifest_lang: &str,
    selected_theme: &str,
) -> Result<(), String> {
    if manifest_lang == "Flutter" {
        add_asset_pattern(
            root,
            "project/data/theme/dart/*.dart",
            files_by_path,
            manifest_lang,
        )?;
        if selected_theme != "default" && selected_theme != "all" {
            let dart_key = "project/data/theme/dart/theme_data.dart";
            if let Some(existing) = files_by_path.get(dart_key).cloned() {
                files_by_path.insert(
                    dart_key.to_string(),
                    existing.replace(
                        "static ThemeData toMaterialTheme([String? bootswatchId])",
                        &format!(
                            "static ThemeData toMaterialTheme([String? bootswatchId = '{selected_theme}'])"
                        ),
                    ),
                );
            }
        }
        return Ok(());
    }

    add_source_file(
        root,
        "project/data/style.component.ts",
        files_by_path,
        manifest_lang,
        true,
    )?;
    add_asset_pattern(
        root,
        "project/data/theme/ts/*.ts",
        files_by_path,
        manifest_lang,
    )?;
    add_asset_pattern(
        root,
        "project/data/styles/*.css",
        files_by_path,
        manifest_lang,
    )?;

    if selected_theme == "all" {
        add_asset_pattern(
            root,
            "project/data/styles/themes/*.css",
            files_by_path,
            manifest_lang,
        )?;
    } else if selected_theme == "default" {
        files_by_path.insert(
            "project/data/styles/bootswatch.css".to_string(),
            format!(
                "/* NegoDesign Default Blender Theme + Tailwind @layer components */\n{COMPONENT_CSS_IMPORTS}\n"
            ),
        );
    } else if let Some(single_css) = extract_bootswatch_theme_block(root, selected_theme) {
        files_by_path.insert(
            format!("project/data/styles/themes/{selected_theme}.css"),
            single_css,
        );
        files_by_path.insert(
            "project/data/styles/bootswatch.css".to_string(),
            format!(
                "/* NegoDesign Bootswatch ({selected_theme}) + Tailwind @layer components */\n@import \"./themes/{selected_theme}.css\";\n{COMPONENT_CSS_IMPORTS}\n"
            ),
        );
    }

    Ok(())
}
