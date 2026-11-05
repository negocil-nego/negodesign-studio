use serde_json::Value;

use crate::model::migration::{OpenApiEntity, OpenApiField};

/// Lê a documentação Swagger/OpenAPI e extrai as entidades (response, dto, ...).
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

    let spec: Value = serde_json::from_str(&body).map_err(|_| {
        "A documentação não está em JSON. Indique uma rota que devolva OpenAPI/Swagger em JSON."
            .to_string()
    })?;

    let schemas = spec
        .get("components")
        .and_then(|value| value.get("schemas"))
        .or_else(|| spec.get("definitions"))
        .and_then(Value::as_object);

    let Some(schemas) = schemas else {
        return Ok(Vec::new());
    };

    let mut entities: Vec<OpenApiEntity> = schemas
        .iter()
        .map(|(name, schema)| OpenApiEntity {
            name: name.clone(),
            kind: kind_for(name).to_string(),
            fields: fields_for(schema),
        })
        .collect();
    entities.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(entities)
}

fn kind_for(name: &str) -> &'static str {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with("response") {
        "response"
    } else if lower.contains("dto") {
        "dto"
    } else if lower.ends_with("request") {
        "request"
    } else {
        "schema"
    }
}

fn fields_for(schema: &Value) -> Vec<OpenApiField> {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return Vec::new();
    };
    let required: Vec<&str> = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();

    properties
        .iter()
        .map(|(name, property)| OpenApiField {
            name: name.clone(),
            field_type: type_of(property),
            required: required.contains(&name.as_str()),
            description: property
                .get("description")
                .and_then(Value::as_str)
                .map(str::to_string),
        })
        .collect()
}

fn type_of(property: &Value) -> String {
    if let Some(reference) = property.get("$ref").and_then(Value::as_str) {
        return reference.rsplit('/').next().unwrap_or(reference).to_string();
    }
    let Some(kind) = property.get("type").and_then(Value::as_str) else {
        return "any".to_string();
    };
    if kind == "array" {
        let items = property
            .get("items")
            .map(type_of)
            .unwrap_or_else(|| "any".to_string());
        return format!("{items}[]");
    }
    match property.get("format").and_then(Value::as_str) {
        Some(format) => format!("{kind} ({format})"),
        None => kind.to_string(),
    }
}
