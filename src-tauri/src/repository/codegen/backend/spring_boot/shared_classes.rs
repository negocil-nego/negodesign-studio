use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::to_snake_case,
};
use std::collections::BTreeMap;

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
}}
"#
        ),
    );

    // 3. ConcreteTableModel.java
    let mut table_constants = String::new();
    table_constants.push_str("    public static final String ROLE = \"TB_ROLES\";\n");
    table_constants.push_str("    public static final String PERMISSION = \"TB_PERMISSIONS\";\n");
    table_constants.push_str("    public static final String USER = \"TB_USERS\";\n");

    for t in tables {
        let upper_snake = to_snake_case(&t.name).to_ascii_uppercase();
        if upper_snake != "ROLES" && upper_snake != "PERMISSIONS" && upper_snake != "USERS" && upper_snake != "ROLE" && upper_snake != "PERMISSION" && upper_snake != "USER" {
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

@AllArgsConstructor
public abstract class CommonService<T extends CommonModel> {{
    private final CommonRepository<T> repository;

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

