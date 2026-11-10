use super::{
    assets::{add_asset_pattern, add_source_file, build_translations_barrel},
    manifest::ManifestCatalog,
    paths::normalize_logical_path,
};
use crate::model::project::ComponentExport;
use std::{
    collections::{BTreeMap, HashSet},
    path::Path,
};

pub struct CopyCollector<'a> {
    pub root: &'a Path,
    pub catalog: &'a ManifestCatalog,
    pub manifest_lang: &'a str,
    pub locales: &'a [String],
    pub visited: HashSet<String>,
    pub files_by_path: BTreeMap<String, String>,
    pub exported_by_id: BTreeMap<String, ComponentExport>,
}

impl<'a> CopyCollector<'a> {
    pub fn new(
        root: &'a Path,
        catalog: &'a ManifestCatalog,
        manifest_lang: &'a str,
        locales: &'a [String],
    ) -> Self {
        Self {
            root,
            catalog,
            manifest_lang,
            locales,
            visited: HashSet::new(),
            files_by_path: BTreeMap::new(),
            exported_by_id: BTreeMap::new(),
        }
    }

    pub fn collect(&mut self, id: &str, manifest_hint: Option<&str>) -> Result<(), String> {
        if self.visited.contains(id) {
            return Ok(());
        }

        let (component, directory) = self.catalog.resolve(id, manifest_hint)?;
        let entry = component.entry.get(self.manifest_lang).ok_or_else(|| {
            format!(
                "{} não tem uma implementação para {}.",
                component.name, self.manifest_lang
            )
        })?;

        self.visited.insert(id.to_string());
        let entry_logical = normalize_logical_path(&format!("{directory}/{entry}"));
        add_source_file(
            self.root,
            &entry_logical,
            &mut self.files_by_path,
            self.manifest_lang,
            false,
        )?;

        if let Some(extra_files) = component.files.get(self.manifest_lang) {
            for file in extra_files {
                let file_logical = normalize_logical_path(&format!("{directory}/{file}"));
                add_source_file(
                    self.root,
                    &file_logical,
                    &mut self.files_by_path,
                    self.manifest_lang,
                    false,
                )?;
            }
        }

        for asset in component.assets.for_language(self.manifest_lang) {
            let normalized_asset = normalize_logical_path(&asset);
            if let Some(rest) = normalized_asset.strip_prefix("project/data/i18n/langs/") {
                if let Some(locale_code) = rest.strip_suffix(".js") {
                    if !self.locales.iter().any(|l| l == locale_code) {
                        continue;
                    }
                }
            }
            if normalized_asset == "project/data/i18n/translations.js" {
                let barrel = build_translations_barrel(self.locales)?;
                self.files_by_path.insert(normalized_asset, barrel);
                continue;
            }
            add_asset_pattern(
                self.root,
                &asset,
                &mut self.files_by_path,
                self.manifest_lang,
            )?;
        }

        self.exported_by_id.insert(
            id.to_string(),
            ComponentExport {
                id: component.id.clone(),
                name: component.name.clone(),
                entry: entry_logical,
            },
        );

        for dep in component.dependencies.for_language(self.manifest_lang) {
            self.collect(&dep, None)?;
        }

        Ok(())
    }
}

