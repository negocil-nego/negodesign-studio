use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub language: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentExport {
    pub id: String,
    pub name: String,
    pub entry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentInstallResult {
    pub copied_files: usize,
    pub exported_components: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentCopyRequest {
    pub component_id: String,
    pub manifest_path: Option<String>,
    pub locales: Option<Vec<String>>,
    pub theme: Option<String>,
    pub static_directory: Option<String>,
}
