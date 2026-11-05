use std::{collections::BTreeMap, fs};

use crate::model::migration::{
    DatabaseColumn, DatabaseConnection, DatabaseTable, SavedDatabaseConnection,
};
use rusqlite::{Connection, params};
use tauri::{AppHandle, Manager};

const MYSQL_QUERY: &str = "SELECT TABLE_NAME, COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_KEY \
FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = ? ORDER BY TABLE_NAME, ORDINAL_POSITION";

const POSTGRES_QUERY: &str = "SELECT c.table_name, c.column_name, c.data_type, c.is_nullable, \
COALESCE(pk.is_primary, false) FROM information_schema.columns c \
LEFT JOIN (SELECT kcu.table_schema, kcu.table_name, kcu.column_name, true AS is_primary \
FROM information_schema.table_constraints tc JOIN information_schema.key_column_usage kcu \
ON tc.constraint_name = kcu.constraint_name AND tc.table_schema = kcu.table_schema \
WHERE tc.constraint_type = 'PRIMARY KEY') pk \
ON pk.table_schema = c.table_schema AND pk.table_name = c.table_name \
AND pk.column_name = c.column_name \
WHERE c.table_schema NOT IN ('pg_catalog', 'information_schema') \
ORDER BY c.table_name, c.ordinal_position";

/// Obtém a estrutura das tabelas do motor indicado na ligação.
pub fn inspect(connection: &DatabaseConnection) -> Result<Vec<DatabaseTable>, String> {
    let mut map: BTreeMap<String, Vec<DatabaseColumn>> = BTreeMap::new();
    match connection.driver.to_ascii_lowercase().as_str() {
        "mysql" => inspect_mysql(connection, &mut map)?,
        "postgres" | "postgresql" => inspect_postgres(connection, &mut map)?,
        other => {
            return Err(format!(
                "Motor «{other}» não suportado. Escolha MySQL ou PostgreSQL."
            ))
        }
    }
    Ok(map
        .into_iter()
        .map(|(name, columns)| DatabaseTable { name, columns })
        .collect())
}

fn open_saved_connections(app: &AppHandle) -> Result<Connection, String> {
    let app_data = app.path().app_data_dir().map_err(|error| error.to_string())?;
    fs::create_dir_all(&app_data).map_err(|error| error.to_string())?;
    let db = Connection::open(app_data.join("negodesign.db")).map_err(|error| error.to_string())?;
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
    ).map_err(|error| error.to_string())?;
    Ok(db)
}

/// Checks the remote database before saving its credentials for future access.
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
        params![connection.driver, connection.host, connection.port, connection.database, connection.user, connection.password],
    ).map_err(|error| error.to_string())?;
    Ok(tables)
}

pub fn list_saved_connections(app: &AppHandle) -> Result<Vec<SavedDatabaseConnection>, String> {
    let db = open_saved_connections(app)?;
    let mut statement = db.prepare(
        "SELECT id, driver, host, port, database_name, username, last_connected
         FROM TB_DATABASE_CONNECTIONS ORDER BY last_connected DESC, id DESC LIMIT 8",
    ).map_err(|error| error.to_string())?;
    let saved = statement.query_map([], |row| Ok(SavedDatabaseConnection {
        id: row.get(0)?, driver: row.get(1)?, host: row.get(2)?, port: row.get(3)?,
        database: row.get(4)?, user: row.get(5)?, last_connected: row.get(6)?,
    })).map_err(|error| error.to_string())?;
    saved.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

pub fn inspect_saved_connection(app: &AppHandle, id: i64) -> Result<Vec<DatabaseTable>, String> {
    let db = open_saved_connections(app)?;
    let connection = db.query_row(
        "SELECT driver, host, port, database_name, username, password
         FROM TB_DATABASE_CONNECTIONS WHERE id = ?1",
        [id],
        |row| Ok(DatabaseConnection {
            driver: row.get(0)?, host: row.get(1)?, port: row.get(2)?,
            database: row.get(3)?, user: row.get(4)?, password: row.get(5)?,
        }),
    ).map_err(|error| error.to_string())?;
    inspect_and_save(app, &connection)
}

fn push(map: &mut BTreeMap<String, Vec<DatabaseColumn>>, table: String, column: DatabaseColumn) {
    map.entry(table).or_default().push(column);
}

fn inspect_mysql(
    connection: &DatabaseConnection,
    map: &mut BTreeMap<String, Vec<DatabaseColumn>>,
) -> Result<(), String> {
    use mysql::prelude::Queryable;

    let opts = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(connection.host.clone()))
        .tcp_port(connection.port)
        .user(Some(connection.user.clone()))
        .pass(Some(connection.password.clone()))
        .db_name(Some(connection.database.clone()));

    let mut client = mysql::Conn::new(opts).map_err(|error| error.to_string())?;
    let rows: Vec<(String, String, String, String, String)> = client
        .exec(MYSQL_QUERY, (connection.database.clone(),))
        .map_err(|error| error.to_string())?;

    for (table, name, data_type, nullable, key) in rows {
        push(
            map,
            table,
            DatabaseColumn {
                name,
                data_type,
                nullable: nullable.eq_ignore_ascii_case("yes"),
                primary_key: key.eq_ignore_ascii_case("pri"),
            },
        );
    }
    Ok(())
}

fn inspect_postgres(
    connection: &DatabaseConnection,
    map: &mut BTreeMap<String, Vec<DatabaseColumn>>,
) -> Result<(), String> {
    let mut config = postgres::Config::new();
    config
        .host(&connection.host)
        .port(connection.port)
        .user(&connection.user)
        .password(&connection.password)
        .dbname(&connection.database);

    let mut client = config
        .connect(postgres::NoTls)
        .map_err(|error| error.to_string())?;
    let rows = client
        .query(POSTGRES_QUERY, &[])
        .map_err(|error| error.to_string())?;

    for row in rows {
        push(
            map,
            row.get::<_, String>(0),
            DatabaseColumn {
                name: row.get::<_, String>(1),
                data_type: row.get::<_, String>(2),
                nullable: row.get::<_, String>(3).eq_ignore_ascii_case("yes"),
                primary_key: row.get::<_, bool>(4),
            },
        );
    }
    Ok(())
}
