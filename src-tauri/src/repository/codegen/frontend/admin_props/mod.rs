use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{to_kebab_case, to_pascal_case, to_title_label},
};

mod helpers;

use self::helpers::{
    build_columns_ts, build_form_fields_ts, build_row_cells_ts, build_sidebar_items,
};

pub fn build_admin_config_ts(table: &DatabaseTable, all_tables: &[DatabaseTable]) -> String {
    let pascal = to_pascal_case(&table.name);
    let current_kebab = to_kebab_case(&table.name);
    let title = to_title_label(&table.name);

    let sidebar_items = build_sidebar_items(table, all_tables);
    let columns_ts = build_columns_ts(table);
    let row_cells_ts = build_row_cells_ts(table);
    let form_fields_ts = build_form_fields_ts(table);

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
