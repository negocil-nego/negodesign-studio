use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DatabaseConnection {
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseColumn {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
}

#[derive(Debug, Serialize)]
pub struct DatabaseTable {
    pub name: String,
    pub columns: Vec<DatabaseColumn>,
}

#[derive(Debug, Serialize)]
pub struct OpenApiField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OpenApiEntity {
    pub name: String,
    pub kind: String,
    pub fields: Vec<OpenApiField>,
}
