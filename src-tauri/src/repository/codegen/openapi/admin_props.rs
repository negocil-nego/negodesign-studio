use crate::model::migration::{OpenApiEntity, OpenApiField};
use crate::repository::codegen::naming::{
    to_camel_case, to_kebab_case, to_pascal_case, to_title_label,
};
use crate::repository::openapi::crud_matcher::canonical_entity_stem;

pub fn build_openapi_admin_config_ts(
    entity: &OpenApiEntity,
    active_entities: &[OpenApiEntity],
    all_entities: &[OpenApiEntity],
) -> String {
    let pascal = to_pascal_case(&entity.name);
    let current_kebab = to_kebab_case(&entity.name);
    let title = to_title_label(&entity.name);
    let upper_title = title.to_ascii_uppercase();

    let sidebar_items = active_entities
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
        .join(",\n");

    let columns_ts = entity
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
        .join(",\n");

    let row_cells_ts = entity
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
        .join(",\n");

    let stem = canonical_entity_stem(&entity.name);
    let form_source = all_entities
        .iter()
        .find(|e| {
            (e.kind == "request" || e.kind == "dto")
                && canonical_entity_stem(&e.name) == stem
                && !e.fields.is_empty()
        })
        .unwrap_or(entity);

    let form_fields_ts = form_source
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
        .join(",\n");

    format!(
        "import type {{ DataTextProps }} from \"../../negodesign/project/ui/organisms/data-table/data/types\";\n\
import type {{ AdminProps }} from \"../../negodesign/project/ui/pages/admin/data/types\";\n\n\
export const {pascal}Columns: DataTextProps[] = [\n{columns_ts}\n];\n\n\
export function map{pascal}Rows(records: Record<string, any>[] = []): DataTextProps[] {{\n\
  return records.flatMap((item) => [\n{row_cells_ts}\n  ]);\n\
}}\n\n\
export function create{pascal}AdminProps(\n\
  records: Record<string, any>[] = [],\n\
  handlers?: {{\n\
    onSubmit?: (values?: Record<string, unknown>) => void;\n\
    onRowAction?: (action: string, rowIndex: number, row: DataTextProps[]) => void;\n\
  }}\n\
): AdminProps {{\n\
  return {{\n\
    title: \"{title}\",\n\
    breadcrumb: {{\n\
      items: [\n\
        {{ label: \"Admin Dashboard\", href: \"/admin\" }},\n\
        {{ label: \"OpenAPI\", href: \"/admin/{current_kebab}\" }},\n\
        {{ label: \"{title}\", active: true }},\n\
      ],\n\
    }},\n\
    addButtonLabel: \"ADD {upper_title}\",\n\
    sidebarProps: {{\n\
      variant: \"01\",\n\
      theme: \"dark\",\n\
      brand: \"Admin Panel\",\n\
      sections: [\n\
        {{\n\
          title: \"OpenAPI Resources\",\n\
          items: [\n{sidebar_items}\n          ],\n\
        }},\n\
      ],\n\
    }},\n\
    menuProps: {{\n\
      style: \"default\",\n\
      logoText: \"{title} Management\",\n\
      menus: [\n\
        {{ textProps: {{ text: \"Dashboard\" }}, href: \"/admin/{current_kebab}\" }},\n\
      ],\n\
    }},\n\
    datatableProps: {{\n\
      variant: \"01\",\n\
      title: \"{title}\",\n\
      columns: {pascal}Columns,\n\
      rows: map{pascal}Rows(records),\n\
      addLabel: \"ADD {upper_title}\",\n\
      searchPlaceholder: \"Search {title}...\",\n\
      onRowAction: handlers?.onRowAction,\n\
    }},\n\
    modalFormProps: {{\n\
      title: \"{title}\",\n\
      form: {{\n\
        style: \"grid\",\n\
        submitLabel: \"Save {title}\",\n\
        fields: [\n{form_fields_ts}\n        ],\n\
        onSubmit: handlers?.onSubmit,\n\
      }},\n\
    }},\n\
  }};\n\
}}\n"
    )
}

fn is_status_field(f: &OpenApiField) -> bool {
    let lower = f.name.to_lowercase();
    lower.contains("status")
        || lower.contains("state")
        || lower == "active"
        || lower == "enabled"
        || lower == "is_active"
}

fn is_id_or_timestamp(f: &OpenApiField) -> bool {
    let lower = f.name.to_lowercase();
    lower == "id"
        || lower == "createdat"
        || lower == "updatedat"
        || lower.starts_with("created_")
        || lower.starts_with("updated_")
}

fn infer_input_component(f: &OpenApiField) -> (&'static str, &'static str) {
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
