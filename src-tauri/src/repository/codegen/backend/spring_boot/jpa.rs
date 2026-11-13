use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{
        editable_columns, sql_to_java_type, to_camel_case, to_kebab_case, to_pascal_case,
        to_snake_case,
    },
};
use std::collections::BTreeMap;

fn is_inherited_column(name: &str) -> bool {
    let lower = name.to_ascii_lowercase().replace('_', "");
    matches!(
        lower.as_str(),
        "id" | "uuid" | "createdat" | "updatedat" | "createdby" | "updatedby" | "deletedby" | "deletedat"
    )
}

pub fn generate_jpa_module(
    files: &mut BTreeMap<String, String>,
    base_package: &str,
    table: &DatabaseTable,
) {
    let lower_name = table.name.to_ascii_lowercase();
    if matches!(
        lower_name.as_str(),
        "role" | "roles" | "tb_roles" | "permission" | "permissions" | "tb_permissions"
    ) {
        return;
    }

    let pkg_path = base_package.replace('.', "/");
    let feature_name = to_snake_case(&table.name);
    let pascal = to_pascal_case(&table.name);
    let kebab = to_kebab_case(&table.name);
    let upper_snake = to_snake_case(&table.name).to_ascii_uppercase();

    let module_pkg = format!("{base_package}.modules.{feature_name}");
    let module_dir = format!("src/main/java/{pkg_path}/modules/{feature_name}");

    // Filter columns not inherited from ConcreteModel
    let own_cols: Vec<_> = table
        .columns
        .iter()
        .filter(|c| !is_inherited_column(&c.name))
        .collect();

    // 1. Entity
    let mut fields_str = String::new();
    for col in &own_cols {
        let jtype = sql_to_java_type(&col.data_type);
        let camel = to_camel_case(&col.name);
        if col.nullable {
            fields_str.push_str(&format!(
                "    @Column(name = \"{}\")\n    private {} {};\n\n",
                col.name, jtype, camel
            ));
        } else {
            fields_str.push_str(&format!(
                "    @Column(name = \"{}\", nullable = false)\n    private {} {};\n\n",
                col.name, jtype, camel
            ));
        }
    }

    let all_args = if own_cols.is_empty() {
        ""
    } else {
        "@AllArgsConstructor\n"
    };

    let entity_code = format!(
        r#"package {module_pkg}.model;

import {base_package}.shared.model.ConcreteModel;
import {base_package}.shared.utils.ConcreteTableModel;
import jakarta.persistence.*;
import lombok.*;
import lombok.experimental.SuperBuilder;

@Data
@Entity
@NoArgsConstructor
{all_args}@SuperBuilder(toBuilder = true)
@EqualsAndHashCode(callSuper = true)
@Table(name = ConcreteTableModel.{upper_snake})
public class {pascal} extends ConcreteModel {{
{fields_str}}}
"#
    );
    files.insert(format!("{module_dir}/model/{pascal}.java"), entity_code);

    // 2. Repository
    let repo_code = format!(
        r#"package {module_pkg}.repository;

import {module_pkg}.model.{pascal};
import {base_package}.shared.repository.ConcreteRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface {pascal}Repository extends ConcreteRepository<{pascal}> {{
}}
"#
    );
    files.insert(format!("{module_dir}/repository/{pascal}Repository.java"), repo_code);

    // 3. Service
    let service_code = format!(
        r#"package {module_pkg}.service;

import {module_pkg}.model.{pascal};
import {module_pkg}.repository.{pascal}Repository;
import {base_package}.shared.service.ConcreteService;

@org.springframework.stereotype.Service
public class {pascal}Service extends ConcreteService<{pascal}> {{
    public {pascal}Service({pascal}Repository repository) {{
        super(repository);
    }}
}}
"#
    );
    files.insert(format!("{module_dir}/service/{pascal}Service.java"), service_code);

    // 4. Request DTO
    let edit_cols: Vec<_> = editable_columns(table)
        .into_iter()
        .filter(|c| !is_inherited_column(&c.name))
        .collect();

    let req_fields = if edit_cols.is_empty() {
        "    String name".to_string()
    } else {
        edit_cols
            .iter()
            .map(|c| format!("    {} {}", sql_to_java_type(&c.data_type), to_camel_case(&c.name)))
            .collect::<Vec<_>>()
            .join(",\n")
    };

    let req_dto = format!(
        r#"package {module_pkg}.dto;

public record {pascal}Request(
{req_fields}
) {{}}
"#
    );
    files.insert(format!("{module_dir}/dto/{pascal}Request.java"), req_dto);

    // 5. Response DTO
    let mut resp_record_fields = vec![
        "    java.util.UUID uuid".to_string(),
    ];
    for col in &own_cols {
        resp_record_fields.push(format!(
            "    {} {}",
            sql_to_java_type(&col.data_type),
            to_camel_case(&col.name)
        ));
    }
    resp_record_fields.push("    java.time.Instant createdAt".to_string());
    resp_record_fields.push("    java.time.Instant updatedAt".to_string());

    let mut of_args = vec!["entity.getUuid()".to_string()];
    for col in &own_cols {
        let camel = to_camel_case(&col.name);
        let mut chars = camel.chars();
        let cap = match chars.next() {
            Some(f) => f.to_ascii_uppercase().to_string() + chars.as_str(),
            None => String::new(),
        };
        of_args.push(format!("entity.get{cap}()"));
    }
    of_args.push("entity.getCreatedAt()".to_string());
    of_args.push("entity.getUpdatedAt()".to_string());

    let resp_dto = format!(
        r#"package {module_pkg}.dto;

import {module_pkg}.model.{pascal};

public record {pascal}Response(
{}
) {{
    public static {pascal}Response of({pascal} entity) {{
        return new {pascal}Response(
            {}
        );
    }}
}}
"#,
        resp_record_fields.join(",\n"),
        of_args.join(",\n            ")
    );
    files.insert(format!("{module_dir}/dto/{pascal}Response.java"), resp_dto);

    // 6. Controller
    let mut to_entity_setters = String::new();
    for col in &edit_cols {
        let camel = to_camel_case(&col.name);
        to_entity_setters.push_str(&format!("                .{camel}(request.{camel}())\n"));
    }

    let controller_code = format!(
        r#"package {module_pkg}.controller;

import {module_pkg}.dto.{pascal}Request;
import {module_pkg}.dto.{pascal}Response;
import {module_pkg}.model.{pascal};
import {module_pkg}.service.{pascal}Service;
import {base_package}.shared.PageResponse;
import {base_package}.shared.annotation.CanPermission;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.UUID;

@RestController
@RequiredArgsConstructor
@RequestMapping("/api/v1/admin/{kebab}")
@Tag(name = "{pascal}", description = "Endpoints para gestão de {kebab}")
public class {pascal}Controller {{
    private final {pascal}Service service;

    @GetMapping
    @Operation(operationId = "list{pascal}", summary = "Listar registos paginados")
    @CanPermission("{upper_snake}:READ")
    public ResponseEntity<PageResponse<{pascal}Response>> findAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size) {{
        Page<{pascal}> result = service.findAll(PageRequest.of(page, size));
        return ResponseEntity.ok(new PageResponse<>(
                result.getContent().stream().map({pascal}Response::of).toList(),
                result.getTotalElements(),
                page,
                size
        ));
    }}

    @GetMapping("/{{uuid}}")
    @Operation(operationId = "get{pascal}ByUuid", summary = "Obter registo por UUID")
    @CanPermission("{upper_snake}:READ")
    public ResponseEntity<{pascal}Response> findByUuid(@PathVariable UUID uuid) {{
        return ResponseEntity.ok({pascal}Response.of(service.findByUuid(uuid)));
    }}

    @PostMapping
    @Operation(operationId = "create{pascal}", summary = "Criar novo registo")
    @CanPermission("{upper_snake}:CREATE")
    public ResponseEntity<{pascal}Response> save(@RequestBody @Valid {pascal}Request request) {{
        {pascal} entity = {pascal}.builder()
{to_entity_setters}                .build();
        return new ResponseEntity<>({pascal}Response.of(service.save(entity)), HttpStatus.CREATED);
    }}

    @PutMapping("/{{uuid}}")
    @Operation(operationId = "update{pascal}", summary = "Actualizar registo por UUID")
    @CanPermission("{upper_snake}:UPDATE")
    public ResponseEntity<{pascal}Response> update(@PathVariable UUID uuid, @RequestBody @Valid {pascal}Request request) {{
        {pascal} entity = {pascal}.builder()
{to_entity_setters}                .build();
        return new ResponseEntity<>({pascal}Response.of(service.update(uuid, entity)), HttpStatus.ACCEPTED);
    }}

    @DeleteMapping("/{{uuid}}")
    @Operation(operationId = "delete{pascal}", summary = "Remover registo por UUID")
    @CanPermission("{upper_snake}:DELETE")
    public ResponseEntity<Void> deleteByUuid(@PathVariable UUID uuid) {{
        service.deleteByUuid(uuid);
        return ResponseEntity.noContent().build();
    }}
}}
"#
    );
    files.insert(format!("{module_dir}/controller/{pascal}Controller.java"), controller_code);
}
