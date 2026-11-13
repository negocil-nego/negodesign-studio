use crate::{
    model::{
        codegen::{
            ArchitectureMode, FrontendFramework, FullstackCodegenRequest, FullstackCodegenResult,
            OpenApiFrontendCodegenRequest, OpenApiFrontendCodegenResult,
        },
        migration::{DatabaseColumn, DatabaseTable},
        project::{ComponentExport, ComponentFile},
    },
    repository::{
        codegen::{
            backend::generate_backend_files, frontend::generate_frontend_files,
            naming::{to_camel_case, to_snake_case},
            openapi::generate_openapi_frontend_files,
        },
        component_copy::{
            assets::relocate_static_assets, collector::CopyCollector, manifest::ManifestCatalog,
            paths::workspace_root, theme::attach_theme_and_styles,
        },
        project::installer::{build_index, validate_relative_path, write_project_files},
    },
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

/// Remove tabelas que colidem depois da normalização de nomes.
///
/// A mesma tabela lógica pode aparecer com gerações de nomes diferentes na BD
/// (legado PascalCase vs. snake_case): "Address" e "address", "AgentAddress" e
/// "agent_address", "LogsActions" e "logs_actions". Sem esta dedupe o
/// `ConcreteTableModel` recebe constantes duplicadas (o Java não compila) e as
/// entidades geradas se sobrescrevem no mesmo caminho de ficheiro.
///
/// Mantém a variante já em snake_case (o nome canónico); em empate, a primeira.
/// As colunas de cada tabela são normalizadas da mesma forma.
fn dedupe_tables(tables: Vec<DatabaseTable>) -> Vec<DatabaseTable> {
    let mut by_key: BTreeMap<String, DatabaseTable> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();

    for table in tables {
        let key = to_snake_case(&table.name);
        match by_key.get(&key) {
            Some(existing) => {
                let existing_is_snake = existing.name == key;
                let candidate_is_snake = table.name == key;
                if candidate_is_snake && !existing_is_snake {
                    by_key.insert(key, table);
                }
            }
            None => {
                order.push(key.clone());
                by_key.insert(key, table);
            }
        }
    }

    let mut result: Vec<DatabaseTable> = order
        .into_iter()
        .filter_map(|k| by_key.remove(&k))
        .collect();
    for table in &mut result {
        dedupe_columns(table);
    }
    result
}

/// Remove colunas que colidem depois da normalização para o identificador Java.
///
/// Uma mesma tabela pode ter colunas das duas gerações de nomes
/// ("messageType" e "message_type", "isRead" e "is_read"): ambas produzem o
/// campo `messageType`/`isRead` em Java, o que gera propriedades duplicadas na
/// entidade, nos DTOs (`record` com componentes duplicados) e no controller.
///
/// Mantém a variante já em snake_case (a coluna canónica); em empate, a primeira.
fn dedupe_columns(table: &mut DatabaseTable) {
    let mut by_key: BTreeMap<String, DatabaseColumn> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();

    for column in std::mem::take(&mut table.columns) {
        let key = to_camel_case(&column.name);
        match by_key.get(&key) {
            Some(existing) => {
                let existing_is_snake = existing.name == to_snake_case(&existing.name);
                let candidate_is_snake = column.name == to_snake_case(&column.name);
                if candidate_is_snake && !existing_is_snake {
                    by_key.insert(key, column);
                }
            }
            None => {
                order.push(key.clone());
                by_key.insert(key, column);
            }
        }
    }

    table.columns = order
        .into_iter()
        .filter_map(|k| by_key.remove(&k))
        .collect();
}

pub fn generate_fullstack_crud(
    mut request: FullstackCodegenRequest,
) -> Result<FullstackCodegenResult, String> {
    if request.tables.is_empty() {
        return Err("Seleccione pelo menos uma tabela para gerar o CRUD.".to_string());
    }
    request.tables = dedupe_tables(std::mem::take(&mut request.tables));
    let frontend_path = request.frontend_path.trim();
    if frontend_path.is_empty() {
        return Err("Indique o caminho da aplicação frontend.".to_string());
    }
    let frontend_root = PathBuf::from(frontend_path);

    let mut summary = Vec::new();
    let copied_ui_files = install_admin_crud_ui_by_framework(
        &frontend_root,
        &request.frontend_framework,
        request.theme.as_deref(),
    )?;
    summary.push(format!(
        "Copiados {copied_ui_files} ficheiros do componente AdminCrud e dependências para {frontend_path}/negodesign"
    ));

    let frontend_files = generate_frontend_files(&request, &request.tables)?;
    let generated_frontend_files = write_file_map(&frontend_root, &frontend_files)?;
    summary.push(format!(
        "Gerados {generated_frontend_files} ficheiros de páginas e integração no projecto frontend ({:?})",
        request.frontend_framework
    ));

    let mut generated_backend_files = 0;
    if request.architecture_mode == ArchitectureMode::Split {
        let backend_path = request
            .backend_path
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| "Indique o caminho da aplicação backend.".to_string())?;
        let backend_root = PathBuf::from(backend_path);
        let backend_files = generate_backend_files(&request, &request.tables)?;

        // Remove directórios de código gerado que já não fazem parte da
        // geração actual (ex.: `modules/users` depois de a classe passar a
        // `User`, ou `modules/annualbilling` depois da correcção
        // camelCase → `annual_billing`). Sem isto ficam entidades antigas em
        // disco que duplicam o mapeamento da mesma tabela e impedem o arranque
        // do Spring.
        remove_stale_generated_dirs(&backend_root, &backend_files);

        // Se for projecto Spring Boot existente com pom.xml, adiciona apenas dependências em falta sem sobrescrever
        if matches!(
            request.backend_framework,
            Some(crate::model::codegen::BackendFramework::SpringBoot)
        ) {
            let pom_path = backend_root.join("pom.xml");
            if pom_path.exists() {
                if let Ok(true) = crate::repository::codegen::backend::spring_boot::pom_merger::ensure_pom_dependencies(
                    &pom_path,
                    &request.database_engine,
                ) {
                    summary.push("Adicionadas dependências em falta ao pom.xml existente sem sobrescrever a configuração do projecto".to_string());
                }
            }
        }

        generated_backend_files = write_file_map(&backend_root, &backend_files)?;
        let lang_note = match (&request.backend_framework, &request.spring_language) {
            (Some(crate::model::codegen::BackendFramework::SpringBoot), Some(lang)) => {
                format!(", linguagem: {lang:?}")
            }
            _ => String::new(),
        };
        summary.push(format!(
            "Gerados {generated_backend_files} ficheiros no projecto backend ({:?}, ORM: {}{})",
            request.backend_framework,
            request.orm_strategy.as_deref().unwrap_or("default"),
            lang_note
        ));
    } else {
        summary.push(format!(
            "Geradas rotas de API fullstack com ligação direta a {:?} na aplicação frontend",
            request.database_engine
        ));
    }

    Ok(FullstackCodegenResult {
        generated_backend_files,
        generated_frontend_files,
        copied_ui_files,
        tables_processed: request.tables.len(),
        summary,
    })
}

pub fn generate_openapi_frontend(
    request: OpenApiFrontendCodegenRequest,
) -> Result<OpenApiFrontendCodegenResult, String> {
    if request.entities.is_empty() {
        return Err("Nenhuma entidade OpenAPI fornecida para geração.".to_string());
    }
    let frontend_path = request.frontend_path.trim();
    if frontend_path.is_empty() {
        return Err("Indique o caminho da aplicação frontend.".to_string());
    }
    let frontend_root = PathBuf::from(frontend_path);

    let mut summary = Vec::new();
    let copied_ui_files = install_admin_crud_ui_by_framework(
        &frontend_root,
        &request.frontend_framework,
        request.theme.as_deref(),
    )?;
    summary.push(format!(
        "Copiados {copied_ui_files} ficheiros do componente AdminCrud e dependências para {frontend_path}/negodesign"
    ));

    let frontend_files = generate_openapi_frontend_files(&request)?;
    let generated_frontend_files = write_file_map(&frontend_root, &frontend_files)?;
    let endpoints_count = if request.selected_endpoints.is_empty() {
        request.entities.len()
    } else {
        request.selected_endpoints.len()
    };
    summary.push(format!(
        "Gerados {generated_frontend_files} ficheiros de DTOs, API Client, TanStack Queries e páginas AdminCrud ({:?})",
        request.frontend_framework
    ));

    Ok(OpenApiFrontendCodegenResult {
        generated_frontend_files,
        copied_ui_files,
        endpoints_processed: endpoints_count,
        summary,
    })
}

fn install_admin_crud_ui_by_framework(
    frontend_root: &Path,
    framework: &FrontendFramework,
    theme: Option<&str>,
) -> Result<usize, String> {
    let root = workspace_root()?;
    let catalog = ManifestCatalog::global(&root.join("src"))?;

    let (manifest_lang, project_lang, static_dir) = match framework {
        FrontendFramework::Sveltekit => ("Svelte", "Svelte", Some("static/nego")),
        FrontendFramework::Nuxt => ("Vue", "Nuxt", Some("public")),
        FrontendFramework::Nextjs => ("React", "Nextjs", Some("public")),
    };

    let locales = vec!["en".to_string(), "pt".to_string()];
    let selected_theme = theme.unwrap_or("default");

    let mut collector = CopyCollector::new(&root, catalog, manifest_lang, &locales);
    collector.collect(
        "admin-crud",
        Some("src/project/ui/pages/admin/dependence.json"),
    )?;

    attach_theme_and_styles(
        &root,
        &mut collector.files_by_path,
        manifest_lang,
        selected_theme,
    )?;

    let final_map = if let Some(sd) = static_dir {
        relocate_static_assets(collector.files_by_path, sd)
    } else {
        collector.files_by_path
    };

    let files: Vec<ComponentFile> = final_map
        .into_iter()
        .map(|(path, content)| ComponentFile { path, content })
        .collect();
    let mut components: Vec<ComponentExport> = collector.exported_by_id.into_values().collect();

    let copied = write_project_files(frontend_root, project_lang, files)?;
    let package_root = frontend_root.join("negodesign");
    let index = build_index(project_lang, &mut components)?;
    fs::create_dir_all(&package_root).map_err(|e| e.to_string())?;
    fs::write(package_root.join("index.ts"), index).map_err(|e| e.to_string())?;

    Ok(copied)
}

fn write_file_map(base_dir: &Path, files: &BTreeMap<String, String>) -> Result<usize, String> {
    let mut count = 0;
    for (rel_path, content) in files {
        let valid_rel = validate_relative_path(rel_path)?;
        let full_path = base_dir.join(valid_rel);
        if rel_path == "pom.xml" && full_path.exists() {
            continue;
        }
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(full_path, content).map_err(|e| e.to_string())?;
        count += 1;
    }
    Ok(count)
}

/// Remove do disco os directórios de código totalmente gerado pelo backend que
/// já não fazem parte da geração actual (`modules/*` do Spring JPA e
/// `feature/*` do fallback JDBC).
///
/// Sem isto, regenerações que renomeiam classes/ficheiros (ex.: `Users` →
/// `User`, `annualbilling` → `annual_billing`) deixam os ficheiros antigos em
/// disco: duas entidades passam a mapear a mesma tabela e o Spring não arranca.
fn remove_stale_generated_dirs(root: &Path, files: &BTreeMap<String, String>) {
    const MARKERS: [&str; 2] = ["/modules/", "/feature/"];

    let mut owned: BTreeSet<&str> = BTreeSet::new();
    for key in files.keys() {
        for marker in MARKERS {
            if let Some(index) = key.find(marker) {
                owned.insert(&key[..index + marker.len() - 1]);
            }
        }
    }

    for dir in owned {
        let path = root.join(dir);
        if path.is_dir() {
            let _ = fs::remove_dir_all(path);
        }
    }
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

    #[test]
    fn dedupe_merges_case_variants_keeping_snake() {
        let result = dedupe_tables(vec![
            table("Address"),
            table("Agent"),
            table("_prisma_migrations"),
            table("address"),
            table("agent"),
            table("Product"),
            table("product"),
        ]);
        let names: Vec<&str> = result.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "address",
                "agent",
                "_prisma_migrations",
                "product"
            ]
        );
    }

    #[test]
    fn dedupe_merges_camel_and_snake_variants() {
        let result = dedupe_tables(vec![table("AgentAddress"), table("agent_address")]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "agent_address");

        // Ordem invertida: mantém a variante snake na mesma
        let result = dedupe_tables(vec![table("agent_address"), table("AgentAddress")]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "agent_address");
    }

    #[test]
    fn dedupe_keeps_distinct_tables() {
        let result = dedupe_tables(vec![
            table("AnnualBilling"),
            table("annual_billing_tier"),
            table("CompanyUsers"),
        ]);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn concrete_table_model_has_no_duplicate_constants() {
        // Reproduz o cenário real: BD com tabelas legadas em PascalCase e
        // tabelas novas em snake_case para a mesma tabela lógica.
        let tables = dedupe_tables(vec![
            table("Address"),
            table("address"),
            table("Agent"),
            table("agent"),
            table("Product"),
            table("product"),
            table("AnnualBilling"),
            table("_prisma_migrations"),
        ]);

        let mut files = BTreeMap::new();
        crate::repository::codegen::backend::spring_boot::shared_classes::generate_shared_classes(
            &mut files,
            "com.nego.design",
            &tables,
        );

        let source = files
            .get("src/main/java/com/nego/design/shared/utils/ConcreteTableModel.java")
            .expect("ConcreteTableModel.java deve ser gerado");

        let constants: Vec<&str> = source
            .lines()
            .filter_map(|line| {
                line.trim()
                    .strip_prefix("public static final String ")
                    .and_then(|rest| rest.split('=').next())
            })
            .map(|name| name.trim())
            .collect();

        let mut seen = std::collections::HashSet::new();
        for constant in &constants {
            assert!(
                seen.insert(*constant),
                "constante duplicada no ConcreteTableModel: {constant}"
            );
        }

        // Mantém a variante snake_case como valor da tabela
        assert!(source.contains("ADDRESS = \"address\";"));
        assert!(source.contains("AGENT = \"agent\";"));
        // CamelCase passa a partir palavras: AnnualBilling → ANNUAL_BILLING
        assert!(source.contains("ANNUAL_BILLING = \"AnnualBilling\";"));
        assert!(source.contains("PRISMA_MIGRATIONS = \"_prisma_migrations\";"));
    }

    fn col(name: &str, data_type: &str, nullable: bool) -> DatabaseColumn {
        DatabaseColumn {
            name: name.to_string(),
            data_type: data_type.to_string(),
            nullable,
            primary_key: false,
        }
    }

    /// Tabela real da BD do utilizador: colunas das duas gerações de nomes
    /// (camelCase legado + snake_case Prisma) na mesma tabela.
    fn messages_table() -> DatabaseTable {
        DatabaseTable {
            name: "messages".to_string(),
            columns: vec![
                col("content", "varchar", false),
                col("messageType", "varchar", false),
                col("mediaUrl", "varchar", true),
                col("sentAt", "datetime", false),
                col("status", "varchar", false),
                col("isRead", "bool", false),
                col("readAt", "datetime", true),
                col("productId", "uuid", true),
                col("productQtd", "int", true),
                col("senderCompanyId", "uuid", false),
                col("conversationId", "varchar", false),
                col("conversation_id", "varchar", false),
                col("is_read", "bool", false),
                col("media_url", "varchar", true),
                col("message_type", "varchar", false),
                col("product_id", "uuid", true),
                col("product_qtd", "int", true),
                col("read_at", "datetime", true),
                col("sender_company_id", "uuid", false),
                col("sent_at", "datetime", false),
            ],
        }
    }

    #[test]
    fn dedupe_columns_merges_camel_and_snake_variants() {
        let result = dedupe_tables(vec![messages_table()]);
        let names: Vec<&str> = result[0]
            .columns
            .iter()
            .map(|c| c.name.as_str())
            .collect();
        assert_eq!(
            names,
            vec![
                "content",
                "message_type",
                "media_url",
                "sent_at",
                "status",
                "is_read",
                "read_at",
                "product_id",
                "product_qtd",
                "sender_company_id",
                "conversation_id",
            ]
        );
    }

    #[test]
    fn generated_entity_and_dtos_have_no_duplicate_properties() {
        use crate::model::codegen::{DatabaseEngine, SpringLanguage};
        use crate::repository::codegen::backend::spring_boot;

        let tables = dedupe_tables(vec![messages_table()]);
        let files = spring_boot::generate(
            &tables,
            "com.nego.design",
            "spring-data-jpa",
            &SpringLanguage::Java,
            None,
            &DatabaseEngine::Postgres,
            &FrontendFramework::Sveltekit,
        )
        .expect("a geração Spring Boot deve suceder");

        let record_components = |source: &str, record: &str| -> Vec<String> {
            let start = source
                .find(&format!("public record {record}("))
                .expect("record deve existir")
                + format!("public record {record}(").len();
            let end = source[start..].find(") {").expect("fim do record") + start;
            source[start..end]
                .split(',')
                .map(|component| {
                    component
                        .trim()
                        .split_whitespace()
                        .next_back()
                        .unwrap_or_default()
                        .to_string()
                })
                .filter(|component| !component.is_empty())
                .collect()
        };

        let assert_unique = |items: &[String], label: &str| {
            let mut seen = std::collections::HashSet::new();
            for item in items {
                assert!(
                    seen.insert(item),
                    "propriedade duplicada em {label}: {item}"
                );
            }
        };

        // Entidade: campos privados únicos e @Column só na variante snake
        let entity = files
            .get("src/main/java/com/nego/design/modules/messages/model/Messages.java")
            .expect("entidade Messages.java deve ser gerada");
        let fields: Vec<String> = entity
            .lines()
            .filter_map(|line| line.trim().strip_prefix("private "))
            .filter_map(|rest| rest.split_whitespace().nth(1))
            .map(|name| name.trim_end_matches(';').to_string())
            .collect();
        assert_eq!(fields.len(), 11, "a entidade deve ter 11 campos: {fields:?}");
        assert_unique(&fields, "Messages.java");
        assert!(entity.contains("@Column(name = \"message_type\""));
        assert!(!entity.contains("@Column(name = \"messageType\""));

        // DTOs: componentes do record únicos
        let response = files
            .get("src/main/java/com/nego/design/modules/messages/dto/MessagesResponse.java")
            .expect("MessagesResponse.java deve ser gerado");
        let response_components = record_components(response, "MessagesResponse");
        assert_unique(&response_components, "MessagesResponse.java");
        assert!(response_components.contains(&"messageType".to_string()));

        let request = files
            .get("src/main/java/com/nego/design/modules/messages/dto/MessagesRequest.java")
            .expect("MessagesRequest.java deve ser gerado");
        let request_components = record_components(request, "MessagesRequest");
        assert_unique(&request_components, "MessagesRequest.java");
    }

    #[test]
    fn users_entity_is_singular_and_references_defined_constant() {
        use crate::model::codegen::{DatabaseEngine, SpringLanguage};
        use crate::repository::codegen::backend::spring_boot;

        let tables = dedupe_tables(vec![table("TB_USERS")]);
        let files = spring_boot::generate(
            &tables,
            "com.nego.design",
            "spring-data-jpa",
            &SpringLanguage::Java,
            None,
            &DatabaseEngine::Postgres,
            &FrontendFramework::Sveltekit,
        )
        .expect("a geração Spring Boot deve suceder");

        // Classe singular (User), como Role/Permission
        let entity = files
            .get("src/main/java/com/nego/design/modules/users/model/User.java")
            .expect("User.java (singular) deve ser gerado");
        assert!(entity.contains("@Table(name = ConcreteTableModel.USER)"));
        assert!(
            !files
                .keys()
                .any(|key| key.ends_with("modules/users/model/Users.java")),
            "não deve gerar a versão plural Users.java"
        );

        // USER e USERS existem no ConcreteTableModel gerado
        let model = files
            .get("src/main/java/com/nego/design/shared/utils/ConcreteTableModel.java")
            .expect("ConcreteTableModel.java deve ser gerado");
        assert!(model.contains("USER = \"TB_USERS\";"));
        assert!(model.contains("USERS = \"TB_USERS\";"));
    }
}

