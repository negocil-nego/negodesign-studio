use crate::model::project::Project;
use rusqlite::Connection;
use std::fs;
use tauri::{AppHandle, Manager};

pub fn open_database(app: &AppHandle) -> Result<Connection, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&app_data_dir).map_err(|error| error.to_string())?;

    let connection =
        Connection::open(app_data_dir.join("negodesign.db")).map_err(|error| error.to_string())?;
    connection
        .execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             CREATE TABLE IF NOT EXISTS TB_PROJECTS (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                language TEXT NOT NULL DEFAULT 'Unknown',
                active INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
             );
             CREATE TABLE IF NOT EXISTS TB_PROJECT_COMPONENTS (
                project_id INTEGER NOT NULL,
                component_id TEXT NOT NULL,
                name TEXT NOT NULL,
                entry TEXT NOT NULL,
                language TEXT NOT NULL,
                PRIMARY KEY (project_id, component_id)
             );",
        )
        .map_err(|error| error.to_string())?;

    ensure_column(&connection, "language", "TEXT NOT NULL DEFAULT 'Unknown'")?;
    ensure_column(&connection, "active", "INTEGER NOT NULL DEFAULT 0")?;
    connection
        .execute(
            "UPDATE TB_PROJECTS SET active = 1
             WHERE id = (SELECT id FROM TB_PROJECTS ORDER BY id DESC LIMIT 1)
               AND NOT EXISTS (SELECT 1 FROM TB_PROJECTS WHERE active = 1)",
            [],
        )
        .map_err(|error| error.to_string())?;
    Ok(connection)
}

fn ensure_column(connection: &Connection, column: &str, definition: &str) -> Result<(), String> {
    let mut statement = connection
        .prepare("PRAGMA table_info(TB_PROJECTS)")
        .map_err(|error| error.to_string())?;
    let columns = statement
        .query_map([], |row| row.get::<_, String>("name"))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    drop(statement);

    if !columns.iter().any(|existing| existing == column) {
        connection
            .execute_batch(&format!(
                "ALTER TABLE TB_PROJECTS ADD COLUMN {column} {definition};"
            ))
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub fn map_project(row: &rusqlite::Row<'_>) -> rusqlite::Result<Project> {
    Ok(Project {
        id: row.get("id")?,
        name: row.get("name")?,
        path: row.get("path")?,
        language: row.get("language")?,
        active: row.get::<_, i64>("active")? != 0,
    })
}

