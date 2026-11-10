use super::paths::normalize_logical_path;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path, sync::OnceLock};

static CATALOG_CACHE: OnceLock<ManifestCatalog> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum LanguageScopedList {
    All(Vec<String>),
    PerLanguage(HashMap<String, Vec<String>>),
}

impl Default for LanguageScopedList {
    fn default() -> Self {
        Self::All(Vec::new())
    }
}

impl LanguageScopedList {
    pub fn for_language(&self, lang: &str) -> Vec<String> {
        match self {
            Self::All(list) => list.clone(),
            Self::PerLanguage(map) => map.get(lang).cloned().unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestComponent {
    pub id: String,
    pub name: String,
    pub entry: HashMap<String, String>,
    #[serde(default)]
    pub files: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub dependencies: LanguageScopedList,
    #[serde(default)]
    pub assets: LanguageScopedList,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ComponentManifest {
    #[serde(default)]
    pub components: Vec<ManifestComponent>,
}

#[derive(Debug, Clone)]
pub struct IndexedManifest {
    pub directory: String,
    pub components: Vec<ManifestComponent>,
}

pub struct ManifestCatalog {
    pub by_path: HashMap<String, IndexedManifest>,
    pub by_id: HashMap<String, (ManifestComponent, String)>,
}

impl ManifestCatalog {
    pub fn global(src_root: &Path) -> Result<&'static Self, String> {
        if let Some(cached) = CATALOG_CACHE.get() {
            return Ok(cached);
        }
        let loaded = Self::load(src_root)?;
        let _ = CATALOG_CACHE.set(loaded);
        CATALOG_CACHE
            .get()
            .ok_or_else(|| "Falha ao inicializar a cache de manifestos.".to_string())
    }

    pub fn load(src_root: &Path) -> Result<Self, String> {
        let ui_root = src_root.join("project/ui");
        let mut by_path = HashMap::new();
        let mut by_id = HashMap::new();
        Self::scan_dir(&ui_root, src_root, &mut by_path, &mut by_id)?;
        Ok(Self { by_path, by_id })
    }

    pub fn resolve(
        &self,
        id: &str,
        manifest_hint: Option<&str>,
    ) -> Result<(ManifestComponent, String), String> {
        let from_hint = manifest_hint.and_then(|hint| {
            let clean = normalize_logical_path(hint);
            let rel = clean.strip_prefix("src/").unwrap_or(&clean);
            self.by_path.get(rel).and_then(|m| {
                m.components
                    .iter()
                    .find(|c| c.id == id)
                    .map(|c| (c.clone(), m.directory.clone()))
            })
        });
        from_hint
            .or_else(|| self.by_id.get(id).cloned())
            .ok_or_else(|| format!("O componente dependente '{id}' não existe no catálogo."))
    }

    fn scan_dir(
        dir: &Path,
        src_root: &Path,
        by_path: &mut HashMap<String, IndexedManifest>,
        by_id: &mut HashMap<String, (ManifestComponent, String)>,
    ) -> Result<(), String> {
        for entry in fs::read_dir(dir).map_err(|e| e.to_string())?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::scan_dir(&path, src_root, by_path, by_id)?;
            } else if path.file_name().and_then(|s| s.to_str()) == Some("dependence.json") {
                let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                if let Ok(parsed) = serde_json::from_str::<ComponentManifest>(&content) {
                    let rel = path
                        .strip_prefix(src_root)
                        .map_err(|e| e.to_string())?
                        .to_string_lossy()
                        .replace('\\', "/");
                    let directory = rel
                        .strip_suffix("/dependence.json")
                        .unwrap_or(&rel)
                        .to_string();
                    for comp in &parsed.components {
                        by_id.insert(comp.id.clone(), (comp.clone(), directory.clone()));
                    }
                    by_path.insert(
                        rel,
                        IndexedManifest {
                            directory,
                            components: parsed.components,
                        },
                    );
                }
            }
        }
        Ok(())
    }
}
