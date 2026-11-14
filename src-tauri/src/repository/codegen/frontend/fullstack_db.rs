use crate::{
    model::{codegen::DatabaseEngine, migration::DatabaseTable},
    repository::codegen::naming::{editable_columns, primary_key_column, to_camel_case, to_pascal_case},
};

pub fn build_db_client_ts(engine: &DatabaseEngine) -> String {
    match engine {
        DatabaseEngine::Postgres => r#"import { Pool } from "pg";

export const pool = new Pool({
  connectionString: process.env.DATABASE_URL ?? "postgresql://postgres:postgres@localhost:5432/app",
});

export async function queryDb<T = any>(sql: string, params: any[] = []): Promise<T[]> {
  const { rows } = await pool.query(sql, params);
  return rows as T[];
}
"#
        .to_string(),
        DatabaseEngine::Mysql => r#"import mysql from "mysql2/promise";

export const pool = mysql.createPool(
  process.env.DATABASE_URL ?? "mysql://root:root@localhost:3306/app",
);

export async function queryDb<T = any>(sql: string, params: any[] = []): Promise<T[]> {
  const [rows] = await pool.execute(sql, params);
  return (Array.isArray(rows) ? rows : []) as T[];
}
"#
        .to_string(),
        DatabaseEngine::Sqlite => r#"import Database from "better-sqlite3";

const db = new Database(process.env.SQLITE_PATH ?? "./database.sqlite");

export async function queryDb<T = any>(sql: string, params: any[] = []): Promise<T[]> {
  const stmt = db.prepare(sql);
  if (sql.trim().toUpperCase().startsWith("SELECT")) {
    return stmt.all(...params) as T[];
  }
  stmt.run(...params);
  return [] as T[];
}
"#
        .to_string(),
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
        r#"import {{ queryDb }} from "{db_import_path}";

export const {pascal}ServerRepository = {{
  async findAll(page = 0, size = 10) {{
    const offset = Math.max(0, page) * Math.max(1, size);
    const data = await queryDb(
      "SELECT * FROM {table_name} LIMIT {page_placeholders}",
      [size, offset],
    );
    const [countRow] = await queryDb<{{ total: number }}>(
      "SELECT COUNT(*) as total FROM {table_name}",
    );
    return {{ data, total: Number(countRow?.total ?? data.length), page, size }};
  }},
  async insert(payload: Record<string, any>) {{
    await queryDb(
      "INSERT INTO {table_name} ({cols_sql}) VALUES ({placeholders})",
      [{payload_vals}],
    );
    const [created] = await queryDb(
      "SELECT * FROM {table_name} ORDER BY {pk_col} DESC LIMIT 1",
    );
    return created ?? payload;
  }},
  async update(id: string | number, payload: Record<string, any>) {{
    await queryDb(
      "UPDATE {table_name} SET {set_clause} WHERE {pk_col} = {id_placeholder}",
      [{payload_vals}, id],
    );
    const [updated] = await queryDb(
      "SELECT * FROM {table_name} WHERE {pk_col} = {pk_single_placeholder}",
      [id],
    );
    return updated ?? {{ ...payload, {pk_col}: id }};
  }},
  async delete(id: string | number) {{
    await queryDb(
      "DELETE FROM {table_name} WHERE {pk_col} = {pk_single_placeholder}",
      [id],
    );
  }},
}};
"#
    )
}
