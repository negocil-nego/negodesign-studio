use crate::model::project::Project;
use rusqlite::{Connection, params};
use std::{fs, path::Path};
use tauri::{AppHandle, Manager};

fn open_database(app: &AppHandle) -> Result<Connection, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&app_data_dir).map_err(|error| error.to_string())?;

    let connection =
        Connection::open(app_data_dir.join("negodesign.db")).map_err(|error| error.to_string())?;
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS TB_PROJECTS (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );",
        )
        .map_err(|error| error.to_string())?;

    Ok(connection)
}

pub fn list_projects(app: &AppHandle) -> Result<Vec<Project>, String> {
    let connection = open_database(app)?;
    let mut statement = connection
        .prepare("SELECT id, name, path FROM TB_PROJECTS ORDER BY id DESC")
        .map_err(|error| error.to_string())?;
    let projects = statement
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
            })
        })
        .map_err(|error| error.to_string())?;

    projects
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())
}

pub fn save_project(app: &AppHandle, path: &str) -> Result<Project, String> {
    let name = Path::new(path)
        .file_name()
        .and_then(|folder| folder.to_str())
        .filter(|folder| !folder.is_empty())
        .unwrap_or(path)
        .to_string();
    let connection = open_database(app)?;

    connection
        .execute(
            "INSERT INTO TB_PROJECTS (name, path) VALUES (?1, ?2)
             ON CONFLICT(path) DO UPDATE SET name = excluded.name",
            params![name, path],
        )
        .map_err(|error| error.to_string())?;

    connection
        .query_row(
            "SELECT id, name, path FROM TB_PROJECTS WHERE path = ?1",
            [path],
            |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                })
            },
        )
        .map_err(|error| error.to_string())
}
