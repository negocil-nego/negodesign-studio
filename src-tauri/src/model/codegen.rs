use crate::model::migration::{DatabaseTable, OpenApiEntity};
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SpringLanguage {
    Java,
    Kotlin,
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
    pub spring_language: Option<SpringLanguage>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiEndpointSelection {
    pub entity_name: String,
    pub method: String,
    pub path: String,
    pub operation_id: Option<String>,
    pub summary: Option<String>,
    pub kind: String,
    #[serde(default)]
    pub crud_role: Option<String>,
    #[serde(default)]
    pub pattern_style: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiFrontendCodegenRequest {
    pub entities: Vec<OpenApiEntity>,
    pub selected_endpoints: Vec<OpenApiEndpointSelection>,
    pub frontend_path: String,
    pub frontend_framework: FrontendFramework,
    pub api_call_pattern: ApiCallPattern,
    pub http_client: HttpClientKind,
    pub use_tanstack_query: bool,
    pub backend_base_url: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiFrontendCodegenResult {
    pub generated_frontend_files: usize,
    pub copied_ui_files: usize,
    pub endpoints_processed: usize,
    pub summary: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodegenPath {
    pub id: i64,
    pub target: String,
    pub path: String,
    pub label: Option<String>,
    pub last_used: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpringBootProjectRequest {
    pub target_dir: String,
    pub group_id: String,
    pub artifact_id: String,
    pub name: String,
    pub description: String,
    pub language: SpringLanguage,
    pub orm_strategy: String,
    pub database_engine: DatabaseEngine,
    pub include_api_docs: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpringBootProjectResult {
    pub project_dir: String,
    pub build_tool: String,
    pub spring_boot_version: String,
    pub generated_files: usize,
    pub dependencies: Vec<String>,
    pub next_steps: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NestjsProjectRequest {
    pub target_dir: String,
    pub name: String,
    pub description: String,
    pub orm_strategy: String,
    pub database_engine: DatabaseEngine,
    pub include_api_docs: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendProjectResult {
    pub project_dir: String,
    pub build_tool: String,
    pub generated_files: usize,
    pub dependencies: Vec<String>,
    pub next_steps: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaravelProjectRequest {
    pub target_dir: String,
    pub name: String,
    pub description: String,
    pub orm_strategy: String,
    pub database_engine: DatabaseEngine,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AspnetProjectRequest {
    pub target_dir: String,
    pub name: String,
    pub description: String,
    pub orm_strategy: String,
    pub database_engine: DatabaseEngine,
    pub include_api_docs: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct FrontendProjectRequest {
    pub target_dir: String,
    pub name: String,
    pub description: String,
    pub framework: FrontendFramework,
    pub api_call_pattern: ApiCallPattern,
    pub http_client: HttpClientKind,
    pub use_tanstack_query: bool,
    #[serde(default = "default_true")]
    pub include_tailwind: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendProjectResult {
    pub project_dir: String,
    pub framework: String,
    pub build_tool: String,
    pub generated_files: usize,
    pub dependencies: Vec<String>,
    pub next_steps: Vec<String>,
}

