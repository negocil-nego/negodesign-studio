use crate::model::codegen::{
    DatabaseEngine, SpringBootProjectRequest, SpringBootProjectResult, SpringLanguage,
};
use crate::repository::codegen::backend::spring_boot::resources::{
    RSA_PRIVATE_KEY_PEM, RSA_PUBLIC_KEY_PEM,
};
use std::{fs, path::PathBuf};

const SPRING_BOOT_VERSION: &str = "4.1.2";
const SQLITE_JDBC_VERSION: &str = "3.47.1.0";

/// Dependência Maven. `version: None` significa que é gerida pelo parent do Spring Boot.
struct Dependency {
    group: &'static str,
    artifact: &'static str,
    version: Option<&'static str>,
    scope: Option<&'static str>,
}

fn orm_dependencies(orm_strategy: &str) -> Vec<Dependency> {
    let mut deps = Vec::new();
    let is_jdbc = orm_strategy == "spring-jdbc" || orm_strategy == "jdbc";
    if is_jdbc {
        deps.push(Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-starter-jdbc",
            version: None,
            scope: None,
        });
    } else {
        deps.push(Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-starter-data-jpa",
            version: None,
            scope: None,
        });
        deps.push(Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-starter-jdbc",
            version: None,
            scope: None,
        });
    }
    deps
}

fn db_dependency(engine: &DatabaseEngine) -> Dependency {
    match engine {
        DatabaseEngine::Postgres => Dependency {
            group: "org.postgresql",
            artifact: "postgresql",
            version: None,
            scope: Some("runtime"),
        },
        DatabaseEngine::Mysql => Dependency {
            group: "com.mysql",
            artifact: "mysql-connector-j",
            version: None,
            scope: Some("runtime"),
        },
        DatabaseEngine::Sqlite => Dependency {
            group: "org.xerial",
            artifact: "sqlite-jdbc",
            version: Some(SQLITE_JDBC_VERSION),
            scope: Some("runtime"),
        },
    }
}

fn dependencies(req: &SpringBootProjectRequest) -> Vec<Dependency> {
    let mut deps = vec![
        Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-starter-webmvc",
            version: None,
            scope: None,
        },
        Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-starter-security",
            version: None,
            scope: None,
        },
        Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-starter-security-oauth2-resource-server",
            version: None,
            scope: None,
        },
        Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-starter-validation",
            version: None,
            scope: None,
        },
        Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-devtools",
            version: None,
            scope: Some("runtime"),
        },
        Dependency {
            group: "org.projectlombok",
            artifact: "lombok",
            version: None,
            scope: None,
        },
        Dependency {
            group: "org.bouncycastle",
            artifact: "bcprov-jdk18on",
            version: Some("1.84"),
            scope: Some("compile"),
        },
    ];
    deps.extend(orm_dependencies(&req.orm_strategy));
    deps.push(db_dependency(&req.database_engine));
    if req.include_api_docs {
        deps.push(Dependency {
            group: "org.springdoc",
            artifact: "springdoc-openapi-starter-webmvc-scalar",
            version: Some("3.2.0"),
            scope: None,
        });
    }
    // Test starters (mirror the runtime starters)
    deps.push(Dependency {
        group: "org.springframework.boot",
        artifact: "spring-boot-starter-webmvc-test",
        version: None,
        scope: Some("test"),
    });
    deps.push(Dependency {
        group: "org.springframework.boot",
        artifact: "spring-boot-starter-security-test",
        version: None,
        scope: Some("test"),
    });
    deps.push(Dependency {
        group: "org.springframework.boot",
        artifact: "spring-boot-starter-security-oauth2-resource-server-test",
        version: None,
        scope: Some("test"),
    });
    if req.orm_strategy == "spring-jdbc" || req.orm_strategy == "jdbc" {
        deps.push(Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-starter-jdbc-test",
            version: None,
            scope: Some("test"),
        });
    } else {
        deps.push(Dependency {
            group: "org.springframework.boot",
            artifact: "spring-boot-starter-data-jpa-test",
            version: None,
            scope: Some("test"),
        });
    }
    deps
}

fn pom_xml(req: &SpringBootProjectRequest, deps: &[Dependency]) -> String {
    let dep_blocks = deps
        .iter()
        .map(|d| {
            let version = d
                .version
                .map(|v| format!("            <version>{v}</version>\n"))
                .unwrap_or_default();
            let scope = d
                .scope
                .map(|s| format!("            <scope>{s}</scope>\n"))
                .unwrap_or_default();
            format!(
                "        <dependency>\n            <groupId>{}</groupId>\n            <artifactId>{}</artifactId>\n{version}{scope}        </dependency>",
                d.group, d.artifact
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>{SPRING_BOOT_VERSION}</version>
        <relativePath/>
    </parent>

    <groupId>{group_id}</groupId>
    <artifactId>{artifact_id}</artifactId>
    <version>0.0.1-SNAPSHOT</version>
    <name>{name}</name>
    <description>{description}</description>

    <properties>
        <java.version>25</java.version>
    </properties>

    <dependencies>
{dep_blocks}
    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
            </plugin>
        </plugins>
    </build>
</project>
"#,
        group_id = req.group_id,
        artifact_id = req.artifact_id,
        name = req.name,
        description = req.description,
    )
}

fn main_class(package_path: &str, lang: &SpringLanguage, name: &str) -> (String, String) {
    let class_name = format!("{name}Application");
    match lang {
        SpringLanguage::Kotlin => {
            let content = format!(
                "package {package_path}\n\n\
import org.springframework.boot.autoconfigure.SpringBootApplication\n\
import org.springframework.boot.runApplication\n\n\
@SpringBootApplication\n\
class {class_name}\n\n\
fun main(args: Array<String>) {{\n\
    runApplication<{class_name}>(*args)\n\
}}\n"
            );
            (format!("{class_name}.kt"), content)
        }
        SpringLanguage::Java => {
            let content = format!(
                "package {package_path};\n\n\
import org.springframework.boot.SpringApplication;\n\
import org.springframework.boot.autoconfigure.SpringBootApplication;\n\n\
@SpringBootApplication\n\
public class {class_name} {{\n\
\n\
    public static void main(String[] args) {{\n\
        SpringApplication.run({class_name}.class, args);\n\
    }}\n\
}}\n"
            );
            (format!("{class_name}.java"), content)
        }
    }
}

fn readme_md(req: &SpringBootProjectRequest, deps: &[Dependency]) -> String {
    let deps_list = deps
        .iter()
        .map(|d| format!("- `{}:{}` — {}", d.group, d.artifact, dep_description(d.artifact)))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# {name}\n\n{description}\n\n## Stack técnica\n\n- **Spring Boot** {SPRING_BOOT_VERSION}\n- **Linguagem**: {lang}\n- **ORM**: {orm}\n- **Base de dados**: {db}\n{api_docs_line}\n\n## Dependências incluídas\n\n{deps_list}\n\n## Como executar\n\n```bash\nmvn spring-boot:run\n```\n\nA aplicação arranca em `http://localhost:8080`.\n\n{swagger_block}## Próximos passos\n\n1. Configure a ligação à base de dados em `src/main/resources/application.properties`.\n2. Execute `mvn spring-boot:run` para iniciar o servidor.\n3. Use o Negodesign para gerar os endpoints CRUD a partir das tabelas da base de dados.\n",
        name = req.name,
        description = req.description,
        lang = if matches!(req.language, SpringLanguage::Kotlin) { "Kotlin" } else { "Java" },
        orm = if req.orm_strategy == "spring-jdbc" || req.orm_strategy == "jdbc" { "Spring JDBC (JdbcClient)" } else { "Spring Data JPA" },
        db = match req.database_engine {
            DatabaseEngine::Postgres => "PostgreSQL",
            DatabaseEngine::Mysql => "MySQL",
            DatabaseEngine::Sqlite => "SQLite",
        },
        api_docs_line = if req.include_api_docs { "- **API Docs**: SpringDoc OpenAPI + Swagger UI" } else { "" },
        swagger_block = if req.include_api_docs {
            "## Documentação da API\n\n- Swagger UI: `http://localhost:8080/swagger-ui.html`\n- OpenAPI JSON: `http://localhost:8080/v3/api-docs`\n\n"
        } else {
            ""
        },
    )
}

fn dep_description(artifact: &str) -> &'static str {
    if artifact.contains("spring-boot-starter-webmvc") && !artifact.contains("test") {
        "API REST com Spring MVC (controladores, validação, JSON)."
    } else if artifact.contains("spring-boot-starter-data-jpa") && !artifact.contains("test") {
        "Persistência com entidades Jakarta Persistence e JpaRepository paginado."
    } else if artifact.contains("spring-boot-starter-jdbc") && !artifact.contains("test") {
        "Acesso a dados com Spring JDBC e a nova API fluente JdbcClient."
    } else if artifact.contains("postgresql") {
        "Driver JDBC para PostgreSQL."
    } else if artifact.contains("mysql-connector-j") {
        "Driver JDBC para MySQL."
    } else if artifact.contains("sqlite-jdbc") {
        "Driver JDBC para SQLite."
    } else if artifact.contains("springdoc-openapi") {
        "Geração automática de documentação OpenAPI 3 e Swagger UI."
    } else if artifact.contains("lombok") {
        "Anotações que reduzem código boilerplate (getters, setters, builders)."
    } else if artifact.contains("-test") {
        "Dependências de teste para o starter correspondente."
    } else {
        "Dependência do Spring Boot."
    }
}

pub fn create_spring_boot_project(
    req: &SpringBootProjectRequest,
) -> Result<SpringBootProjectResult, String> {
    let target = PathBuf::from(&req.target_dir).join(&req.artifact_id);
    fs::create_dir_all(&target).map_err(|e| e.to_string())?;

    let package_path = req.group_id.replace('.', "/");
    let src_root = match req.language {
        SpringLanguage::Kotlin => target.join("src/main/kotlin").join(&package_path),
        SpringLanguage::Java => target.join("src/main/java").join(&package_path),
    };
    fs::create_dir_all(&src_root).map_err(|e| e.to_string())?;
    let res_dir = target.join("src/main/resources");
    fs::create_dir_all(&res_dir).map_err(|e| e.to_string())?;
    let certs_dir = res_dir.join("certs/rsa");
    fs::create_dir_all(&certs_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(target.join("src/test/java")).map_err(|e| e.to_string())?;

    let deps = dependencies(req);
    let mut generated = 0usize;

    fs::write(target.join("pom.xml"), pom_xml(req, &deps)).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(
        target.join(".gitignore"),
        "target/\n*.class\n*.jar\n*.log\n.idea/\n.vscode/\n*.iml\n.DS_Store\n",
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    let app_yaml = format!(
        r#"server:
  port: ${{PORT:8080}}
  error:
    include-stacktrace: never
    include-binding-errors: always
    include-message: always

spring:
  application:
    name: {name}
  profiles:
    active: ${{SPRING_PROFILES_ACTIVE:local}}
  threads:
    virtual:
      enabled: true
  mvc:
    problem-details:
      enabled: true
  servlet:
    multipart:
      max-file-size: 20MB
      max-request-size: 20MB

springdoc:
  pathsToMatch: /api/v1/admin/**,/api/v1/pub/**

scalar:
  enabled: true

rsa:
  private-key: ${{RSA_PRIVATE_KEY:classpath:certs/rsa/private.pem}}
  public-key: ${{RSA_PUBLIC_KEY:classpath:certs/rsa/public.pem}}

app:
  cors:
    allowed-origins: ${{APP_CORS_ALLOWED_ORIGINS:http://localhost:5173}}
"#,
        name = req.name
    );

    let app_local_yml = match req.database_engine {
        DatabaseEngine::Postgres => r#"spring:
  datasource:
    url: ${DB_URL:jdbc:postgresql://localhost:5432/negodesign}
    username: ${DB_USERNAME:postgres}
    password: ${DB_PASSWORD:postgres}
  jpa:
    hibernate:
      ddl-auto: update
    show-sql: false
"#,
        DatabaseEngine::Mysql => r#"spring:
  datasource:
    url: ${DB_URL:jdbc:mysql://localhost:3306/negodesign?useSSL=false&serverTimezone=UTC}
    username: ${DB_USERNAME:root}
    password: ${DB_PASSWORD:root}
  jpa:
    hibernate:
      ddl-auto: update
    show-sql: false
"#,
        DatabaseEngine::Sqlite => r#"spring:
  datasource:
    url: ${DB_URL:jdbc:sqlite:negodesign.db}
    driver-class-name: org.sqlite.JDBC
  jpa:
    hibernate:
      ddl-auto: update
    show-sql: false
"#,
    };

    fs::write(res_dir.join("application.yaml"), app_yaml).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(res_dir.join("application-local.yml"), app_local_yml).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(certs_dir.join("private.pem"), RSA_PRIVATE_KEY_PEM).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(certs_dir.join("public.pem"), RSA_PUBLIC_KEY_PEM).map_err(|e| e.to_string())?;
    generated += 1;

    let (file_name, content) = main_class(&req.group_id, &req.language, &req.name);
    fs::write(src_root.join(&file_name), content).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("README.md"), readme_md(req, &deps)).map_err(|e| e.to_string())?;
    generated += 1;

    let dep_display = deps
        .iter()
        .map(|d| format!("{}:{}", d.group, d.artifact))
        .collect();

    let next_steps = vec![
        format!("cd {}", req.artifact_id),
        "mvn spring-boot:run".to_string(),
        "Abrir http://localhost:8080".to_string(),
    ];

    Ok(SpringBootProjectResult {
        project_dir: target.to_string_lossy().to_string(),
        build_tool: "Maven".to_string(),
        spring_boot_version: SPRING_BOOT_VERSION.to_string(),
        generated_files: generated,
        dependencies: dep_display,
        next_steps,
    })
}
