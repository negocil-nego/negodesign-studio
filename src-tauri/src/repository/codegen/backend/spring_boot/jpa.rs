use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{
        editable_columns, primary_key_column, sql_to_java_type, to_camel_case,
    },
};
use std::collections::BTreeMap;

pub fn generate_jpa_repository(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    full_pkg: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let pk = primary_key_column(table);
    let pk_type = sql_to_java_type(&pk.data_type);
    let pk_camel = to_camel_case(&pk.name);

    let mut entity_fields = String::new();
    for col in &table.columns {
        let jtype = sql_to_java_type(&col.data_type);
        let camel = to_camel_case(&col.name);
        if col.primary_key {
            entity_fields.push_str(&format!(
                "    @Id\n    @GeneratedValue(strategy = GenerationType.IDENTITY)\n    @Column(name = \"{}\")\n    private {jtype} {camel};\n\n",
                col.name
            ));
        } else {
            entity_fields.push_str(&format!(
                "    @Column(name = \"{}\", nullable = {})\n    private {jtype} {camel};\n\n",
                col.name, col.nullable
            ));
        }
    }

    let mut getters_setters = String::new();
    for col in &table.columns {
        let jtype = sql_to_java_type(&col.data_type);
        let camel = to_camel_case(&col.name);
        let mut chars = camel.chars();
        let cap = match chars.next() {
            Some(f) => f.to_ascii_uppercase().to_string() + chars.as_str(),
            None => String::new(),
        };
        getters_setters.push_str(&format!(
            "    public {jtype} get{cap}() {{ return this.{camel}; }}\n    public void set{cap}({jtype} {camel}) {{ this.{camel} = {camel}; }}\n"
        ));
    }

    let entity_src = format!(
        "package {full_pkg}.entity;\n\n\
import jakarta.persistence.*;\n\n\
@Entity\n\
@Table(name = \"{}\")\n\
public class {pascal}Entity {{\n\
{entity_fields}\
{getters_setters}\
}}\n",
        table.name
    );
    files.insert(format!("{base_dir}/entity/{pascal}Entity.java"), entity_src);

    let jpa_interface = format!(
        "package {full_pkg}.repository;\n\n\
import {full_pkg}.entity.{pascal}Entity;\n\
import org.springframework.data.jpa.repository.JpaRepository;\n\n\
public interface {pascal}JpaRepository extends JpaRepository<{pascal}Entity, {pk_type}> {{}}\n"
    );
    files.insert(
        format!("{base_dir}/repository/{pascal}JpaRepository.java"),
        jpa_interface,
    );

    let edit_cols = editable_columns(table);
    let setters = edit_cols
        .iter()
        .map(|c| {
            let camel = to_camel_case(&c.name);
            let mut chars = camel.chars();
            let cap = match chars.next() {
                Some(f) => f.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            };
            format!("        entity.set{cap}(request.{camel}());")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let response_args = table
        .columns
        .iter()
        .map(|c| {
            let camel = to_camel_case(&c.name);
            let mut chars = camel.chars();
            let cap = match chars.next() {
                Some(f) => f.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            };
            format!("entity.get{cap}()")
        })
        .collect::<Vec<_>>()
        .join(", ");

    let base_pkg = full_pkg
        .rsplit_once(".feature.")
        .map(|(b, _)| b)
        .unwrap_or(full_pkg);

    let repo_impl = format!(
        "package {full_pkg}.repository;\n\n\
import {base_pkg}.shared.PageResponse;\n\
import {full_pkg}.dto.{pascal}Request;\n\
import {full_pkg}.dto.{pascal}Response;\n\
import {full_pkg}.entity.{pascal}Entity;\n\
import org.springframework.data.domain.Page;\n\
import org.springframework.data.domain.PageRequest;\n\
import org.springframework.stereotype.Repository;\n\n\
@Repository\n\
public class {pascal}Repository {{\n\
    private final {pascal}JpaRepository jpaRepository;\n\n\
    public {pascal}Repository({pascal}JpaRepository jpaRepository) {{\n\
        this.jpaRepository = jpaRepository;\n\
    }}\n\n\
    public PageResponse<{pascal}Response> findAll(int page, int size) {{\n\
        Page<{pascal}Entity> result = jpaRepository.findAll(PageRequest.of(page, size));\n\
        return new PageResponse<>(\n\
            result.getContent().stream().map(this::toResponse).toList(),\n\
            result.getTotalElements(),\n\
            page,\n\
            size\n\
        );\n\
    }}\n\n\
    public {pascal}Response insert({pascal}Request request) {{\n\
        {pascal}Entity entity = new {pascal}Entity();\n\
{setters}\n\
        return toResponse(jpaRepository.save(entity));\n\
    }}\n\n\
    public {pascal}Response update({pk_type} {pk_camel}, {pascal}Request request) {{\n\
        {pascal}Entity entity = jpaRepository.findById({pk_camel})\n\
            .orElseThrow(() -> new IllegalArgumentException(\"{pascal} not found: \" + {pk_camel}));\n\
{setters}\n\
        return toResponse(jpaRepository.save(entity));\n\
    }}\n\n\
    public void delete({pk_type} {pk_camel}) {{\n\
        jpaRepository.deleteById({pk_camel});\n\
    }}\n\n\
    private {pascal}Response toResponse({pascal}Entity entity) {{\n\
        return new {pascal}Response({response_args});\n\
    }}\n\
}}\n"
    );
    files.insert(
        format!("{base_dir}/repository/{pascal}Repository.java"),
        repo_impl,
    );
}

