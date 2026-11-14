use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{
        editable_columns, feature_group_keys, primary_key_column, sql_to_java_type, to_camel_case,
        to_kebab_case, to_pascal_case, to_snake_case,
    },
};
use std::collections::BTreeMap;

/// Converts a Java type (from `sql_to_java_type`) into the equivalent Kotlin type.
fn java_to_kotlin_type(java_type: &str) -> String {
    match java_type {
        "Integer" => "Int".to_string(),
        "Long" => "Long".to_string(),
        "Double" => "Double".to_string(),
        "Boolean" => "Boolean".to_string(),
        "String" => "String".to_string(),
        other => other.to_string(),
    }
}

fn kt_type(data_type: &str) -> String {
    java_to_kotlin_type(sql_to_java_type(data_type))
}

pub fn generate(
    tables: &[DatabaseTable],
    base_package: &str,
    orm_strategy: &str,
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();
    let pkg_path = base_package.replace('.', "/");
    let use_jdbc = orm_strategy == "spring-jdbc" || orm_strategy == "jdbc";

    files.insert(
        format!("src/main/kotlin/{pkg_path}/shared/PageResponse.kt"),
        format!(
            "package {base_package}.shared\n\ndata class PageResponse<T>(\n\
    \tval data: List<T>,\n\
    \tval total: Long,\n\
    \tval page: Int,\n\
    \tval size: Int,\n\
)\n"
        ),
    );

    // Agrupamento por domínio (mesma regra do backend Java e do frontend):
    // agent + agent_address + … → feature "agent".
    let kebabs: Vec<String> = tables.iter().map(|t| to_kebab_case(&t.name)).collect();
    let feature_groups = feature_group_keys(&kebabs);

    for (table, table_kebab) in tables.iter().zip(kebabs.iter()) {
        let feature_pkg = to_snake_case(
            feature_groups
                .get(table_kebab)
                .map(String::as_str)
                .unwrap_or(table_kebab),
        );
        let pascal = to_pascal_case(&table.name);
        let kebab = table_kebab.clone();
        let full_pkg = format!("{base_package}.feature.{feature_pkg}");
        let base_dir = format!("src/main/kotlin/{pkg_path}/feature/{feature_pkg}");

        let pk = primary_key_column(table);
        let pk_kt_type = kt_type(&pk.data_type);

        files.insert(
            format!("{base_dir}/dto/{pascal}Request.kt"),
            build_request_dto(&full_pkg, &pascal, table),
        );
        files.insert(
            format!("{base_dir}/dto/{pascal}Response.kt"),
            build_response_dto(&full_pkg, &pascal, table),
        );

        if use_jdbc {
            generate_jdbc_repository(&mut files, &base_dir, &full_pkg, &pascal, table);
        } else {
            generate_jpa_repository(&mut files, &base_dir, &full_pkg, &pascal, table);
        }

        files.insert(
            format!("{base_dir}/service/{pascal}Service.kt"),
            build_service(
                base_package,
                &full_pkg,
                &pascal,
                &pk_kt_type,
            ),
        );
        files.insert(
            format!("{base_dir}/controller/{pascal}Controller.kt"),
            build_controller(
                base_package,
                &full_pkg,
                &pascal,
                &kebab,
                &pk_kt_type,
            ),
        );
    }

    Ok(files)
}

fn build_request_dto(full_pkg: &str, pascal: &str, table: &DatabaseTable) -> String {
    let cols = editable_columns(table);
    let fields = if cols.is_empty() {
        "\tval name: String".to_string()
    } else {
        cols.iter()
            .map(|c| format!("\tval {}: {}", to_camel_case(&c.name), kt_type(&c.data_type)))
            .collect::<Vec<_>>()
            .join(",\n")
    };
    format!("package {full_pkg}.dto\n\ndata class {pascal}Request(\n{fields},\n)\n")
}

fn build_response_dto(full_pkg: &str, pascal: &str, table: &DatabaseTable) -> String {
    let fields = table
        .columns
        .iter()
        .map(|c| format!("\tval {}: {}", to_camel_case(&c.name), kt_type(&c.data_type)))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("package {full_pkg}.dto\n\ndata class {pascal}Response(\n{fields},\n)\n")
}

fn build_service(
    base_package: &str,
    full_pkg: &str,
    pascal: &str,
    pk_type: &str,
) -> String {
    format!(
        "package {full_pkg}.service\n\n\
import {base_package}.shared.PageResponse\n\
import {full_pkg}.dto.{pascal}Request\n\
import {full_pkg}.dto.{pascal}Response\n\
import {full_pkg}.repository.{pascal}Repository\n\
import org.springframework.stereotype.Service\n\
import org.springframework.transaction.annotation.Transactional\n\n\
@Service\n\
class {pascal}Service(\n\
    \tprivate val repository: {pascal}Repository,\n\
) {{\n\n\
    @Transactional(readOnly = true)\n\
    fun findAll(page: Int, size: Int): PageResponse<{pascal}Response> {{\n\
    \treturn repository.findAll(page, size)\n\
    }}\n\n\
    @Transactional\n\
    fun insert(request: {pascal}Request): {pascal}Response {{\n\
    \treturn repository.insert(request)\n\
    }}\n\n\
    @Transactional\n\
    fun update(id: {pk_type}, request: {pascal}Request): {pascal}Response {{\n\
    \treturn repository.update(id, request)\n\
    }}\n\n\
    @Transactional\n\
    fun delete(id: {pk_type}) {{\n\
    \trepository.delete(id)\n\
    }}\n\
}}\n"
    )
}

fn build_controller(
    base_package: &str,
    full_pkg: &str,
    pascal: &str,
    kebab: &str,
    pk_type: &str,
) -> String {
    format!(
        "package {full_pkg}.controller\n\n\
import {base_package}.shared.PageResponse\n\
import {full_pkg}.dto.{pascal}Request\n\
import {full_pkg}.dto.{pascal}Response\n\
import {full_pkg}.service.{pascal}Service\n\
import org.springframework.http.HttpStatus\n\
import org.springframework.web.bind.annotation.*\n\n\
@RestController\n\
@RequestMapping(\"/api/{kebab}\")\n\
@CrossOrigin(origins = [\"*\"])\n\
class {pascal}Controller(\n\
    \tprivate val service: {pascal}Service,\n\
) {{\n\n\
    @GetMapping\n\
    fun findAll(\n\
        @RequestParam(defaultValue = \"0\") page: Int,\n\
        @RequestParam(defaultValue = \"10\") size: Int,\n\
    ): PageResponse<{pascal}Response> {{\n\
    \treturn service.findAll(page, size)\n\
    }}\n\n\
    @PostMapping\n\
    @ResponseStatus(HttpStatus.CREATED)\n\
    fun insert(@RequestBody request: {pascal}Request): {pascal}Response {{\n\
    \treturn service.insert(request)\n\
    }}\n\n\
    @PutMapping(\"/{{id}}\")\n\
    fun update(@PathVariable id: {pk_type}, @RequestBody request: {pascal}Request): {pascal}Response {{\n\
    \treturn service.update(id, request)\n\
    }}\n\n\
    @DeleteMapping(\"/{{id}}\")\n\
    @ResponseStatus(HttpStatus.NO_CONTENT)\n\
    fun delete(@PathVariable id: {pk_type}) {{\n\
    \tservice.delete(id)\n\
    }}\n\
}}\n"
    )
}

fn generate_jpa_repository(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    full_pkg: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let pk = primary_key_column(table);
    let pk_type = kt_type(&pk.data_type);

    let mut entity_fields = String::new();
    for col in &table.columns {
        let kt = kt_type(&col.data_type);
        let camel = to_camel_case(&col.name);
        if col.primary_key {
            entity_fields.push_str(&format!(
                "    @Id\n    @GeneratedValue(strategy = GenerationType.IDENTITY)\n    @Column(name = \"{}\")\n    var {camel}: {kt}? = null\n\n",
                col.name
            ));
        } else {
            entity_fields.push_str(&format!(
                "    @Column(name = \"{}\", nullable = {})\n    var {camel}: {kt}? = null\n\n",
                col.name, col.nullable
            ));
        }
    }

    let entity_src = format!(
        "package {full_pkg}.entity\n\n\
import jakarta.persistence.*\n\n\
@Entity\n\
@Table(name = \"{}\")\n\
class {pascal}Entity {{\n\
{entity_fields}\
}}\n",
        table.name
    );
    files.insert(format!("{base_dir}/entity/{pascal}Entity.kt"), entity_src);

    let jpa_interface = format!(
        "package {full_pkg}.repository\n\n\
import {full_pkg}.entity.{pascal}Entity\n\
import org.springframework.data.jpa.repository.JpaRepository\n\n\
interface {pascal}JpaRepository : JpaRepository<{pascal}Entity, {pk_type}>\n"
    );
    files.insert(
        format!("{base_dir}/repository/{pascal}JpaRepository.kt"),
        jpa_interface,
    );

    let edit_cols = editable_columns(table);
    let setters = edit_cols
        .iter()
        .map(|c| {
            let camel = to_camel_case(&c.name);
            format!("        entity.{camel} = request.{camel}")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let response_args = table
        .columns
        .iter()
        .map(|c| format!("entity.{}!!", to_camel_case(&c.name)))
        .collect::<Vec<_>>()
        .join(", ");

    let base_pkg = full_pkg
        .rsplit_once(".feature.")
        .map(|(b, _)| b)
        .unwrap_or(full_pkg);

    let repo_impl = format!(
        "package {full_pkg}.repository\n\n\
import {base_pkg}.shared.PageResponse\n\
import {full_pkg}.dto.{pascal}Request\n\
import {full_pkg}.dto.{pascal}Response\n\
import {full_pkg}.entity.{pascal}Entity\n\
import org.springframework.data.domain.PageRequest\n\
import org.springframework.stereotype.Repository\n\n\
@Repository\n\
class {pascal}Repository(\n\
    \tprivate val jpaRepository: {pascal}JpaRepository,\n\
) {{\n\n\
    fun findAll(page: Int, size: Int): PageResponse<{pascal}Response> {{\n\
    \tval result = jpaRepository.findAll(PageRequest.of(page, size))\n\
    \treturn PageResponse(\n\
        \tresult.content.map {{ toResponse(it) }},\n\
        \tresult.totalElements,\n\
        \tpage,\n\
        \tsize,\n\
    \t)\n\
    }}\n\n\
    fun insert(request: {pascal}Request): {pascal}Response {{\n\
    \tval entity = {pascal}Entity()\n\
{setters}\n\
    \treturn toResponse(jpaRepository.save(entity))\n\
    }}\n\n\
    fun update(id: {pk_type}, request: {pascal}Request): {pascal}Response {{\n\
    \tval entity = jpaRepository.findById(id)\n\
        \t.orElseThrow {{ IllegalArgumentException(\"{pascal} not found: $id\") }}\n\
{setters}\n\
    \treturn toResponse(jpaRepository.save(entity))\n\
    }}\n\n\
    fun delete(id: {pk_type}) {{\n\
    \tjpaRepository.deleteById(id)\n\
    }}\n\n\
    private fun toResponse(entity: {pascal}Entity): {pascal}Response {{\n\
    \treturn {pascal}Response({response_args})\n\
    }}\n\
}}\n"
    );

    files.insert(
        format!("{base_dir}/repository/{pascal}Repository.kt"),
        repo_impl,
    );
}

fn generate_jdbc_repository(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    full_pkg: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let pk = primary_key_column(table);
    let pk_type = kt_type(&pk.data_type);
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
            format!("            .param(\"{camel}\", request.{camel})")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let base_pkg = full_pkg
        .rsplit_once(".feature.")
        .map(|(b, _)| b)
        .unwrap_or(full_pkg);

    let repo_src = format!(
        "package {full_pkg}.repository\n\n\
import {base_pkg}.shared.PageResponse\n\
import {full_pkg}.dto.{pascal}Request\n\
import {full_pkg}.dto.{pascal}Response\n\
import org.springframework.jdbc.core.simple.JdbcClient\n\
import org.springframework.stereotype.Repository\n\n\
@Repository\n\
class {pascal}Repository(\n\
    \tprivate val jdbcClient: JdbcClient,\n\
) {{\n\n\
    fun findAll(page: Int, size: Int): PageResponse<{pascal}Response> {{\n\
    \tval offset = maxOf(0, page) * maxOf(1, size)\n\
    \tval data = jdbcClient\n\
        \t.sql(\"SELECT * FROM {table_name} LIMIT :size OFFSET :offset\")\n\
        \t.param(\"size\", size)\n\
        \t.param(\"offset\", offset)\n\
        \t.query({pascal}Response::class.java)\n\
        \t.list()\n\
    \tval total = jdbcClient\n\
        \t.sql(\"SELECT COUNT(*) FROM {table_name}\")\n\
        \t.query(Long::class.javaObjectType)\n\
        \t.single()\n\
    \treturn PageResponse(data, total, page, size)\n\
    }}\n\n\
    fun insert(request: {pascal}Request): {pascal}Response {{\n\
    \tjdbcClient\n\
        \t.sql(\"INSERT INTO {table_name} ({insert_cols}) VALUES ({insert_params})\")\n\
{param_bindings}\n\
        \t.update()\n\
    \treturn jdbcClient\n\
        \t.sql(\"SELECT * FROM {table_name} ORDER BY {pk_col} DESC LIMIT 1\")\n\
        \t.query({pascal}Response::class.java)\n\
        \t.single()\n\
    }}\n\n\
    fun update(id: {pk_type}, request: {pascal}Request): {pascal}Response {{\n\
    \tjdbcClient\n\
        \t.sql(\"UPDATE {table_name} SET {update_set} WHERE {pk_col} = :id\")\n\
{param_bindings}\n\
        \t.param(\"id\", id)\n\
        \t.update()\n\
    \treturn jdbcClient\n\
        \t.sql(\"SELECT * FROM {table_name} WHERE {pk_col} = :id\")\n\
        \t.param(\"id\", id)\n\
        \t.query({pascal}Response::class.java)\n\
        \t.single()\n\
    }}\n\n\
    fun delete(id: {pk_type}) {{\n\
    \tjdbcClient\n\
        \t.sql(\"DELETE FROM {table_name} WHERE {pk_col} = :id\")\n\
        \t.param(\"id\", id)\n\
        \t.update()\n\
    }}\n\
}}\n",
        table_name = table.name
    );

    files.insert(
        format!("{base_dir}/repository/{pascal}Repository.kt"),
        repo_src,
    );
}
