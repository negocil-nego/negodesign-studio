use super::{
    admin_props::{build_admin_config_ts, build_admin_sidebar_ts},
    fullstack_db::{build_db_client_ts, build_direct_table_repository_ts},
    http_client::build_api_client_ts,
    tanstack_query::build_tanstack_query_ts,
};
use crate::{
    model::{
        codegen::{ApiCallPattern, ArchitectureMode, FullstackCodegenRequest},
        migration::DatabaseTable,
    },
    repository::codegen::naming::{feature_group_keys, to_kebab_case, to_pascal_case},
};
use std::collections::BTreeMap;

mod admin_page;
mod routes;

use self::admin_page::build_svelte_admin_page;
use self::routes::{
    build_bff_collection_route, build_bff_item_route, build_monolith_collection_route,
    build_monolith_item_route,
};

/// Agrupa as tabelas por "feature" domínio (`naming::feature_group_keys`):
/// `agent`, `agent-address`, … → `features/agent/`; kebab único mantém-se
/// (`prisma-migrations` → `features/prisma-migrations/`).
///
/// Devolve um mapa kebab → nome da pasta da feature.
fn feature_folders(tables: &[DatabaseTable]) -> BTreeMap<String, String> {
    let kebabs: Vec<String> = tables.iter().map(|t| to_kebab_case(&t.name)).collect();
    feature_group_keys(&kebabs)
}

/// Módulo único da base URL do backend (BFF). Todas as rotas em
/// `src/routes/api` importam `BACKEND_BASE_URL` daqui — muda-se a URL do
/// backend num só sítio.
fn build_backend_base_ts(base_api_url: &str) -> String {
    format!(
        r#"// Base URL do backend (BFF) — fonte única para as rotas em src/routes/api.
// Actualize apenas aqui quando o backend mudar de URL.
export const BACKEND_BASE_URL = "{base_api_url}";
"#
    )
}

pub fn generate(
    req: &FullstackCodegenRequest,
    tables: &[DatabaseTable],
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();

    files.insert(
        "src/lib/navigation/admin-sidebar.ts".to_string(),
        build_admin_sidebar_ts(tables),
    );

    if req.architecture_mode == ArchitectureMode::Monolith {
        files.insert(
            "src/lib/server/db.server.ts".to_string(),
            build_db_client_ts(&req.database_engine),
        );
    }

    let backend_base = req
        .backend_base_url
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("http://localhost:8080")
        .trim_end_matches('/')
        .to_string();

    if req.architecture_mode != ArchitectureMode::Monolith
        && req.api_call_pattern == ApiCallPattern::Bff
    {
        files.insert(
            "src/lib/server/backend.ts".to_string(),
            build_backend_base_ts(&format!("{backend_base}/api")),
        );
    }

    let folders = feature_folders(tables);

    for table in tables {
        let kebab = to_kebab_case(&table.name);
        let pascal = to_pascal_case(&table.name);
        let folder = folders.get(&kebab).map(String::as_str).unwrap_or(&kebab);

        files.insert(
            format!("src/lib/features/{folder}/data/{kebab}.admin.ts"),
            build_admin_config_ts(
                table,
                tables,
                "../../../negodesign",
                "../../../navigation/admin-sidebar",
            ),
        );
        files.insert(
            format!("src/lib/features/{folder}/api/{kebab}.api.ts"),
            build_api_client_ts(req, table),
        );

        if req.use_tanstack_query {
            files.insert(
                format!("src/lib/features/{folder}/repo/{kebab}.query.ts"),
                build_tanstack_query_ts(
                    &req.frontend_framework,
                    table,
                    &format!("../api/{kebab}.api"),
                ),
            );
        }

        if req.architecture_mode == ArchitectureMode::Monolith {
            files.insert(
                format!("src/lib/server/repositories/{kebab}.repository.ts"),
                build_direct_table_repository_ts(&req.database_engine, table, "../db.server"),
            );
            files.insert(
                format!("src/routes/api/{kebab}/+server.ts"),
                build_monolith_collection_route(&kebab, &pascal),
            );
            files.insert(
                format!("src/routes/api/{kebab}/[id]/+server.ts"),
                build_monolith_item_route(&kebab, &pascal),
            );
        } else if req.api_call_pattern == ApiCallPattern::Bff {
            files.insert(
                format!("src/routes/api/{kebab}/+server.ts"),
                build_bff_collection_route(&kebab),
            );
            files.insert(
                format!("src/routes/api/{kebab}/[id]/+server.ts"),
                build_bff_item_route(&kebab),
            );
        }

        files.insert(
            format!("src/routes/admin/{kebab}/+page.svelte"),
            build_svelte_admin_page(&kebab, &pascal, folder),
        );
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::codegen::{
        ApiCallPattern, ArchitectureMode, FrontendFramework, FullstackCodegenRequest,
        HttpClientKind,
    };
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

    fn request(tables: Vec<DatabaseTable>) -> FullstackCodegenRequest {
        FullstackCodegenRequest {
            architecture_mode: ArchitectureMode::Split,
            tables,
            frontend_path: "/tmp/project".to_string(),
            frontend_framework: FrontendFramework::Sveltekit,
            api_call_pattern: ApiCallPattern::Bff,
            http_client: HttpClientKind::Native,
            use_tanstack_query: true,
            backend_path: None,
            backend_framework: None,
            orm_strategy: None,
            base_package: None,
            spring_language: None,
            database_engine: crate::model::codegen::DatabaseEngine::Postgres,
            backend_base_url: Some("http://localhost:8080".to_string()),
            theme: None,
            database_connection: None,
        }
    }

    fn monolith_request(tables: Vec<DatabaseTable>) -> FullstackCodegenRequest {
        FullstackCodegenRequest {
            architecture_mode: ArchitectureMode::Monolith,
            ..request(tables)
        }
    }

    #[test]
    fn feature_folders_groups_by_first_segment() {
        let folders = feature_folders(&[
            table("agent"),
            table("AgentAddress"),
            table("agent-assignment"),
            table("prisma-migrations"),
            table("messages"),
        ]);

        assert_eq!(folders["agent"], "agent");
        assert_eq!(folders["agent-address"], "agent");
        assert_eq!(folders["agent-assignment"], "agent");
        // Sem partilhas do primeiro segmento, mantém o kebab completo
        assert_eq!(folders["prisma-migrations"], "prisma-migrations");
        assert_eq!(folders["messages"], "messages");
    }

    #[test]
    fn svelte_features_use_api_repo_data_layout() {
        let tables = vec![table("agent"), table("agent-address")];
        let files = generate(&request(tables.clone()), &tables).expect("geração");

        for path in [
            "src/lib/features/agent/api/agent.api.ts",
            "src/lib/features/agent/repo/agent.query.ts",
            "src/lib/features/agent/data/agent.admin.ts",
            "src/lib/features/agent/api/agent-address.api.ts",
            "src/lib/features/agent/data/agent-address.admin.ts",
            "src/routes/admin/agent-address/+page.svelte",
            "src/lib/navigation/admin-sidebar.ts",
        ] {
            assert!(files.contains_key(path), "ficheiro em falta: {path}");
        }

        // A config importa o negodesign dentro de src/ e o sidebar único
        let admin = &files["src/lib/features/agent/data/agent.admin.ts"];
        assert!(admin.contains("\"../../../negodesign/project/ui/pages/admin/data/types\""));
        assert!(admin.contains("\"../../../navigation/admin-sidebar\""));
        assert!(
            !admin.contains("icon: \"Layout04\""),
            "os itens do sidebar não devem estar embutidos na config"
        );

        // O query importa a api em ../api/
        let query = &files["src/lib/features/agent/repo/agent.query.ts"];
        assert!(query.contains("from \"../api/agent.api\""));

        // A página admin importa as pastas da feature e o negodesign em src/
        let page = &files["src/routes/admin/agent-address/+page.svelte"];
        assert!(page.contains("from \"$lib/features/agent/api/agent-address.api\""));
        assert!(page.contains("from \"$lib/features/agent/data/agent-address.admin\""));
        assert!(page.contains("from \"../../../negodesign/project/ui/pages/admin/ui/AdminCrud.svelte\""));
    }

    #[test]
    fn sidebar_is_generated_once_and_not_repeated() {
        let tables = vec![table("agent"), table("address")];
        let files = generate(&request(tables.clone()), &tables).expect("geração");

        let with_items = files
            .iter()
            .filter(|(_, content)| content.contains("icon: \"Layout04\""))
            .map(|(path, _)| path.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            with_items,
            vec!["src/lib/navigation/admin-sidebar.ts"],
            "os itens do sidebar só devem existir num ficheiro"
        );
    }

    #[test]
    fn bff_routes_use_shared_base_url_and_response_json() {
        let tables = vec![table("product-category")];
        let files = generate(&request(tables.clone()), &tables).expect("geração");

        // Base URL única, gerada uma vez
        let backend = &files["src/lib/server/backend.ts"];
        assert!(backend.contains("export const BACKEND_BASE_URL = \"http://localhost:8080/api\";"));

        // Rota de item no formato pedido: sem json do kit, com Response.json
        // e a URL composta a partir da base partilhada.
        let item = &files["src/routes/api/product-category/[id]/+server.ts"];
        assert!(!item.contains("@sveltejs/kit"), "json do @sveltejs/kit está depreciado");
        assert!(item.contains("import { BACKEND_BASE_URL } from \"$lib/server/backend\";"));
        assert!(item.contains("const BACKEND_URL = `${BACKEND_BASE_URL}/product-category`;"));
        assert!(item.contains("return Response.json(await res.json(), { status: res.status });"));
        assert!(item.contains("return new Response(null, { status: res.status });"));
        // Código indentado (2 espaços no corpo das funções)
        assert!(item.contains("\n  const body = await request.text();"));
        assert!(item.contains("\n    method: \"PUT\","));

        let list = &files["src/routes/api/product-category/+server.ts"];
        assert!(!list.contains("@sveltejs/kit"));
        assert!(list.contains("Response.json"));
        assert!(list.contains("\n  const res = await fetch("));
    }

    #[test]
    fn monolith_routes_use_response_json_without_kit_import() {
        let tables = vec![table("agent")];
        let files = generate(&monolith_request(tables.clone()), &tables).expect("geração");

        let list = &files["src/routes/api/agent/+server.ts"];
        assert!(!list.contains("@sveltejs/kit"));
        assert!(list.contains("return Response.json(result);"));
        assert!(list.contains("return Response.json(created, { status: 201 });"));
        assert!(list.contains("\n  const page = Number("));

        let item = &files["src/routes/api/agent/[id]/+server.ts"];
        assert!(!item.contains("@sveltejs/kit"));
        assert!(item.contains("return Response.json(updated);"));
        assert!(item.contains("return new Response(null, { status: 204 });"));
    }

    #[test]
    fn generated_frontend_files_are_indented() {
        let agent_address = DatabaseTable {
            name: "agent-address".to_string(),
            columns: vec![
                DatabaseColumn {
                    name: "id".to_string(),
                    data_type: "bigint".to_string(),
                    nullable: false,
                    primary_key: true,
                },
                DatabaseColumn {
                    name: "name".to_string(),
                    data_type: "varchar".to_string(),
                    nullable: false,
                    primary_key: false,
                },
                DatabaseColumn {
                    name: "status".to_string(),
                    data_type: "varchar".to_string(),
                    nullable: true,
                    primary_key: false,
                },
            ],
        };
        let tables = vec![table("agent"), agent_address, table("messages")];
        let files = generate(&request(tables.clone()), &tables).expect("geração");

        let api = &files["src/lib/features/agent/api/agent-address.api.ts"];
        assert!(api.contains("\n  async findAll(page = 0, size = 10) {"), ".api.ts sem indentação");

        let query = &files["src/lib/features/agent/repo/agent-address.query.ts"];
        assert!(query.contains("\n  return createQuery({"), ".query.ts sem indentação");

        let admin = &files["src/lib/features/agent/data/agent-address.admin.ts"];
        assert!(admin.contains("\n  return records.flatMap((item) => ["), ".admin.ts sem indentação");
        assert!(admin.contains("\n    title: \""), ".admin.ts sem indentação");
        assert!(admin.contains("\n            id: \""), "campos do formulário sem indentação");

        let page = &files["src/routes/admin/agent-address/+page.svelte"];
        assert!(page.contains("\n  import { onMount } from \"svelte\";"), "+page.svelte sem indentação");
        assert!(page.contains("\n  async function loadRecords() {"));
    }
}
