use crate::model::migration::{OpenApiEntity, OpenApiField};
use crate::repository::codegen::naming::{to_camel_case, to_pascal_case};
use crate::repository::openapi::crud_matcher::canonical_entity_stem;
use std::collections::HashSet;

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

pub fn generate_dtos_ts(entities: &[OpenApiEntity]) -> String {
    let mut out = String::new();
    out.push_str("/**\n * Types and DTOs generated from OpenAPI specification\n */\n\n");

    out.push_str("export interface ApiResponse<T> {\n");
    out.push_str("  data: T;\n");
    out.push_str("  message?: string;\n");
    out.push_str("  status?: number;\n");
    out.push_str("}\n\n");

    out.push_str("export interface PaginatedResponse<T> {\n");
    out.push_str("  data?: T[];\n");
    out.push_str("  items?: T[];\n");
    out.push_str("  total: number;\n");
    out.push_str("  page?: number;\n");
    out.push_str("  size?: number;\n");
    out.push_str("  totalPages?: number;\n");
    out.push_str("}\n\n");

    let mut emitted_interfaces = HashSet::new();

    for entity in entities {
        let pascal = to_pascal_case(&entity.name);
        if emitted_interfaces.insert(pascal.clone()) {
            out.push_str(&format!("export interface {pascal} {{\n"));
            for field in &entity.fields {
                let camel = to_camel_case(&field.name);
                let ts_type = map_field_type(field);
                let opt = if field.required { "" } else { "?" };
                if let Some(desc) = &field.description {
                    out.push_str(&format!("  /** {desc} */\n"));
                }
                out.push_str(&format!("  {camel}{opt}: {ts_type};\n"));
            }
            out.push_str("}\n\n");
        }

        let base_name = strip_entity_action_or_kind(&entity.name);
        let base_pascal = to_pascal_case(base_name);
        let stem = canonical_entity_stem(&entity.name);

        let request_source = entities
            .iter()
            .find(|e| {
                (e.kind == "request" || e.kind == "dto")
                    && canonical_entity_stem(&e.name) == stem
                    && !e.fields.is_empty()
            })
            .unwrap_or(entity);

        let create_dto_name = format!("Create{base_pascal}Dto");
        if emitted_interfaces.insert(create_dto_name.clone()) {
            out.push_str(&format!("export interface {create_dto_name} {{\n"));
            for field in &request_source.fields {
                let lower = field.name.to_lowercase();
                if lower == "id"
                    || lower == "createdat"
                    || lower == "updatedat"
                    || lower.starts_with("created_")
                    || lower.starts_with("updated_")
                {
                    continue;
                }
                let camel = to_camel_case(&field.name);
                let ts_type = map_field_type(field);
                let opt = if field.required { "" } else { "?" };
                out.push_str(&format!("  {camel}{opt}: {ts_type};\n"));
            }
            out.push_str("}\n\n");
        }

        let update_dto_name = format!("Update{base_pascal}Dto");
        if emitted_interfaces.insert(update_dto_name.clone()) {
            out.push_str(&format!(
                "export interface {update_dto_name} extends Partial<{create_dto_name}> {{\n"
            ));
            out.push_str("  id?: string | number;\n");
            out.push_str("}\n\n");
        }
    }

    out
}

fn map_field_type(field: &OpenApiField) -> &'static str {
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
