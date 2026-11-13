use crate::model::migration::OpenApiField;

pub(super) fn is_status_field(f: &OpenApiField) -> bool {
    let lower = f.name.to_lowercase();
    lower.contains("status")
        || lower.contains("state")
        || lower == "active"
        || lower == "enabled"
        || lower == "is_active"
}

pub(super) fn is_id_or_timestamp(f: &OpenApiField) -> bool {
    let lower = f.name.to_lowercase();
    lower == "id"
        || lower == "createdat"
        || lower == "updatedat"
        || lower.starts_with("created_")
        || lower.starts_with("updated_")
}

pub(super) fn infer_input_component(f: &OpenApiField) -> (&'static str, &'static str) {
    let name_lower = f.name.to_lowercase();
    let type_lower = f.field_type.to_lowercase();

    if name_lower.contains("email") || type_lower.contains("email") {
        ("input-email", "email")
    } else if name_lower.contains("password") {
        ("input-password", "password")
    } else if name_lower.contains("phone") || name_lower.contains("tel") {
        ("input-phone", "tel")
    } else if type_lower.contains("date") || type_lower.contains("time") {
        ("input-date", "date")
    } else if type_lower == "boolean" || type_lower == "bool" {
        ("toggle", "checkbox")
    } else if type_lower.contains("int")
        || type_lower == "number"
        || type_lower.contains("float")
        || type_lower.contains("double")
    {
        ("input-number", "number")
    } else if name_lower.contains("description")
        || name_lower.contains("note")
        || name_lower.contains("comment")
    {
        ("textarea", "text")
    } else {
        ("input-text", "text")
    }
}
