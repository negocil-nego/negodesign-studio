use crate::model::migration::{DatabaseColumn, DatabaseConnection};
use std::collections::BTreeMap;

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

fn push(map: &mut BTreeMap<String, Vec<DatabaseColumn>>, table: String, column: DatabaseColumn) {
    map.entry(table).or_default().push(column);
}

pub fn inspect_mysql(
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

    let mut client = mysql::Conn::new(opts).map_err(|e| e.to_string())?;
    let rows: Vec<(String, String, String, String, String)> = client
        .exec(MYSQL_QUERY, (connection.database.clone(),))
        .map_err(|e| e.to_string())?;

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

pub fn inspect_postgres(
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
        .map_err(|e| e.to_string())?;
    let rows = client.query(POSTGRES_QUERY, &[]).map_err(|e| e.to_string())?;

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

