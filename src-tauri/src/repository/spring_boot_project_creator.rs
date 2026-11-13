use crate::model::codegen::{
    DatabaseEngine, SpringBootProjectRequest, SpringBootProjectResult, SpringLanguage,
};
use std::{
    fs,
    path::PathBuf,
};

const SPRING_BOOT_VERSION: &str = "3.4.1";
const SPRINGDOC_VERSION: &str = "2.6.0";

/// Resolve o driver JDBC e a dependência Maven conforme o motor de base de dados.
fn db_dependency(engine: &DatabaseEngine) -> (&'static str, &'static str, &'static str) {
    match engine {
        DatabaseEngine::Postgres => (
            "org.postgresql",
            "postgresql",
            "runtime",
        ),
        DatabaseEngine::Mysql => (
            "com.mysql",
            "mysql-connector-j",
            "runtime",
        ),
        DatabaseEngine::Sqlite => (
            "org.xerial",
            "sqlite-jdbc",
            "runtime",
        ),
    }
}

/// Resolve as propriedades `application.properties` para a base de dados.
fn db_properties(engine: &DatabaseEngine) -> String {
    match engine {
        DatabaseEngine::Postgres => [
            "spring.datasource.url=jdbc:postgresql://localhost:5432/negodesign",
            "spring.datasource.username=postgres",
            "spring.datasource.password=postgres",
            "spring.datasource.driver-class-name=org.postgresql.Driver",
        ]
        .join("\n"),
        DatabaseEngine::Mysql => [
            "spring.datasource.url=jdbc:mysql://localhost:3306/negodesign?useSSL=false&serverTimezone=UTC",
            "spring.datasource.username=root",
            "spring.datasource.password=root",
            "spring.datasource.driver-class-name=com.mysql.cj.jdbc.Driver",
        ]
        .join("\n"),
        DatabaseEngine::Sqlite => [
            "spring.datasource.url=jdbc:sqlite:negodesign.db",
            "spring.datasource.username=",
            "spring.datasource.password=",
            "spring.datasource.driver-class-name=org.sqlite.JDBC",
        ]
        .join("\n"),
    }
}

fn orm_dependency(orm_strategy: &str) -> (&'static str, &'static str, &'static str) {
    match orm_strategy {
        "spring-jdbc" | "jdbc" => (
            "org.springframework.boot",
            "spring-boot-starter-jdbc",
            "compile",
        ),
        _ => (
            "org.springframework.boot",
            "spring-boot-starter-data-jpa",
            "compile",
        ),
    }
}

fn pom_xml(req: &SpringBootProjectRequest, deps: &[String]) -> String {
    let dep_blocks = deps
        .iter()
        .map(|d| {
            let parts: Vec<&str> = d.split(':').collect();
            let (group, artifact, scope) = (parts[0], parts[1], parts[2]);
            let scope_block = if scope == "runtime" {
                "        <scope>runtime</scope>\n"
            } else {
                ""
            };
            format!(
                "        <dependency>\n            <groupId>{group}</groupId>\n            <artifactId>{artifact}</artifactId>\n{scope_block}        </dependency>"
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
        <java.version>21</java.version>
        <springdoc.version>{SPRINGDOC_VERSION}</springdoc.version>
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

fn application_properties(req: &SpringBootProjectRequest) -> String {
    let db = db_properties(&req.database_engine);
    let jpa = if req.orm_strategy != "spring-jdbc" && req.orm_strategy != "jdbc" {
        "\n# JPA / Hibernate\nspring.jpa.hibernate.ddl-auto=update\nspring.jpa.show-sql=true\nspring.jpa.properties.hibernate.format_sql=true\nspring.jpa.open-in-view=false"
    } else {
        ""
    };
    let api_docs = if req.include_api_docs {
        "\n\n# SpringDoc OpenAPI / Swagger UI\nspringdoc.api-docs.path=/v3/api-docs\nspringdoc.swagger-ui.path=/swagger-ui.html\nspringdoc.swagger-ui.operationsSorter=method"
    } else {
        ""
    };
    format!(
        "# Aplicação Spring Boot gerada pelo Negodesign\nspring.application.name={name}\nserver.port=8080\n\n# Base de dados\n{db}{jpa}{api_docs}\n",
        name = req.name
    )
}

fn readme_md(req: &SpringBootProjectRequest, deps: &[String]) -> String {
    let deps_list = deps
        .iter()
        .map(|d| {
            let parts: Vec<&str> = d.split(':').collect();
            format!("- `{}:{}` — {}", parts[0], parts[1], dep_description(d))
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# {name}\n\n{description}\n\n## Stack técnica\n\n- **Spring Boot** {SPRING_BOOT_VERSION}\n- **Linguagem**: {lang}\n- **ORM**: {orm}\n- **Base de dados**: {db}\n{api_docs_line}\n\n## Dependências incluídas\n\n{deps_list}\n\n## Como executar\n\n```bash\n./mvnw spring-boot:run\n```\n\nA aplicação arranca em `http://localhost:8080`.\n\n{swagger_block}\n## Próximos passos\n\n1. Configure a ligação à base de dados em `src/main/resources/application.properties`.\n2. Execute `./mvnw spring-boot:run` para iniciar o servidor.\n3. Use o Negodesign para gerar os endpoints CRUD a partir das tabelas da base de dados.\n",
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

fn dep_description(dep: &str) -> &'static str {
    if dep.contains("spring-boot-starter-web") {
        "API REST com Spring MVC (controladores, validação, JSON)."
    } else if dep.contains("spring-boot-starter-data-jpa") {
        "Persistência com entidades Jakarta Persistence e JpaRepository paginado."
    } else if dep.contains("spring-boot-starter-jdbc") {
        "Acesso a dados com Spring JDBC e a nova API fluente JdbcClient."
    } else if dep.contains("postgresql") {
        "Driver JDBC para PostgreSQL."
    } else if dep.contains("mysql-connector-j") {
        "Driver JDBC para MySQL."
    } else if dep.contains("sqlite-jdbc") {
        "Driver JDBC para SQLite."
    } else if dep.contains("springdoc-openapi") {
        "Geração automática de documentação OpenAPI 3 e Swagger UI."
    } else if dep.contains("lombok") {
        "Anotações que reduzem código boilerplate (getters, setters, builders)."
    } else if dep.contains("spring-boot-starter-test") {
        "Dependências de teste (JUnit 5, Mockito, AssertJ)."
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
    fs::create_dir_all(target.join("src/main/resources"))
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(target.join("src/test/java")).map_err(|e| e.to_string())?;

    let mut deps: Vec<String> = vec![
        "org.springframework.boot:spring-boot-starter-web:compile".to_string(),
    ];
    let (orm_g, orm_a, orm_s) = orm_dependency(&req.orm_strategy);
    deps.push(format!("{orm_g}:{orm_a}:{orm_s}"));
    let (db_g, db_a, db_s) = db_dependency(&req.database_engine);
    deps.push(format!("{db_g}:{db_a}:{db_s}"));
    if req.include_api_docs {
        deps.push(format!(
            "org.springdoc:springdoc-openapi-starter-webmvc-ui:{}",
            SPRINGDOC_VERSION
        ));
    }
    deps.push("org.springframework.boot:spring-boot-starter-test:test".to_string());

    let mut generated = 0usize;

    // pom.xml
    fs::write(target.join("pom.xml"), pom_xml(req, &deps)).map_err(|e| e.to_string())?;
    generated += 1;

    // mvnw + mvnw.cmd (placeholders to remind user; real wrapper would need binary)
    fs::write(
        target.join(".gitignore"),
        "target/\n*.class\n*.jar\n*.log\n.idea/\n.vscode/\n*.iml\n.DS_Store\n",
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    // application.properties
    fs::write(
        target.join("src/main/resources/application.properties"),
        application_properties(req),
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    // Main class
    let (file_name, content) = main_class(&req.group_id, &req.language, &req.name);
    fs::write(src_root.join(&file_name), content).map_err(|e| e.to_string())?;
    generated += 1;

    // README
    fs::write(target.join("README.md"), readme_md(req, &deps)).map_err(|e| e.to_string())?;
    generated += 1;

    let dep_display = deps
        .iter()
        .map(|d| {
            let parts: Vec<&str> = d.split(':').collect();
            format!("{}:{}", parts[0], parts[1])
        })
        .collect();

    let next_steps = vec![
        format!("cd {}", req.artifact_id),
        "./mvnw spring-boot:run".to_string(),
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
