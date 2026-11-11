pub mod jdbc;
pub mod jpa;

use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{
        editable_columns, primary_key_column, sql_to_java_type, to_camel_case, to_kebab_case,
        to_pascal_case, to_snake_case,
    },
};
use std::collections::BTreeMap;

pub fn generate(
    tables: &[DatabaseTable],
    base_package: &str,
    orm_strategy: &str,
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();
    let pkg_path = base_package.replace('.', "/");
    let use_jdbc = orm_strategy == "spring-jdbc" || orm_strategy == "jdbc";

    files.insert(
        format!("src/main/java/{pkg_path}/shared/PageResponse.java"),
        format!(
            "package {base_package}.shared;\n\nimport java.util.List;\n\npublic record PageResponse<T>(\n    List<T> data,\n    long total,\n    int page,\n    int size\n) {{}}\n"
        ),
    );

    for table in tables {
        let feature_pkg = to_snake_case(&table.name);
        let pascal = to_pascal_case(&table.name);
        let kebab = to_kebab_case(&table.name);
        let full_pkg = format!("{base_package}.feature.{feature_pkg}");
        let base_dir = format!("src/main/java/{pkg_path}/feature/{feature_pkg}");

        let pk = primary_key_column(table);
        let pk_java_type = sql_to_java_type(&pk.data_type);
        let pk_camel = to_camel_case(&pk.name);

        files.insert(
            format!("{base_dir}/dto/{pascal}Request.java"),
            build_request_dto(&full_pkg, &pascal, table),
        );
        files.insert(
            format!("{base_dir}/dto/{pascal}Response.java"),
            build_response_dto(&full_pkg, &pascal, table),
        );

        if use_jdbc {
            jdbc::generate_jdbc_repository(&mut files, &base_dir, &full_pkg, &pascal, table);
        } else {
            jpa::generate_jpa_repository(&mut files, &base_dir, &full_pkg, &pascal, table);
        }

        files.insert(
            format!("{base_dir}/service/{pascal}Service.java"),
            build_service(
                base_package,
                &full_pkg,
                &pascal,
                &pk_java_type,
                &pk_camel,
            ),
        );
        files.insert(
            format!("{base_dir}/controller/{pascal}Controller.java"),
            build_controller(
                base_package,
                &full_pkg,
                &pascal,
                &kebab,
                &pk_java_type,
            ),
        );
    }

    Ok(files)
}

fn build_request_dto(full_pkg: &str, pascal: &str, table: &DatabaseTable) -> String {
    let cols = editable_columns(table);
    let fields = if cols.is_empty() {
        "    String name".to_string()
    } else {
        cols.iter()
            .map(|c| format!("    {} {}", sql_to_java_type(&c.data_type), to_camel_case(&c.name)))
            .collect::<Vec<_>>()
            .join(",\n")
    };
    format!("package {full_pkg}.dto;\n\npublic record {pascal}Request(\n{fields}\n) {{}}\n")
}

fn build_response_dto(full_pkg: &str, pascal: &str, table: &DatabaseTable) -> String {
    let fields = table
        .columns
        .iter()
        .map(|c| format!("    {} {}", sql_to_java_type(&c.data_type), to_camel_case(&c.name)))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("package {full_pkg}.dto;\n\npublic record {pascal}Response(\n{fields}\n) {{}}\n")
}

fn build_service(
    base_package: &str,
    full_pkg: &str,
    pascal: &str,
    pk_type: &str,
    _pk_camel: &str,
) -> String {
    format!(
        "package {full_pkg}.service;\n\n\
import {base_package}.shared.PageResponse;\n\
import {full_pkg}.dto.{pascal}Request;\n\
import {full_pkg}.dto.{pascal}Response;\n\
import {full_pkg}.repository.{pascal}Repository;\n\
import org.springframework.stereotype.Service;\n\
import org.springframework.transaction.annotation.Transactional;\n\n\
@Service\n\
public class {pascal}Service {{\n\
    private final {pascal}Repository repository;\n\n\
    public {pascal}Service({pascal}Repository repository) {{\n\
        this.repository = repository;\n\
    }}\n\n\
    @Transactional(readOnly = true)\n\
    public PageResponse<{pascal}Response> findAll(int page, int size) {{\n\
        return repository.findAll(page, size);\n\
    }}\n\n\
    @Transactional\n\
    public {pascal}Response insert({pascal}Request request) {{\n\
        return repository.insert(request);\n\
    }}\n\n\
    @Transactional\n\
    public {pascal}Response update({pk_type} id, {pascal}Request request) {{\n\
        return repository.update(id, request);\n\
    }}\n\n\
    @Transactional\n\
    public void delete({pk_type} id) {{\n\
        repository.delete(id);\n\
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
        "package {full_pkg}.controller;\n\n\
import {base_package}.shared.PageResponse;\n\
import {full_pkg}.dto.{pascal}Request;\n\
import {full_pkg}.dto.{pascal}Response;\n\
import {full_pkg}.service.{pascal}Service;\n\
import org.springframework.http.HttpStatus;\n\
import org.springframework.web.bind.annotation.*;\n\n\
@RestController\n\
@RequestMapping(\"/api/{kebab}\")\n\
@CrossOrigin(origins = \"*\")\n\
public class {pascal}Controller {{\n\
    private final {pascal}Service service;\n\n\
    public {pascal}Controller({pascal}Service service) {{\n\
        this.service = service;\n\
    }}\n\n\
    @GetMapping\n\
    public PageResponse<{pascal}Response> findAll(\n\
        @RequestParam(defaultValue = \"0\") int page,\n\
        @RequestParam(defaultValue = \"10\") int size\n\
    ) {{\n\
        return service.findAll(page, size);\n\
    }}\n\n\
    @PostMapping\n\
    @ResponseStatus(HttpStatus.CREATED)\n\
    public {pascal}Response insert(@RequestBody {pascal}Request request) {{\n\
        return service.insert(request);\n\
    }}\n\n\
    @PutMapping(\"/{{id}}\")\n\
    public {pascal}Response update(@PathVariable {pk_type} id, @RequestBody {pascal}Request request) {{\n\
        return service.update(id, request);\n\
    }}\n\n\
    @DeleteMapping(\"/{{id}}\")\n\
    @ResponseStatus(HttpStatus.NO_CONTENT)\n\
    public void delete(@PathVariable {pk_type} id) {{\n\
        service.delete(id);\n\
    }}\n\
}}\n"
    )
}

