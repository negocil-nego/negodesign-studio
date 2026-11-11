use crate::{
    model::{codegen::DatabaseEngine, migration::DatabaseTable},
    repository::codegen::naming::{editable_columns, primary_key_column, to_camel_case, to_pascal_case},
};

pub fn build_db_client_ts(engine: &DatabaseEngine) -> String {
    match engine {
        DatabaseEngine::Postgres => String::from(
            "import { Pool } from \"pg\";\n\n\
export const pool = new Pool({\n\
  connectionString: process.env.DATABASE_URL ?? \"postgresql://postgres:postgres@localhost:5432/app\",\n\
});\n\n\
export async function queryDb<T = any>(sql: string, params: any[] = []): Promise<T[]> {\n\
  const { rows } = await pool.query(sql, params);\n\
  return rows as T[];\n\
}\n",
        ),
        DatabaseEngine::Mysql => String::from(
            "import mysql from \"mysql2/promise\";\n\n\
export const pool = mysql.createPool(\n\
  process.env.DATABASE_URL ?? \"mysql://root:root@localhost:3306/app\",\n\
);\n\n\
export async function queryDb<T = any>(sql: string, params: any[] = []): Promise<T[]> {\n\
  const [rows] = await pool.execute(sql, params);\n\
  return (Array.isArray(rows) ? rows : []) as T[];\n\
}\n",
        ),
        DatabaseEngine::Sqlite => String::from(
            "import Database from \"better-sqlite3\";\n\n\
const db = new Database(process.env.SQLITE_PATH ?? \"./database.sqlite\");\n\n\
export async function queryDb<T = any>(sql: string, params: any[] = []): Promise<T[]> {\n\
  const stmt = db.prepare(sql);\n\
  if (sql.trim().toUpperCase().startsWith(\"SELECT\")) {\n\
    return stmt.all(...params) as T[];\n\
  }\n\
  stmt.run(...params);\n\
  return [] as T[];\n\
}\n",
        ),
    }
}

pub fn build_direct_table_repository_ts(
    engine: &DatabaseEngine,
    table: &DatabaseTable,
    db_import_path: &str,
) -> String {
    let pascal = to_pascal_case(&table.name);
    let pk = primary_key_column(table);
    let pk_col = &pk.name;
    let table_name = &table.name;
    let edit_cols = editable_columns(table);

    let cols_sql = edit_cols
        .iter()
        .map(|c| c.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    let is_pg = *engine == DatabaseEngine::Postgres;
    let placeholders = edit_cols
        .iter()
        .enumerate()
        .map(|(i, _)| if is_pg { format!("${}", i + 1) } else { "?".to_string() })
        .collect::<Vec<_>>()
        .join(", ");

    let set_clause = edit_cols
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if is_pg {
                format!("{} = ${}", c.name, i + 1)
            } else {
                format!("{} = ?", c.name)
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    let id_placeholder = if is_pg {
        format!("${}", edit_cols.len() + 1)
    } else {
        "?".to_string()
    };
    let page_placeholders = if is_pg { "$1 OFFSET $2" } else { "? OFFSET ?" };
    let pk_single_placeholder = if is_pg { "$1" } else { "?" };

    let payload_vals = edit_cols
        .iter()
        .map(|c| {
            let camel = to_camel_case(&c.name);
            format!("payload.{camel} ?? payload[\"{}\"] ?? null", c.name)
        })
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "import {{ queryDb }} from \"{db_import_path}\";\n\n\
export const {pascal}ServerRepository = {{\n\
  async findAll(page = 0, size = 10) {{\n\
    const offset = Math.max(0, page) * Math.max(1, size);\n\
    const data = await queryDb(\n\
      \"SELECT * FROM {table_name} LIMIT {page_placeholders}\",\n\
      [size, offset],\n\
    );\n\
    const [countRow] = await queryDb<{{ total: number }}>(\n\
      \"SELECT COUNT(*) as total FROM {table_name}\",\n\
    );\n\
    return {{ data, total: Number(countRow?.total ?? data.length), page, size }};\n\
  }},\n\
  async insert(payload: Record<string, any>) {{\n\
    await queryDb(\n\
      \"INSERT INTO {table_name} ({cols_sql}) VALUES ({placeholders})\",\n\
      [{payload_vals}],\n\
    );\n\
    const [created] = await queryDb(\n\
      \"SELECT * FROM {table_name} ORDER BY {pk_col} DESC LIMIT 1\",\n\
    );\n\
    return created ?? payload;\n\
  }},\n\
  async update(id: string | number, payload: Record<string, any>) {{\n\
    await queryDb(\n\
      \"UPDATE {table_name} SET {set_clause} WHERE {pk_col} = {id_placeholder}\",\n\
      [{payload_vals}, id],\n\
    );\n\
    const [updated] = await queryDb(\n\
      \"SELECT * FROM {table_name} WHERE {pk_col} = {pk_single_placeholder}\",\n\
      [id],\n\
    );\n\
    return updated ?? {{ ...payload, {pk_col}: id }};\n\
  }},\n\
  async delete(id: string | number) {{\n\
    await queryDb(\n\
      \"DELETE FROM {table_name} WHERE {pk_col} = {pk_single_placeholder}\",\n\
      [id],\n\
    );\n\
  }},\n\
}};\n"
    )
}

