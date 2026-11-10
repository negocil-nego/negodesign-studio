use crate::{
    model::migration::{
        DatabaseColumn, DatabaseConnection, DatabaseTable, SavedDatabaseConnection,
    },
    repository::database::drivers::{inspect_mysql, inspect_postgres},
};
use rusqlite::{Connection, params};
use std::{collections::BTreeMap, fs};
use tauri::{AppHandle, Manager};

pub fn inspect(connection: &DatabaseConnection) -> Result<Vec<DatabaseTable>, String> {
    let mut map: BTreeMap<String, Vec<DatabaseColumn>> = BTreeMap::new();
    match connection.driver.to_ascii_lowercase().as_str() {
        "mysql" => inspect_mysql(connection, &mut map)?,
        "postgres" | "postgresql" => inspect_postgres(connection, &mut map)?,
        other => {
            return Err(format!(
                "Motor «{other}» não suportado. Escolha MySQL ou PostgreSQL."
            ));
        }
    }
    Ok(map
        .into_iter()
        .map(|(name, columns)| DatabaseTable { name, columns })
        .collect())
}

fn open_saved_connections(app: &AppHandle) -> Result<Connection, String> {
    let app_data = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_data).map_err(|e| e.to_string())?;
    let db = Connection::open(app_data.join("negodesign.db")).map_err(|e| e.to_string())?;
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS TB_DATABASE_CONNECTIONS (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            driver TEXT NOT NULL,
            host TEXT NOT NULL,
            port INTEGER NOT NULL,
            database_name TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            last_connected TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(driver, host, port, database_name, username)
        );",
    )
    .map_err(|e| e.to_string())?;
    Ok(db)
}

pub fn inspect_and_save(
    app: &AppHandle,
    connection: &DatabaseConnection,
) -> Result<Vec<DatabaseTable>, String> {
    let tables = inspect(connection)?;
    let db = open_saved_connections(app)?;
    db.execute(
        "INSERT INTO TB_DATABASE_CONNECTIONS
            (driver, host, port, database_name, username, password, last_connected)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, CURRENT_TIMESTAMP)
         ON CONFLICT(driver, host, port, database_name, username) DO UPDATE SET
            password = excluded.password,
            last_connected = CURRENT_TIMESTAMP",
        params![
            connection.driver,
            connection.host,
            connection.port,
            connection.database,
            connection.user,
            connection.password
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(tables)
}

pub fn list_saved_connections(app: &AppHandle) -> Result<Vec<SavedDatabaseConnection>, String> {
    let db = open_saved_connections(app)?;
    let mut statement = db
        .prepare(
            "SELECT id, driver, host, port, database_name, username, last_connected
             FROM TB_DATABASE_CONNECTIONS ORDER BY last_connected DESC, id DESC LIMIT 8",
        )
        .map_err(|e| e.to_string())?;
    let saved = statement
        .query_map([], |row| {
            Ok(SavedDatabaseConnection {
                id: row.get(0)?,
                driver: row.get(1)?,
                host: row.get(2)?,
                port: row.get(3)?,
                database: row.get(4)?,
                user: row.get(5)?,
                last_connected: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;
    saved
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

pub fn inspect_saved_connection(app: &AppHandle, id: i64) -> Result<Vec<DatabaseTable>, String> {
    let db = open_saved_connections(app)?;
    let connection = db
        .query_row(
            "SELECT driver, host, port, database_name, username, password
             FROM TB_DATABASE_CONNECTIONS WHERE id = ?1",
            [id],
            |row| {
                Ok(DatabaseConnection {
                    driver: row.get(0)?,
                    host: row.get(1)?,
                    port: row.get(2)?,
                    database: row.get(3)?,
                    user: row.get(4)?,
                    password: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    inspect_and_save(app, &connection)
}
