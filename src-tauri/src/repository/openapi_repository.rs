use crate::{
    model::migration::OpenApiEntity,
    repository::openapi::parser::{fields_for, kind_for, operations_for, operations_for_schema},
};
use serde_json::Value;
use std::fs;

pub fn extract(url: &str) -> Result<Vec<OpenApiEntity>, String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("Indique uma rota válida (http:// ou https://).".to_string());
    }

    let mut response = ureq::get(url)
        .call()
        .map_err(|error| format!("Não foi possível ler a documentação: {error}"))?;
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|error| error.to_string())?;

    extract_from_document(&body)
}

pub fn extract_file(path: &str) -> Result<Vec<OpenApiEntity>, String> {
    let body = fs::read_to_string(path)
        .map_err(|error| format!("Não foi possível ler o ficheiro: {error}"))?;
    extract_from_document(&body)
}

fn extract_from_document(body: &str) -> Result<Vec<OpenApiEntity>, String> {
    let spec: Value = serde_yaml::from_str(body)
        .map_err(|_| "O documento não é um OpenAPI válido em JSON ou YAML.".to_string())?;

    let schemas = spec
        .get("components")
        .and_then(|v| v.get("schemas"))
        .or_else(|| spec.get("definitions"))
        .and_then(Value::as_object);

    let Some(schemas) = schemas else {
        return Ok(Vec::new());
    };

    let operations = operations_for(&spec);
    let server_url = spec
        .get("servers")
        .and_then(Value::as_array)
        .and_then(|s| s.first())
        .and_then(|s| s.get("url"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            let host = spec.get("host")?.as_str()?;
            let scheme = spec
                .get("schemes")
                .and_then(Value::as_array)
                .and_then(|s| s.first())
                .and_then(Value::as_str)
                .unwrap_or("https");
            let base = spec.get("basePath").and_then(Value::as_str).unwrap_or("");
            Some(format!("{scheme}://{host}{base}"))
        });

    let mut entities: Vec<OpenApiEntity> = schemas
        .iter()
        .map(|(name, schema)| OpenApiEntity {
            name: name.clone(),
            kind: kind_for(name).to_string(),
            fields: fields_for(schema),
            operations: operations_for_schema(name, &operations),
            server_url: server_url.clone(),
        })
        .collect();
    entities.sort_by(|l, r| l.name.cmp(&r.name));
    Ok(entities)
}
