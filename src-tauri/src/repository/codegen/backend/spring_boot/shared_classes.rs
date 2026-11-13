use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::to_snake_case,
};
use std::collections::{BTreeMap, BTreeSet};

/// Nome real da tabela RBAC na BD (ex.: "tb_roles"), ou o default legado.
fn rbac_table_name(tables: &[DatabaseTable], stems: &[&str], default: &str) -> String {
    tables
        .iter()
        .find(|table| stems.contains(&to_snake_case(&table.name).as_str()))
        .map(|table| table.name.clone())
        .unwrap_or_else(|| default.to_string())
}

/// Valida que todas as referências `ConcreteTableModel.X` existem mesmo na
/// classe gerada — evita entidades a usar variáveis que não foram definidas
/// (ex.: `ConcreteTableModel.USERS` sem a constante `USERS`).
///
/// Quando a constante em falta corresponde a uma tabela real, completa o
/// `ConcreteTableModel` automaticamente; se não corresponder a nenhuma,
/// devolve um erro claro em vez de gerar código que não compila.
pub fn validate_table_model_references(
    files: &mut BTreeMap<String, String>,
    tables: &[DatabaseTable],
) -> Result<(), String> {
    const MODEL_SUFFIX: &str = "shared/utils/ConcreteTableModel.java";
    let Some(model_key) = files
        .keys()
        .find(|key| key.ends_with(MODEL_SUFFIX))
        .cloned()
    else {
        return Ok(());
    };

    let mut defined: Vec<String> = files[&model_key]
        .lines()
        .filter_map(|line| line.trim().strip_prefix("public static final String "))
        .filter_map(|rest| rest.split('=').next())
        .map(|name| name.trim().to_string())
        .collect();

    let mut missing: BTreeSet<String> = BTreeSet::new();
    for (path, content) in files.iter() {
        if *path == model_key || !path.ends_with(".java") {
            continue;
        }
        let mut rest = content.as_str();
        while let Some(index) = rest.find("ConcreteTableModel.") {
            rest = &rest[index + "ConcreteTableModel.".len()..];
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_uppercase() || *c == '_')
                .collect();
            if !name.is_empty() && !defined.iter().any(|item| item == &name) {
                missing.insert(name);
            }
        }
    }

    if missing.is_empty() {
        return Ok(());
    }

    let mut source = files[&model_key].clone();
    let mut unresolved: Vec<String> = Vec::new();
    for name in missing {
        match tables
            .iter()
            .find(|table| to_snake_case(&table.name).to_ascii_uppercase() == name)
        {
            Some(table) => {
                let line = format!(
                    "    public static final String {name} = \"{}\";\n",
                    table.name
                );
                match source.rfind('}') {
                    Some(index) => source.insert_str(index, &line),
                    None => source.push_str(&line),
                }
                defined.push(name);
            }
            None => unresolved.push(name),
        }
    }

    if !unresolved.is_empty() {
        return Err(format!(
            "ConcreteTableModel: constantes referenciadas sem definição e sem tabela correspondente: {}",
            unresolved.join(", ")
        ));
    }

    files.insert(model_key, source);
    Ok(())
}

pub fn generate_shared_classes(
    files: &mut BTreeMap<String, String>,
    base_package: &str,
    tables: &[DatabaseTable],
) {
    let pkg_path = base_package.replace('.', "/");

    // 1. CommonModel.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/model/CommonModel.java"),
        format!(
            r#"package {base_package}.shared.model;

import jakarta.persistence.*;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.experimental.SuperBuilder;
import org.hibernate.annotations.CreationTimestamp;
import org.hibernate.annotations.UpdateTimestamp;

import java.time.Instant;

@Data
@MappedSuperclass
@NoArgsConstructor
@AllArgsConstructor
@SuperBuilder(toBuilder = true)
public class CommonModel {{
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(updatable = false, nullable = false)
    protected long id;

    @CreationTimestamp
    @Column(nullable = false, updatable = false)
    protected Instant createdAt;

    @UpdateTimestamp
    @Column(nullable = false)
    protected Instant updatedAt;

    public long getId() {{ return id; }}
    public void setId(long id) {{ this.id = id; }}
    public Instant getCreatedAt() {{ return createdAt; }}
    public void setCreatedAt(Instant createdAt) {{ this.createdAt = createdAt; }}
    public Instant getUpdatedAt() {{ return updatedAt; }}
    public void setUpdatedAt(Instant updatedAt) {{ this.updatedAt = updatedAt; }}
}}
"#
        ),
    );

    // 2. ConcreteModel.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/model/ConcreteModel.java"),
        format!(
            r#"package {base_package}.shared.model;

import jakarta.persistence.Column;
import jakarta.persistence.MappedSuperclass;
import jakarta.persistence.PrePersist;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.experimental.SuperBuilder;
import org.hibernate.annotations.SQLRestriction;

import java.time.Instant;
import java.util.UUID;

@Data
@MappedSuperclass
@NoArgsConstructor
@AllArgsConstructor
@SuperBuilder(toBuilder = true)
@EqualsAndHashCode(callSuper = true)
@SQLRestriction(value = "deleted_at IS NULL AND deleted_by IS NULL")
public class ConcreteModel extends CommonModel {{
    @Column(nullable = false, unique = true, updatable = false)
    protected UUID uuid;
    protected UUID createdBy;
    protected UUID updatedBy;
    protected UUID deletedBy;
    protected Instant deletedAt;

    @PrePersist
    public void onCreate() {{
        if (uuid == null) {{
            uuid = UUID.randomUUID();
        }}
    }}

    public UUID getUuid() {{ return uuid; }}
    public void setUuid(UUID uuid) {{ this.uuid = uuid; }}
    public UUID getCreatedBy() {{ return createdBy; }}
    public void setCreatedBy(UUID createdBy) {{ this.createdBy = createdBy; }}
    public UUID getUpdatedBy() {{ return updatedBy; }}
    public void setUpdatedBy(UUID updatedBy) {{ this.updatedBy = updatedBy; }}
    public UUID getDeletedBy() {{ return deletedBy; }}
    public void setDeletedBy(UUID deletedBy) {{ this.deletedBy = deletedBy; }}
    public Instant getDeletedAt() {{ return deletedAt; }}
    public void setDeletedAt(Instant deletedAt) {{ this.deletedAt = deletedAt; }}
}}
"#
        ),
    );

    // 3. ConcreteTableModel.java
    //
    // As constantes RBAC vêm do nome REAL da tabela na BD quando ela existe
    // (ex.: "tb_roles" em vez do default "TB_ROLES"); sem tabela seleccionada,
    // mantêm-se os defaults do esquema legado.
    let role_table = rbac_table_name(tables, &["role", "roles"], "TB_ROLES");
    let permission_table = rbac_table_name(
        tables,
        &["permission", "permissions"],
        "TB_PERMISSIONS",
    );
    let user_table = rbac_table_name(tables, &["user", "users"], "TB_USERS");

    let mut table_constants = String::new();
    table_constants.push_str(&format!(
        "    public static final String ROLE = \"{role_table}\";\n"
    ));
    table_constants.push_str(&format!(
        "    public static final String PERMISSION = \"{permission_table}\";\n"
    ));
    table_constants.push_str(&format!(
        "    public static final String USER = \"{user_table}\";\n"
    ));

    for t in tables {
        let upper_snake = to_snake_case(&t.name).to_ascii_uppercase();
        // Exclui apenas nomes que colidem com as constantes RBAC acima (ROLE,
        // PERMISSION, USER) ou cujas entidades não são geradas (role/permission
        // são geradas pelo módulo rbac). USERS NÃO é excluído: a entidade
        // gerada pelo jpa referencia ConcreteTableModel.USERS e a constante
        // tem de existir.
        if upper_snake != "ROLES"
            && upper_snake != "PERMISSIONS"
            && upper_snake != "ROLE"
            && upper_snake != "PERMISSION"
            && upper_snake != "USER"
        {
            table_constants.push_str(&format!(
                "    public static final String {} = \"{}\";\n",
                upper_snake, t.name
            ));
        }
    }

    files.insert(
        format!("src/main/java/{pkg_path}/shared/utils/ConcreteTableModel.java"),
        format!(
            r#"package {base_package}.shared.utils;

public final class ConcreteTableModel {{
    private ConcreteTableModel() {{}}

{table_constants}}}
"#
        ),
    );

    // 4. PivotTableModel.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/utils/PivotTableModel.java"),
        format!(
            r#"package {base_package}.shared.utils;

public final class PivotTableModel {{
    private PivotTableModel() {{}}

    public static final String TB_ROLES_PERMISSIONS = "TB_ROLES_PERMISSIONS";
    public static final String TB_USERS_ROLES = "TB_USERS_ROLES";
}}
"#
        ),
    );

    // 5. CommonRepository.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/repository/CommonRepository.java"),
        format!(
            r#"package {base_package}.shared.repository;

import {base_package}.shared.model.CommonModel;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.JpaSpecificationExecutor;
import org.springframework.data.repository.NoRepositoryBean;

@NoRepositoryBean
public interface CommonRepository<T extends CommonModel> extends JpaRepository<T, Long>, JpaSpecificationExecutor<T> {{
}}
"#
        ),
    );

    // 6. ConcreteRepository.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/repository/ConcreteRepository.java"),
        format!(
            r#"package {base_package}.shared.repository;

import {base_package}.shared.model.ConcreteModel;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.repository.NoRepositoryBean;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

@NoRepositoryBean
public interface ConcreteRepository<T extends ConcreteModel> extends CommonRepository<T> {{
    Optional<T> findByUuid(UUID uuid);

    List<T> findByUuidIn(List<UUID> uuids);

    @Modifying
    @Transactional
    void deleteByUuid(UUID uuid);
}}
"#
        ),
    );

    // 7. NotFoundException.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/exception/NotFoundException.java"),
        format!(
            r#"package {base_package}.shared.exception;

import java.util.UUID;

public class NotFoundException extends RuntimeException {{
    public NotFoundException() {{
        super("Recurso não encontrado");
    }}

    public NotFoundException(long id) {{
        super("Recurso não encontrado com id: " + id);
    }}

    public NotFoundException(UUID uuid) {{
        super("Recurso não encontrado com uuid: " + uuid);
    }}

    public NotFoundException(String message) {{
        super(message);
    }}
}}
"#
        ),
    );

    // 8. CommonService.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/service/CommonService.java"),
        format!(
            r#"package {base_package}.shared.service;

import {base_package}.shared.exception.NotFoundException;
import {base_package}.shared.model.CommonModel;
import {base_package}.shared.repository.CommonRepository;
import lombok.AllArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.domain.Specification;

public abstract class CommonService<T extends CommonModel> {{
    private final CommonRepository<T> repository;

    public CommonService(CommonRepository<T> repository) {{
        this.repository = repository;
    }}

    public T findById(long id) {{
        return repository.findById(id).orElseThrow(() -> new NotFoundException(id));
    }}

    public Page<T> findAll(Specification<T> spec, Pageable pageable) {{
        return repository.findAll(spec, pageable);
    }}

    public Page<T> findAll(Pageable pageable) {{
        return repository.findAll(pageable);
    }}

    public T save(T data) {{
        return repository.save(data);
    }}

    public T update(long id, T data) {{
        var item = repository.findById(id).orElseThrow(NotFoundException::new);
        data.setId(item.getId());
        return save(data);
    }}

    public boolean deleteAll() {{
        repository.deleteAll();
        return true;
    }}
}}
"#
        ),
    );

    // 9. ConcreteService.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/service/ConcreteService.java"),
        format!(
            r#"package {base_package}.shared.service;

import {base_package}.shared.exception.NotFoundException;
import {base_package}.shared.model.ConcreteModel;
import {base_package}.shared.repository.ConcreteRepository;
import org.springframework.beans.BeanUtils;

import java.time.Instant;
import java.util.UUID;

public abstract class ConcreteService<T extends ConcreteModel> extends CommonService<T> {{
    private final ConcreteRepository<T> repository;

    public ConcreteService(ConcreteRepository<T> repository) {{
        super(repository);
        this.repository = repository;
    }}

    public T findByUuid(UUID uuid) {{
        return repository.findByUuid(uuid).orElseThrow(() -> new NotFoundException(uuid));
    }}

    public T update(UUID uuid, T data) {{
        var item = findByUuid(uuid);
        BeanUtils.copyProperties(data, item, "id", "uuid", "createdAt");
        return save(item);
    }}

    @Override
    public T update(long id, T data) {{
        var item = findById(id);
        BeanUtils.copyProperties(data, item, "id", "uuid", "createdAt");
        return save(item);
    }}

    public boolean deleteByUuid(UUID uuid) {{
        var item = findByUuid(uuid);
        item.setDeletedAt(Instant.now());
        super.save(item);
        return true;
    }}
}}
"#
        ),
    );

    // 10. CanPermission.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/annotation/CanPermission.java"),
        format!(
            r#"package {base_package}.shared.annotation;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

@Retention(RetentionPolicy.RUNTIME)
@Target({{ElementType.METHOD, ElementType.TYPE}})
public @interface CanPermission {{
    String value();
}}
"#
        ),
    );

    // 11. CanPermissionAspect.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/aspect/CanPermissionAspect.java"),
        format!(
            r#"package {base_package}.shared.aspect;

import {base_package}.shared.annotation.CanPermission;
import lombok.RequiredArgsConstructor;
import org.aspectj.lang.JoinPoint;
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.annotation.Before;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.security.authentication.AuthenticationCredentialsNotFoundException;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Component;

@Aspect
@Component
@RequiredArgsConstructor
public class CanPermissionAspect {{
    @Before("@annotation(canPermission)")
    public void checkPermission(JoinPoint joinPoint, CanPermission canPermission) {{
        Authentication authentication = SecurityContextHolder.getContext().getAuthentication();

        if (authentication == null || !authentication.isAuthenticated()) {{
            throw new AuthenticationCredentialsNotFoundException("User is not authenticated");
        }}

        String required = canPermission.value();
        boolean hasPermission = authentication.getAuthorities().stream()
                .anyMatch(a -> a.getAuthority().equals(required));

        if (!hasPermission) {{
            throw new AccessDeniedException("User does not have required authority: " + required);
        }}
    }}
}}
"#
        ),
    );

    // 12. PageResponse.java
    files.insert(
        format!("src/main/java/{pkg_path}/shared/PageResponse.java"),
        format!(
            r#"package {base_package}.shared;

import java.util.List;

public record PageResponse<T>(
    List<T> data,
    long total,
    int page,
    int size
) {{}}
"#
        ),
    );

    // 13. SecurityConfig.java
    files.insert(
        format!("src/main/java/{pkg_path}/config/SecurityConfig.java"),
        format!(
            r#"package {base_package}.config;

import com.nimbusds.jose.jwk.JWK;
import com.nimbusds.jose.jwk.JWKSet;
import com.nimbusds.jose.jwk.RSAKey;
import com.nimbusds.jose.jwk.source.ImmutableJWKSet;
import com.nimbusds.jose.jwk.source.JWKSource;
import com.nimbusds.jose.proc.SecurityContext;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.config.Customizer;
import org.springframework.security.config.annotation.authentication.configuration.AuthenticationConfiguration;
import org.springframework.security.config.annotation.method.configuration.EnableMethodSecurity;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.annotation.web.configurers.AbstractHttpConfigurer;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.security.oauth2.jwt.JwtDecoder;
import org.springframework.security.oauth2.jwt.JwtEncoder;
import org.springframework.security.oauth2.jwt.NimbusJwtDecoder;
import org.springframework.security.oauth2.jwt.NimbusJwtEncoder;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.web.cors.CorsConfiguration;
import org.springframework.web.cors.CorsConfigurationSource;
import org.springframework.web.cors.UrlBasedCorsConfigurationSource;

import java.security.interfaces.RSAPrivateKey;
import java.security.interfaces.RSAPublicKey;
import java.util.List;

@Configuration
@EnableWebSecurity
@EnableMethodSecurity
public class SecurityConfig {{

    @Value("${{rsa.public-key}}")
    private RSAPublicKey publicKey;

    @Value("${{rsa.private-key}}")
    private RSAPrivateKey privateKey;

    @Value("${{app.cors.allowed-origins:http://localhost:5173}}")
    private List<String> allowedOrigins;

    @Bean
    public PasswordEncoder passwordEncoder() {{
        return new BCryptPasswordEncoder();
    }}

    @Bean
    public AuthenticationManager authenticationManager(AuthenticationConfiguration config) throws Exception {{
        return config.getAuthenticationManager();
    }}

    @Bean
    public CorsConfigurationSource corsConfigurationSource() {{
        CorsConfiguration config = new CorsConfiguration();
        config.setAllowedOrigins(allowedOrigins);
        config.setAllowedMethods(List.of("GET", "POST", "PUT", "DELETE", "OPTIONS"));
        config.setAllowedHeaders(List.of("*"));
        config.setAllowCredentials(true);

        UrlBasedCorsConfigurationSource source = new UrlBasedCorsConfigurationSource();
        source.registerCorsConfiguration("/**", config);
        return source;
    }}

    @Bean
    public JwtDecoder jwtDecoder() {{
        return NimbusJwtDecoder.withPublicKey(publicKey).build();
    }}

    @Bean
    public JwtEncoder jwtEncoder() {{
        JWK jwk = new RSAKey.Builder(publicKey).privateKey(privateKey).build();
        JWKSource<SecurityContext> jwks = new ImmutableJWKSet<>(new JWKSet(jwk));
        return new NimbusJwtEncoder(jwks);
    }}

    @Bean
    public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {{
        http
            .cors(Customizer.withDefaults())
            .csrf(AbstractHttpConfigurer::disable)
            .sessionManagement(session -> session.sessionCreationPolicy(SessionCreationPolicy.STATELESS))
            .oauth2ResourceServer(oauth2 -> oauth2.jwt(Customizer.withDefaults()))
            .authorizeHttpRequests(authorize -> authorize
                .requestMatchers(
                    "/api/v1/pub/**",
                    "/v3/api-docs/**",
                    "/scalar/**",
                    "/swagger-ui/**",
                    "/swagger-ui.html"
                ).permitAll()
                .anyRequest().authenticated()
            );

        return http.build();
    }}
}}
"#
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::migration::DatabaseColumn;

    fn table(name: &str) -> DatabaseTable {
        DatabaseTable {
            name: name.to_string(),
            columns: vec![DatabaseColumn {
                name: "id".to_string(),
                data_type: "bigint".to_string(),
                nullable: false,
                primary_key: true,
            }],
        }
    }

    fn model_source(files: &BTreeMap<String, String>) -> &str {
        files
            .iter()
            .find(|(key, _)| key.ends_with("shared/utils/ConcreteTableModel.java"))
            .map(|(_, content)| content.as_str())
            .expect("ConcreteTableModel.java deve ser gerado")
    }

    #[test]
    fn rbac_constants_come_from_real_tables() {
        let tables = vec![
            table("TB_ROLES"),
            table("tb_permissions"),
            table("TB_USERS"),
        ];
        let mut files = BTreeMap::new();
        generate_shared_classes(&mut files, "com.nego.design", &tables);

        let model = model_source(&files);
        assert!(model.contains("ROLE = \"TB_ROLES\";"));
        assert!(model.contains("PERMISSION = \"tb_permissions\";"));
        assert!(model.contains("USER = \"TB_USERS\";"));
        // USERS deixa de ser excluído: a entidade de utilizadores referencia-o.
        assert!(model.contains("USERS = \"TB_USERS\";"));
    }

    #[test]
    fn rbac_constants_fall_back_to_defaults_without_tables() {
        let mut files = BTreeMap::new();
        generate_shared_classes(&mut files, "com.nego.design", &[]);

        let model = model_source(&files);
        assert!(model.contains("ROLE = \"TB_ROLES\";"));
        assert!(model.contains("PERMISSION = \"TB_PERMISSIONS\";"));
        assert!(model.contains("USER = \"TB_USERS\";"));
        assert!(!model.contains("USERS ="));
    }

    #[test]
    fn validation_resolves_missing_constant_from_tables() {
        let mut files = BTreeMap::new();
        generate_shared_classes(&mut files, "com.nego.design", &[]);
        files.insert(
            "src/main/java/com/nego/design/modules/x/model/X.java".to_string(),
            "@Table(name = ConcreteTableModel.MESSAGES)\n".to_string(),
        );

        validate_table_model_references(&mut files, &[table("messages")])
            .expect("MESSAGES deve ser resolvida a partir da tabela");

        assert!(model_source(&files).contains("MESSAGES = \"messages\";"));
    }

    #[test]
    fn validation_fails_for_unknown_reference() {
        let mut files = BTreeMap::new();
        generate_shared_classes(&mut files, "com.nego.design", &[]);
        files.insert(
            "src/main/java/com/nego/design/modules/x/model/X.java".to_string(),
            "@Table(name = ConcreteTableModel.NAO_EXISTE)\n".to_string(),
        );

        let error = validate_table_model_references(&mut files, &[])
            .expect_err("sem tabela correspondente deve falhar");
        assert!(error.contains("NAO_EXISTE"));
    }

    #[test]
    fn validation_accepts_defined_references() {
        let mut files = BTreeMap::new();
        generate_shared_classes(&mut files, "com.nego.design", &[table("TB_USERS")]);
        files.insert(
            "src/main/java/com/nego/design/modules/users/model/User.java".to_string(),
            "@Table(name = ConcreteTableModel.USER)\n@Table(name = ConcreteTableModel.USERS)\n"
                .to_string(),
        );

        validate_table_model_references(&mut files, &[table("TB_USERS")])
            .expect("USER e USERS estão definidas");
    }
}

