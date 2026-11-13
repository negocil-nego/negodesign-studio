use std::collections::BTreeMap;

pub fn generate_rbac_module(files: &mut BTreeMap<String, String>, base_package: &str) {
    let pkg_path = base_package.replace('.', "/");

    // 1. Permission.java
    files.insert(
        format!("src/main/java/{pkg_path}/modules/permission/model/Permission.java"),
        format!(
            r#"package {base_package}.modules.permission.model;

import {base_package}.modules.role.model.Role;
import {base_package}.shared.model.ConcreteModel;
import {base_package}.shared.utils.ConcreteTableModel;
import com.fasterxml.jackson.annotation.JsonIgnore;
import jakarta.persistence.*;
import jakarta.validation.constraints.NotBlank;
import lombok.*;
import lombok.experimental.SuperBuilder;

import java.util.HashSet;
import java.util.Set;

@Data
@Entity
@NoArgsConstructor
@AllArgsConstructor
@SuperBuilder(toBuilder = true)
@EqualsAndHashCode(callSuper = true, exclude = {{"roles"}})
@Table(name = ConcreteTableModel.PERMISSION)
public class Permission extends ConcreteModel {{
    @NotBlank
    @Column(unique = true, nullable = false)
    private String code;

    private String name;

    private String description;

    @Builder.Default
    @JsonIgnore
    @ManyToMany(mappedBy = "permissions", fetch = FetchType.LAZY)
    private Set<Role> roles = new HashSet<>();
}}
"#
        ),
    );

    // 2. PermissionRepository.java
    files.insert(
        format!("src/main/java/{pkg_path}/modules/permission/repository/PermissionRepository.java"),
        format!(
            r#"package {base_package}.modules.permission.repository;

import {base_package}.modules.permission.model.Permission;
import {base_package}.shared.repository.ConcreteRepository;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public interface PermissionRepository extends ConcreteRepository<Permission> {{
    Optional<Permission> findByCode(String code);
}}
"#
        ),
    );

    // 3. PermissionDTOs
    files.insert(
        format!("src/main/java/{pkg_path}/modules/permission/dto/PermissionRequest.java"),
        format!(
            r#"package {base_package}.modules.permission.dto;

import jakarta.validation.constraints.NotBlank;

public record PermissionRequest(
    @NotBlank String code,
    String name,
    String description
) {{}}
"#
        ),
    );

    files.insert(
        format!("src/main/java/{pkg_path}/modules/permission/dto/PermissionResponse.java"),
        format!(
            r#"package {base_package}.modules.permission.dto;

import {base_package}.modules.permission.model.Permission;

import java.time.Instant;
import java.util.UUID;

public record PermissionResponse(
    UUID uuid,
    String code,
    String name,
    String description,
    Instant createdAt,
    Instant updatedAt
) {{
    public static PermissionResponse of(Permission entity) {{
        return new PermissionResponse(
            entity.getUuid(),
            entity.getCode(),
            entity.getName(),
            entity.getDescription(),
            entity.getCreatedAt(),
            entity.getUpdatedAt()
        );
    }}
}}
"#
        ),
    );

    // 4. PermissionService.java
    files.insert(
        format!("src/main/java/{pkg_path}/modules/permission/service/PermissionService.java"),
        format!(
            r#"package {base_package}.modules.permission.service;

import {base_package}.modules.permission.model.Permission;
import {base_package}.modules.permission.repository.PermissionRepository;
import {base_package}.shared.service.ConcreteService;
import org.springframework.stereotype.Service;

@Service
public class PermissionService extends ConcreteService<Permission> {{
    public PermissionService(PermissionRepository repository) {{
        super(repository);
    }}
}}
"#
        ),
    );

    // 5. PermissionController.java
    files.insert(
        format!("src/main/java/{pkg_path}/modules/permission/controller/PermissionController.java"),
        format!(
            r#"package {base_package}.modules.permission.controller;

import {base_package}.modules.permission.dto.PermissionRequest;
import {base_package}.modules.permission.dto.PermissionResponse;
import {base_package}.modules.permission.model.Permission;
import {base_package}.modules.permission.service.PermissionService;
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
@RequestMapping("/api/v1/admin/permissions")
@Tag(name = "Permissions", description = "Endpoints para gestão de permissões")
public class PermissionController {{
    private final PermissionService service;

    @GetMapping
    @Operation(operationId = "listPermissions", summary = "Listar todas as permissões paginadas")
    @CanPermission("PERMISSION:READ")
    public ResponseEntity<PageResponse<PermissionResponse>> findAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size) {{
        Page<Permission> result = service.findAll(PageRequest.of(page, size));
        return ResponseEntity.ok(new PageResponse<>(
                result.getContent().stream().map(PermissionResponse::of).toList(),
                result.getTotalElements(),
                page,
                size
        ));
    }}

    @GetMapping("/{{uuid}}")
    @Operation(operationId = "getPermissionByUuid", summary = "Obter permissão por UUID")
    @CanPermission("PERMISSION:READ")
    public ResponseEntity<PermissionResponse> findByUuid(@PathVariable UUID uuid) {{
        return ResponseEntity.ok(PermissionResponse.of(service.findByUuid(uuid)));
    }}

    @PostMapping
    @Operation(operationId = "createPermission", summary = "Criar nova permissão")
    @CanPermission("PERMISSION:CREATE")
    public ResponseEntity<PermissionResponse> save(@RequestBody @Valid PermissionRequest request) {{
        Permission entity = Permission.builder()
                .code(request.code())
                .name(request.name())
                .description(request.description())
                .build();
        return new ResponseEntity<>(PermissionResponse.of(service.save(entity)), HttpStatus.CREATED);
    }}

    @PutMapping("/{{uuid}}")
    @Operation(operationId = "updatePermission", summary = "Actualizar permissão")
    @CanPermission("PERMISSION:UPDATE")
    public ResponseEntity<PermissionResponse> update(@PathVariable UUID uuid, @RequestBody @Valid PermissionRequest request) {{
        Permission entity = Permission.builder()
                .code(request.code())
                .name(request.name())
                .description(request.description())
                .build();
        return new ResponseEntity<>(PermissionResponse.of(service.update(uuid, entity)), HttpStatus.ACCEPTED);
    }}

    @DeleteMapping("/{{uuid}}")
    @Operation(operationId = "deletePermission", summary = "Remover permissão por UUID")
    @CanPermission("PERMISSION:DELETE")
    public ResponseEntity<Void> deleteByUuid(@PathVariable UUID uuid) {{
        service.deleteByUuid(uuid);
        return ResponseEntity.noContent().build();
    }}
}}
"#
        ),
    );

    // 6. Role.java
    files.insert(
        format!("src/main/java/{pkg_path}/modules/role/model/Role.java"),
        format!(
            r#"package {base_package}.modules.role.model;

import {base_package}.modules.permission.model.Permission;
import {base_package}.shared.model.ConcreteModel;
import {base_package}.shared.utils.ConcreteTableModel;
import {base_package}.shared.utils.PivotTableModel;
import jakarta.persistence.*;
import jakarta.validation.constraints.NotBlank;
import lombok.*;
import lombok.experimental.SuperBuilder;

import java.util.HashSet;
import java.util.Set;

@Data
@Entity
@NoArgsConstructor
@AllArgsConstructor
@SuperBuilder(toBuilder = true)
@EqualsAndHashCode(callSuper = true, exclude = {{"permissions"}})
@Table(name = ConcreteTableModel.ROLE)
public class Role extends ConcreteModel {{
    @NotBlank
    @Column(unique = true, nullable = false)
    private String code;

    private String name;

    private String description;

    @ManyToMany(fetch = FetchType.LAZY)
    @JoinTable(
            name = PivotTableModel.TB_ROLES_PERMISSIONS,
            joinColumns = @JoinColumn(name = "role_id"),
            inverseJoinColumns = @JoinColumn(name = "permission_id")
    )
    @Builder.Default
    private Set<Permission> permissions = new HashSet<>();
}}
"#
        ),
    );

    // 7. RoleRepository.java
    files.insert(
        format!("src/main/java/{pkg_path}/modules/role/repository/RoleRepository.java"),
        format!(
            r#"package {base_package}.modules.role.repository;

import {base_package}.modules.role.model.Role;
import {base_package}.shared.repository.ConcreteRepository;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public interface RoleRepository extends ConcreteRepository<Role> {{
    Optional<Role> findByCode(String code);
}}
"#
        ),
    );

    // 8. RoleDTOs
    files.insert(
        format!("src/main/java/{pkg_path}/modules/role/dto/RoleRequest.java"),
        format!(
            r#"package {base_package}.modules.role.dto;

import jakarta.validation.constraints.NotBlank;
import java.util.Set;
import java.util.UUID;

public record RoleRequest(
    @NotBlank String code,
    String name,
    String description,
    Set<UUID> permissionUuids
) {{}}
"#
        ),
    );

    files.insert(
        format!("src/main/java/{pkg_path}/modules/role/dto/RoleResponse.java"),
        format!(
            r#"package {base_package}.modules.role.dto;

import {base_package}.modules.permission.dto.PermissionResponse;
import {base_package}.modules.role.model.Role;

import java.time.Instant;
import java.util.Set;
import java.util.UUID;
import java.util.stream.Collectors;

public record RoleResponse(
    UUID uuid,
    String code,
    String name,
    String description,
    Set<PermissionResponse> permissions,
    Instant createdAt,
    Instant updatedAt
) {{
    public static RoleResponse of(Role entity) {{
        Set<PermissionResponse> perms = entity.getPermissions() != null
                ? entity.getPermissions().stream().map(PermissionResponse::of).collect(Collectors.toSet())
                : Set.of();

        return new RoleResponse(
            entity.getUuid(),
            entity.getCode(),
            entity.getName(),
            entity.getDescription(),
            perms,
            entity.getCreatedAt(),
            entity.getUpdatedAt()
        );
    }}
}}
"#
        ),
    );

    // 9. RoleService.java
    files.insert(
        format!("src/main/java/{pkg_path}/modules/role/service/RoleService.java"),
        format!(
            r#"package {base_package}.modules.role.service;

import {base_package}.modules.permission.repository.PermissionRepository;
import {base_package}.modules.role.dto.RoleRequest;
import {base_package}.modules.role.model.Role;
import {base_package}.modules.role.repository.RoleRepository;
import {base_package}.shared.service.ConcreteService;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.HashSet;
import java.util.UUID;

@Service
public class RoleService extends ConcreteService<Role> {{
    private final RoleRepository roleRepository;
    private final PermissionRepository permissionRepository;

    public RoleService(RoleRepository repository, PermissionRepository permissionRepository) {{
        super(repository);
        this.roleRepository = repository;
        this.permissionRepository = permissionRepository;
    }}

    @Transactional
    public Role createRole(RoleRequest request) {{
        Role role = Role.builder()
                .code(request.code())
                .name(request.name())
                .description(request.description())
                .build();

        if (request.permissionUuids() != null && !request.permissionUuids().isEmpty()) {{
            role.setPermissions(new HashSet<>(permissionRepository.findByUuidIn(request.permissionUuids().stream().toList())));
        }}

        return save(role);
    }}

    @Transactional
    public Role updateRole(UUID uuid, RoleRequest request) {{
        Role role = findByUuid(uuid);
        role.setCode(request.code());
        role.setName(request.name());
        role.setDescription(request.description());

        if (request.permissionUuids() != null) {{
            role.setPermissions(new HashSet<>(permissionRepository.findByUuidIn(request.permissionUuids().stream().toList())));
        }}

        return save(role);
    }}
}}
"#
        ),
    );

    // 10. RoleController.java
    files.insert(
        format!("src/main/java/{pkg_path}/modules/role/controller/RoleController.java"),
        format!(
            r#"package {base_package}.modules.role.controller;

import {base_package}.modules.role.dto.RoleRequest;
import {base_package}.modules.role.dto.RoleResponse;
import {base_package}.modules.role.model.Role;
import {base_package}.modules.role.service.RoleService;
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
@RequestMapping("/api/v1/admin/roles")
@Tag(name = "Roles", description = "Endpoints para gestão de perfis e funções")
public class RoleController {{
    private final RoleService service;

    @GetMapping
    @Operation(operationId = "listRoles", summary = "Listar todas as roles paginadas")
    @CanPermission("ROLE:READ")
    public ResponseEntity<PageResponse<RoleResponse>> findAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size) {{
        Page<Role> result = service.findAll(PageRequest.of(page, size));
        return ResponseEntity.ok(new PageResponse<>(
                result.getContent().stream().map(RoleResponse::of).toList(),
                result.getTotalElements(),
                page,
                size
        ));
    }}

    @GetMapping("/{{uuid}}")
    @Operation(operationId = "getRoleByUuid", summary = "Obter role por UUID")
    @CanPermission("ROLE:READ")
    public ResponseEntity<RoleResponse> findByUuid(@PathVariable UUID uuid) {{
        return ResponseEntity.ok(RoleResponse.of(service.findByUuid(uuid)));
    }}

    @PostMapping
    @Operation(operationId = "createRole", summary = "Criar nova role")
    @CanPermission("ROLE:CREATE")
    public ResponseEntity<RoleResponse> save(@RequestBody @Valid RoleRequest request) {{
        return new ResponseEntity<>(RoleResponse.of(service.createRole(request)), HttpStatus.CREATED);
    }}

    @PutMapping("/{{uuid}}")
    @Operation(operationId = "updateRole", summary = "Actualizar role")
    @CanPermission("ROLE:UPDATE")
    public ResponseEntity<RoleResponse> update(@PathVariable UUID uuid, @RequestBody @Valid RoleRequest request) {{
        return new ResponseEntity<>(RoleResponse.of(service.updateRole(uuid, request)), HttpStatus.ACCEPTED);
    }}

    @DeleteMapping("/{{uuid}}")
    @Operation(operationId = "deleteRole", summary = "Remover role por UUID")
    @CanPermission("ROLE:DELETE")
    public ResponseEntity<Void> deleteByUuid(@PathVariable UUID uuid) {{
        service.deleteByUuid(uuid);
        return ResponseEntity.noContent().build();
    }}
}}
"#
        ),
    );
}

