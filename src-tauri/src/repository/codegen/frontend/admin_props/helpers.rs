use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{
        editable_columns, infer_form_component, is_badge_column, to_camel_case, to_kebab_case,
        to_title_label,
    },
};

pub fn is_audit_column(name: &str) -> bool {
    let lower = name.to_ascii_lowercase().replace(['_', '-', ' '], "");
    matches!(
        lower.as_str(),
        "createdat" | "createdby" | "updatedat" | "updatedby" | "deletedat" | "deletedby"
    )
}

pub fn build_admin_sidebar_ts(all_tables: &[DatabaseTable]) -> String {
    let items = all_tables
        .iter()
        .map(|t| {
            let k = to_kebab_case(&t.name);
            let label = to_title_label(&t.name);
            format!("  {{ label: \"{label}\", href: \"/admin/{k}\", icon: \"Layout04\" }}")
        })
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        r#"export interface AdminSidebarItem {{
  label: string;
  href: string;
  icon: string;
  active?: boolean;
}}

export interface AdminSidebarSection {{
  title: string;
  items: AdminSidebarItem[];
}}

export const adminSidebarItems: AdminSidebarItem[] = [
{items}
];

export function getAdminSidebarSections(activeKebab?: string): AdminSidebarSection[] {{
  const currentPath = activeKebab ? `/admin/${{activeKebab}}` : "";
  return [
    {{
      title: "Database Tables",
      items: adminSidebarItems.map((item) => ({{
        ...item,
        active: item.href === currentPath,
      }})),
    }},
  ];
}}

export function getAdminSidebarProps(activeKebab?: string) {{
  return {{
    variant: "01" as const,
    theme: "dark" as const,
    brand: "Admin Panel",
    sections: getAdminSidebarSections(activeKebab),
  }};
}}
"#
    )
}

pub(super) fn build_columns_ts(table: &DatabaseTable) -> String {
    table
        .columns
        .iter()
        .filter(|c| !is_audit_column(&c.name))
        .map(|c| {
            let label = to_title_label(&c.name);
            if is_badge_column(c) {
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

pub(super) fn build_row_cells_ts(table: &DatabaseTable) -> String {
    table
        .columns
        .iter()
        .filter(|c| !is_audit_column(&c.name))
        .map(|c| {
            let camel = to_camel_case(&c.name);
            let raw = &c.name;
            if is_badge_column(c) {
                format!(
                    "      {{\n        text: String(item.{camel} ?? item[\"{raw}\"] ?? \"Active\"),\n        type: \"badge\",\n        badgeVariant: (item.{camel} ?? item[\"{raw}\"]) === false ? \"danger\" : \"success\",\n        dot: true,\n      }}"
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

pub(super) fn build_form_fields_ts(table: &DatabaseTable) -> String {
    editable_columns(table)
        .iter()
        .filter(|c| !is_audit_column(&c.name))
        .map(|c| {
            let (comp, input_type) = infer_form_component(c);
            let label = to_title_label(&c.name);
            let name = to_camel_case(&c.name);
            let req = !c.nullable;
            if comp.starts_with("input-") {
                format!(
                    "      {{\n        id: \"{name}\",\n        name: \"{name}\",\n        component: \"{comp}\",\n        labelProps: {{ label: \"{label}\", for: \"{name}\" }},\n        inputProps: {{ id: \"{name}\", name: \"{name}\", type: \"{input_type}\", required: {req}, placeholder: \"Enter {label}...\" }},\n      }}"
                )
            } else {
                format!(
                    "      {{\n        id: \"{name}\",\n        name: \"{name}\",\n        component: \"{comp}\",\n        label: \"{label}\",\n        required: {req},\n      }}"
                )
            }
        })
        .collect::<Vec<_>>()
        .join(",\n")
}
