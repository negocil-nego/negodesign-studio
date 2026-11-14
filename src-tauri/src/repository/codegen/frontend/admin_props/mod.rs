use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{to_kebab_case, to_pascal_case, to_title_label},
};

mod helpers;

pub use self::helpers::build_admin_sidebar_ts;
use self::helpers::{build_columns_ts, build_form_fields_ts, build_row_cells_ts};

/// Constrói o ficheiro de configuração AdminCrud de uma tabela.
///
/// `negodesign_import` e `sidebar_import` são passados pelo chamador porque
/// dependem da estrutura de pastas de cada framework (SvelteKit guarda as
/// features em `src/lib/features/<grupo>/<subpasta>/`, o Nuxt em
/// `composables/features/<kebab>/`, etc.).
pub fn build_admin_config_ts(
    table: &DatabaseTable,
    _all_tables: &[DatabaseTable],
    negodesign_import: &str,
    sidebar_import: &str,
) -> String {
    let pascal = to_pascal_case(&table.name);
    let current_kebab = to_kebab_case(&table.name);
    let title = to_title_label(&table.name);

    let columns_ts = build_columns_ts(table);
    let row_cells_ts = build_row_cells_ts(table);
    let form_fields_ts = build_form_fields_ts(table);

    format!(
        r#"import type {{ DataTextProps }} from "{negodesign_import}/project/ui/organisms/data-table/data/types";
import type {{ AdminProps }} from "{negodesign_import}/project/ui/pages/admin/data/types";
import {{ getAdminSidebarProps }} from "{sidebar_import}";

export const {pascal}Columns: DataTextProps[] = [
{columns_ts}
];

export function map{pascal}Rows(records: Record<string, any>[]): DataTextProps[] {{
  return records.flatMap((item) => [
{row_cells_ts}
  ]);
}}

export function create{pascal}AdminProps(
  records: Record<string, any>[] = [],
  handlers?: {{
    onSubmit?: (values?: Record<string, unknown>) => void;
    onRowAction?: (action: string, rowIndex: number, row: DataTextProps[]) => void;
  }}
): AdminProps {{
  return {{
    title: "{title}",
    breadcrumb: {{
      items: [
        {{ label: "Admin Dashboard", href: "/admin" }},
        {{ label: "{title}", active: true }},
      ],
    }},
    addButtonLabel: "ADD {upper_title}",
    sidebarProps: getAdminSidebarProps("{current_kebab}"),
    menuProps: {{
      style: "default",
      logoText: "{title} Management",
      menus: [
        {{ textProps: {{ text: "Dashboard" }}, href: "/admin/{current_kebab}" }},
      ],
    }},
    datatableProps: {{
      variant: "01",
      title: "{title}",
      columns: {pascal}Columns,
      rows: map{pascal}Rows(records),
      addLabel: "ADD {upper_title}",
      onRowAction: handlers?.onRowAction,
    }},
    modalFormProps: {{
      title: "{title}",
      form: {{
        style: "grid",
        fields: [
{form_fields_ts}
        ],
        onSubmit: handlers?.onSubmit,
      }},
    }},
  }};
}}
"#,
        upper_title = title.to_ascii_uppercase()
    )
}
