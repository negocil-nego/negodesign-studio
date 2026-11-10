use crate::model::migration::{OpenApiField, OpenApiOperation};
use serde_json::Value;

pub fn operations_for(spec: &Value) -> Vec<(Vec<String>, OpenApiOperation)> {
    let Some(paths) = spec.get("paths").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut result = Vec::new();
    for (path, path_item) in paths {
        let Some(methods) = path_item.as_object() else {
            continue;
        };
        for method in ["get", "post", "put", "patch", "delete"] {
            let Some(operation) = methods.get(method) else {
                continue;
            };
            let mut refs = operation
                .get("requestBody")
                .and_then(|body| body.pointer("/content/application~1json/schema"))
                .and_then(schema_refs)
                .unwrap_or_default();
            if let Some(responses) = operation.get("responses").and_then(Value::as_object) {
                for response in responses.values() {
                    if let Some(schema) = response.pointer("/content/application~1json/schema") {
                        refs.extend(schema_refs(schema).unwrap_or_default());
                    }
                }
            }
            result.push((
                refs,
                OpenApiOperation {
                    method: method.to_uppercase(),
                    path: path.clone(),
                    operation_id: operation
                        .get("operationId")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                    summary: operation
                        .get("summary")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                },
            ));
        }
    }
    result
}

fn schema_refs(schema: &Value) -> Option<Vec<String>> {
    let mut refs = Vec::new();
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        refs.push(reference.rsplit('/').next().unwrap_or(reference).to_string());
    }
    for key in ["items", "allOf", "oneOf", "anyOf"] {
        match schema.get(key) {
            Some(Value::Array(items)) => {
                for item in items {
                    refs.extend(schema_refs(item).unwrap_or_default());
                }
            }
            Some(item) => refs.extend(schema_refs(item).unwrap_or_default()),
            None => (),
        }
    }
    Some(refs)
}

pub fn operations_for_schema(
    name: &str,
    operations: &[(Vec<String>, OpenApiOperation)],
) -> Vec<OpenApiOperation> {
    operations
        .iter()
        .filter(|(refs, _)| refs.iter().any(|r| r == name))
        .map(|(_, op)| op.clone())
        .collect()
}

pub fn kind_for(name: &str) -> &'static str {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with("response") || lower.starts_with("response") {
        "response"
    } else if lower.contains("dto") || lower.ends_with("model") {
        "dto"
    } else if lower.ends_with("request") || lower.starts_with("request") {
        "request"
    } else {
        "schema"
    }
}

pub fn fields_for(schema: &Value) -> Vec<OpenApiField> {
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

