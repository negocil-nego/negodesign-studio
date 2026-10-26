use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub language: String,
    pub active: bool,
}

#[derive(serde::Deserialize)]
pub struct ComponentFile {
    pub path: String,
    pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ComponentExport {
    pub id: String,
    pub name: String,
    pub entry: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentInstallResult {
    pub copied_files: usize,
    pub exported_components: usize,
}
