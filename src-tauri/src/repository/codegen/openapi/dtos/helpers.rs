use crate::model::migration::OpenApiField;

pub fn strip_entity_action_or_kind(name: &str) -> &str {
    for suffix in ["Response", "Request", "Dto", "DTO", "Model", "Entity", "Schema"] {
        if name.len() > suffix.len() {
            if let Some(stripped) = name.strip_suffix(suffix) {
                return stripped;
            }
        }
    }
    name
}

pub fn map_field_type(field: &OpenApiField) -> &'static str {
    let lower = field.field_type.to_lowercase();
    if lower.contains("int")
        || lower == "number"
        || lower.contains("float")
        || lower.contains("double")
        || lower.contains("decimal")
    {
        "number"
    } else if lower == "boolean" || lower == "bool" {
        "boolean"
    } else if lower.contains("date") || lower.contains("time") {
        "string"
    } else if lower.ends_with("[]") || lower == "array" {
        "any[]"
    } else if lower == "object" {
        "Record<string, any>"
    } else {
        "string"
    }
}
