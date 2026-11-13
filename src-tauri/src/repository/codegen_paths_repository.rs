use crate::model::codegen::CodegenPath;
use rusqlite::{params, Connection};
use std::fs;
use tauri::{AppHandle, Manager};

fn open_db(app: &AppHandle) -> Result<Connection, String> {
    let app_data = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_data).map_err(|e| e.to_string())?;
    let db = Connection::open(app_data.join("negodesign.db")).map_err(|e| e.to_string())?;
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS TB_CODEGEN_PATHS (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            target TEXT NOT NULL,
            path TEXT NOT NULL,
            label TEXT,
            last_used TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(target, path)
        );",
    )
    .map_err(|e| e.to_string())?;
    Ok(db)
}

pub fn list_paths(app: &AppHandle, target: &str) -> Result<Vec<CodegenPath>, String> {
    let db = open_db(app)?;
    let mut statement = db
        .prepare(
            "SELECT id, target, path, label, last_used
             FROM TB_CODEGEN_PATHS
             WHERE target = ?1
             ORDER BY last_used DESC, id DESC
             LIMIT 12",
        )
        .map_err(|e| e.to_string())?;
    let rows = statement
        .query_map([target], |row| {
            Ok(CodegenPath {
                id: row.get(0)?,
                target: row.get(1)?,
                path: row.get(2)?,
                label: row.get(3)?,
                last_used: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

pub fn save_path(
    app: &AppHandle,
    target: &str,
    path: &str,
    label: Option<&str>,
) -> Result<CodegenPath, String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err("O caminho não pode estar vazio.".to_string());
    }
    let db = open_db(app)?;
    db.execute(
        "INSERT INTO TB_CODEGEN_PATHS (target, path, label, last_used)
         VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)
         ON CONFLICT(target, path) DO UPDATE SET
            label = COALESCE(excluded.label, TB_CODEGEN_PATHS.label),
            last_used = CURRENT_TIMESTAMP",
        params![target, trimmed, label],
    )
    .map_err(|e| e.to_string())?;
    db.query_row(
        "SELECT id, target, path, label, last_used
         FROM TB_CODEGEN_PATHS WHERE target = ?1 AND path = ?2",
        params![target, trimmed],
        |row| {
            Ok(CodegenPath {
                id: row.get(0)?,
                target: row.get(1)?,
                path: row.get(2)?,
                label: row.get(3)?,
                last_used: row.get(4)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

pub fn delete_path(app: &AppHandle, id: i64) -> Result<(), String> {
    let db = open_db(app)?;
    db.execute("DELETE FROM TB_CODEGEN_PATHS WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
