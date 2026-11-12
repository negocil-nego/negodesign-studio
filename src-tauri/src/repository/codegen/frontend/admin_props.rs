use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{
        editable_columns, infer_form_component, is_badge_column, to_camel_case, to_kebab_case,
        to_pascal_case, to_title_label,
    },
};

pub fn build_admin_config_ts(table: &DatabaseTable, all_tables: &[DatabaseTable]) -> String {
    let pascal = to_pascal_case(&table.name);
    let current_kebab = to_kebab_case(&table.name);
    let title = to_title_label(&table.name);

    let sidebar_items = all_tables
        .iter()
        .map(|t| {
            let k = to_kebab_case(&t.name);
            let label = to_title_label(&t.name);
            let active = k == current_kebab;
            format!(
                "        {{ label: \"{label}\", href: \"/admin/{k}\", icon: \"Layout04\", active: {active} }}"
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let columns_ts = table
        .columns
        .iter()
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
        .join(",\n");

    let row_cells_ts = table
        .columns
        .iter()
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
        .join(",\n");

    let form_fields_ts = editable_columns(table)
        .iter()
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
        .join(",\n");

    format!(
        "import type {{ DataTextProps }} from \"../../negodesign/project/ui/organisms/data-table/data/types\";\n\
import type {{ AdminProps }} from \"../../negodesign/project/ui/pages/admin/data/types\";\n\n\
export const {pascal}Columns: DataTextProps[] = [\n{columns_ts}\n];\n\n\
export function map{pascal}Rows(records: Record<string, any>[]): DataTextProps[] {{\n\
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
          title: \"Database Tables\",\n\
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
      onRowAction: handlers?.onRowAction,\n\
    }},\n\
    modalFormProps: {{\n\
      title: \"{title}\",\n\
      form: {{\n\
        style: \"grid\",\n\
        fields: [\n{form_fields_ts}\n        ],\n\
        onSubmit: handlers?.onSubmit,\n\
      }},\n\
    }},\n\
  }};\n\
}}\n",
        upper_title = title.to_ascii_uppercase()
    )
}

