use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{
        editable_columns, primary_key_column, sql_to_java_type, to_camel_case,
    },
};
use std::collections::BTreeMap;

/// Generates a Spring Boot repository using the new Spring 6.1+ `JdbcClient` fluent API.
pub fn generate_jdbc_repository(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    full_pkg: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let pk = primary_key_column(table);
    let pk_type = sql_to_java_type(&pk.data_type);
    let pk_col = &pk.name;
    let edit_cols = editable_columns(table);

    let col_names: Vec<&str> = edit_cols.iter().map(|c| c.name.as_str()).collect();
    let insert_cols = col_names.join(", ");
    let insert_params = edit_cols
        .iter()
        .map(|c| format!(":{}", to_camel_case(&c.name)))
        .collect::<Vec<_>>()
        .join(", ");
    let update_set = edit_cols
        .iter()
        .map(|c| format!("{} = :{}", c.name, to_camel_case(&c.name)))
        .collect::<Vec<_>>()
        .join(", ");

    let param_bindings = edit_cols
        .iter()
        .map(|c| {
            let camel = to_camel_case(&c.name);
            format!("            .param(\"{camel}\", request.{camel}())")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let base_pkg = full_pkg
        .rsplit_once(".feature.")
        .map(|(b, _)| b)
        .unwrap_or("com.negocil.negoturismo.admin");

    let repo_src = format!(
        "package {full_pkg}.repository;\n\n\
import {base_pkg}.shared.PageResponse;\n\
import {full_pkg}.dto.{pascal}Request;\n\
import {full_pkg}.dto.{pascal}Response;\n\
import org.springframework.jdbc.core.simple.JdbcClient;\n\
import org.springframework.stereotype.Repository;\n\
import java.util.List;\n\n\
@Repository\n\
public class {pascal}Repository {{\n\
    private final JdbcClient jdbcClient;\n\n\
    public {pascal}Repository(JdbcClient jdbcClient) {{\n\
        this.jdbcClient = jdbcClient;\n\
    }}\n\n\
    public PageResponse<{pascal}Response> findAll(int page, int size) {{\n\
        int offset = Math.max(0, page) * Math.max(1, size);\n\
        List<{pascal}Response> data = jdbcClient\n\
            .sql(\"SELECT * FROM {table_name} LIMIT :size OFFSET :offset\")\n\
            .param(\"size\", size)\n\
            .param(\"offset\", offset)\n\
            .query({pascal}Response.class)\n\
            .list();\n\
        long total = jdbcClient\n\
            .sql(\"SELECT COUNT(*) FROM {table_name}\")\n\
            .query(Long.class)\n\
            .single();\n\
        return new PageResponse<>(data, total, page, size);\n\
    }}\n\n\
    public {pascal}Response insert({pascal}Request request) {{\n\
        jdbcClient\n\
            .sql(\"INSERT INTO {table_name} ({insert_cols}) VALUES ({insert_params})\")\n\
{param_bindings}\n\
            .update();\n\
        return jdbcClient\n\
            .sql(\"SELECT * FROM {table_name} ORDER BY {pk_col} DESC LIMIT 1\")\n\
            .query({pascal}Response.class)\n\
            .single();\n\
    }}\n\n\
    public {pascal}Response update({pk_type} id, {pascal}Request request) {{\n\
        jdbcClient\n\
            .sql(\"UPDATE {table_name} SET {update_set} WHERE {pk_col} = :id\")\n\
{param_bindings}\n\
            .param(\"id\", id)\n\
            .update();\n\
        return jdbcClient\n\
            .sql(\"SELECT * FROM {table_name} WHERE {pk_col} = :id\")\n\
            .param(\"id\", id)\n\
            .query({pascal}Response.class)\n\
            .single();\n\
    }}\n\n\
    public void delete({pk_type} id) {{\n\
        jdbcClient\n\
            .sql(\"DELETE FROM {table_name} WHERE {pk_col} = :id\")\n\
            .param(\"id\", id)\n\
            .update();\n\
    }}\n\
}}\n",
        table_name = table.name
    );

    files.insert(
        format!("{base_dir}/repository/{pascal}Repository.java"),
        repo_src,
    );
}

