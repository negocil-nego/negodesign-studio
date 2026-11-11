use crate::model::migration::DatabaseTable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ArchitectureMode {
    Split,
    Monolith,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum FrontendFramework {
    Sveltekit,
    Nuxt,
    Nextjs,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BackendFramework {
    SpringBoot,
    Nestjs,
    Laravel,
    AspnetCore,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ApiCallPattern {
    Bff,
    Direct,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HttpClientKind {
    Native,
    Axios,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DatabaseEngine {
    Postgres,
    Mysql,
    Sqlite,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullstackCodegenRequest {
    pub architecture_mode: ArchitectureMode,
    pub tables: Vec<DatabaseTable>,
    pub frontend_path: String,
    pub frontend_framework: FrontendFramework,
    pub api_call_pattern: ApiCallPattern,
    pub http_client: HttpClientKind,
    pub use_tanstack_query: bool,
    pub backend_path: Option<String>,
    pub backend_framework: Option<BackendFramework>,
    pub orm_strategy: Option<String>,
    pub base_package: Option<String>,
    pub database_engine: DatabaseEngine,
    pub backend_base_url: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullstackCodegenResult {
    pub generated_backend_files: usize,
    pub generated_frontend_files: usize,
    pub copied_ui_files: usize,
    pub tables_processed: usize,
    pub summary: Vec<String>,
}

