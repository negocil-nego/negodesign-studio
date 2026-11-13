use crate::model::codegen::DatabaseEngine;
use std::{fs, path::Path};

pub fn ensure_pom_dependencies(pom_path: &Path, engine: &DatabaseEngine) -> Result<bool, String> {
    if !pom_path.exists() {
        return Ok(false);
    }
    let content = fs::read_to_string(pom_path).map_err(|e| e.to_string())?;

    struct Dep {
        artifact: &'static str,
        xml: &'static str,
    }

    let db_driver = match engine {
        DatabaseEngine::Postgres => Dep {
            artifact: "postgresql",
            xml: "        <dependency>\n            <groupId>org.postgresql</groupId>\n            <artifactId>postgresql</artifactId>\n            <scope>runtime</scope>\n        </dependency>",
        },
        DatabaseEngine::Mysql => Dep {
            artifact: "mysql-connector-j",
            xml: "        <dependency>\n            <groupId>com.mysql</groupId>\n            <artifactId>mysql-connector-j</artifactId>\n            <scope>runtime</scope>\n        </dependency>",
        },
        DatabaseEngine::Sqlite => Dep {
            artifact: "sqlite-jdbc",
            xml: "        <dependency>\n            <groupId>org.xerial</groupId>\n            <artifactId>sqlite-jdbc</artifactId>\n            <version>3.47.1.0</version>\n            <scope>runtime</scope>\n        </dependency>",
        },
    };

    let required = vec![
        Dep {
            artifact: "spring-boot-starter-data-jpa",
            xml: "        <dependency>\n            <groupId>org.springframework.boot</groupId>\n            <artifactId>spring-boot-starter-data-jpa</artifactId>\n        </dependency>",
        },
        Dep {
            artifact: "spring-boot-starter-security",
            xml: "        <dependency>\n            <groupId>org.springframework.boot</groupId>\n            <artifactId>spring-boot-starter-security</artifactId>\n        </dependency>",
        },
        Dep {
            artifact: "spring-boot-starter-security-oauth2-resource-server",
            xml: "        <dependency>\n            <groupId>org.springframework.boot</groupId>\n            <artifactId>spring-boot-starter-security-oauth2-resource-server</artifactId>\n        </dependency>",
        },
        Dep {
            artifact: "spring-boot-starter-validation",
            xml: "        <dependency>\n            <groupId>org.springframework.boot</groupId>\n            <artifactId>spring-boot-starter-validation</artifactId>\n        </dependency>",
        },
        Dep {
            artifact: "lombok",
            xml: "        <dependency>\n            <groupId>org.projectlombok</groupId>\n            <artifactId>lombok</artifactId>\n            <optional>true</optional>\n        </dependency>",
        },
        Dep {
            artifact: "bcprov-jdk18on",
            xml: "        <dependency>\n            <groupId>org.bouncycastle</groupId>\n            <artifactId>bcprov-jdk18on</artifactId>\n            <version>1.84</version>\n        </dependency>",
        },
        Dep {
            artifact: "springdoc-openapi-starter-webmvc-scalar",
            xml: "        <dependency>\n            <groupId>org.springdoc</groupId>\n            <artifactId>springdoc-openapi-starter-webmvc-scalar</artifactId>\n            <version>3.2.0</version>\n        </dependency>",
        },
        db_driver,
        Dep {
            artifact: "spring-boot-starter-security-test",
            xml: "        <dependency>\n            <groupId>org.springframework.boot</groupId>\n            <artifactId>spring-boot-starter-security-test</artifactId>\n            <scope>test</scope>\n        </dependency>",
        },
        Dep {
            artifact: "spring-boot-starter-security-oauth2-resource-server-test",
            xml: "        <dependency>\n            <groupId>org.springframework.boot</groupId>\n            <artifactId>spring-boot-starter-security-oauth2-resource-server-test</artifactId>\n            <scope>test</scope>\n        </dependency>",
        },
        Dep {
            artifact: "spring-boot-starter-data-jpa-test",
            xml: "        <dependency>\n            <groupId>org.springframework.boot</groupId>\n            <artifactId>spring-boot-starter-data-jpa-test</artifactId>\n            <scope>test</scope>\n        </dependency>",
        },
    ];

    let mut missing_xml = Vec::new();
    for dep in required {
        // If web is present (either web or webmvc), or springdoc is present, don't duplicate
        if dep.artifact == "springdoc-openapi-starter-webmvc-scalar" && content.contains("springdoc-openapi") {
            continue;
        }
        if !content.contains(&format!("<artifactId>{}</artifactId>", dep.artifact)) {
            missing_xml.push(dep.xml);
        }
    }

    if missing_xml.is_empty() {
        return Ok(false);
    }

    if let Some(pos) = content.rfind("</dependencies>") {
        let mut new_content = String::with_capacity(content.len() + 1024);
        new_content.push_str(&content[..pos]);
        for xml in missing_xml {
            new_content.push_str(xml);
            new_content.push('\n');
        }
        new_content.push_str(&content[pos..]);
        fs::write(pom_path, new_content).map_err(|e| e.to_string())?;
        Ok(true)
    } else {
        Ok(false)
    }
}

