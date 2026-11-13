use super::fields::{infer_input_component, is_id_or_timestamp, is_status_field};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::{to_camel_case, to_kebab_case, to_title_label};
use crate::repository::openapi::crud_matcher::canonical_entity_stem;

pub(super) fn build_sidebar_items(active_entities: &[OpenApiEntity], current_kebab: &str) -> String {
    active_entities
        .iter()
        .map(|e| {
            let k = to_kebab_case(&e.name);
            let label = to_title_label(&e.name);
            let active = k == current_kebab;
            format!(
                "            {{ label: \"{label}\", href: \"/admin/{k}\", icon: \"Layout04\", active: {active} }}"
            )
        })
        .collect::<Vec<_>>()
        .join(",\n")
}

pub(super) fn build_columns_ts(entity: &OpenApiEntity) -> String {
    entity
        .fields
        .iter()
        .map(|f| {
            let label = to_title_label(&f.name);
            if is_status_field(f) {
                format!("  {{ text: \"{label}\", type: \"badge\" }}")
            } else {
                format!("  {{ text: \"{label}\", type: \"text\" }}")
            }
        })
        .chain(std::iter::once(
            "  { text: \"Actions\", type: \"dropdown\", actions: [\"Edit\", \"Delete\"] }"
                .to_string(),
        ))
        .collect::<Vec<_>>()
        .join(",\n")
}

pub(super) fn build_row_cells_ts(entity: &OpenApiEntity) -> String {
    entity
        .fields
        .iter()
        .map(|f| {
            let camel = to_camel_case(&f.name);
            let raw = &f.name;
            if is_status_field(f) {
                format!(
                    "      {{\n        text: String(item.{camel} ?? item[\"{raw}\"] ?? \"Active\"),\n        type: \"badge\",\n        badgeVariant: String(item.{camel} ?? item[\"{raw}\"]).toLowerCase().includes(\"inact\") || (item.{camel} === false) ? \"danger\" : \"success\",\n        dot: true,\n      }}"
                )
            } else {
                format!("      {{ text: String(item.{camel} ?? item[\"{raw}\"] ?? \"\") }}")
            }
        })
        .chain(std::iter::once(
            "      {\n        text: \"\",\n        type: \"dropdown\",\n        actions: [\n          { label: \"Edit\", value: \"edit\", icon: \"Edit02\" },\n          { label: \"Delete\", value: \"delete\", icon: \"Delete02\", variant: \"danger\" },\n        ],\n      }".to_string(),
        ))
        .collect::<Vec<_>>()
        .join(",\n")
}

pub(super) fn build_form_fields_ts(
    entity: &OpenApiEntity,
    all_entities: &[OpenApiEntity],
) -> String {
    let stem = canonical_entity_stem(&entity.name);
    let form_source = all_entities
        .iter()
        .find(|e| {
            (e.kind == "request" || e.kind == "dto")
                && canonical_entity_stem(&e.name) == stem
                && !e.fields.is_empty()
        })
        .unwrap_or(entity);

    form_source
        .fields
        .iter()
        .filter(|f| !is_id_or_timestamp(f))
        .map(|f| {
            let (comp, input_type) = infer_input_component(f);
            let label = to_title_label(&f.name);
            let name = to_camel_case(&f.name);
            let req = f.required;
            if comp.starts_with("input-") {
                format!(
                    "          {{\n            id: \"{name}\",\n            name: \"{name}\",\n            component: \"{comp}\",\n            labelProps: {{ label: \"{label}\", for: \"{name}\" }},\n            inputProps: {{ id: \"{name}\", name: \"{name}\", type: \"{input_type}\", required: {req}, placeholder: \"Enter {label}...\" }},\n          }}"
                )
            } else {
                format!(
                    "          {{\n            id: \"{name}\",\n            name: \"{name}\",\n            component: \"{comp}\",\n            label: \"{label}\",\n            required: {req},\n          }}"
                )
            }
        })
        .collect::<Vec<_>>()
        .join(",\n")
}
