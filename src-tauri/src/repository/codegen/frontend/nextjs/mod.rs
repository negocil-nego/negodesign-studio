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
    repository::codegen::naming::{to_kebab_case, to_pascal_case},
};
use std::collections::BTreeMap;

mod admin_page;
mod routes;

use self::admin_page::build_next_admin_page;
use self::routes::{
    build_next_bff_collection_route, build_next_bff_item_route,
    build_next_monolith_collection_route, build_next_monolith_item_route,
};

pub fn generate(
    req: &FullstackCodegenRequest,
    tables: &[DatabaseTable],
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();

    files.insert(
        "src/navigation/admin-sidebar.ts".to_string(),
        build_admin_sidebar_ts(tables),
    );

    if req.architecture_mode == ArchitectureMode::Monolith {
        files.insert(
            "src/lib/server/db.server.ts".to_string(),
            build_db_client_ts(&req.database_engine),
        );
    }

    for table in tables {
        let kebab = to_kebab_case(&table.name);
        let pascal = to_pascal_case(&table.name);

        files.insert(
            format!("src/features/{kebab}/{kebab}.admin.ts"),
            // O negodesign fica na raiz do projecto Next; a config está em
            // src/features/<kebab>/, logo são 3 níveis para a raiz.
            build_admin_config_ts(table, tables, "../../../negodesign", "../../navigation/admin-sidebar"),
        );
        files.insert(
            format!("src/features/{kebab}/{kebab}.api.ts"),
            build_api_client_ts(req, table),
        );

        if req.use_tanstack_query {
            files.insert(
                format!("src/features/{kebab}/{kebab}.query.ts"),
                build_tanstack_query_ts(&req.frontend_framework, table, &format!("./{kebab}.api")),
            );
        }

        if req.architecture_mode == ArchitectureMode::Monolith {
            files.insert(
                format!("src/lib/server/repositories/{kebab}.repository.ts"),
                build_direct_table_repository_ts(&req.database_engine, table, "../db.server"),
            );
            files.insert(
                format!("src/app/api/{kebab}/route.ts"),
                build_next_monolith_collection_route(&kebab, &pascal),
            );
            files.insert(
                format!("src/app/api/{kebab}/[id]/route.ts"),
                build_next_monolith_item_route(&kebab, &pascal),
            );
        } else if req.api_call_pattern == ApiCallPattern::Bff {
            let base = req
                .backend_base_url
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("http://localhost:8080");
            files.insert(
                format!("src/app/api/{kebab}/route.ts"),
                build_next_bff_collection_route(base, &kebab),
            );
            files.insert(
                format!("src/app/api/{kebab}/[id]/route.ts"),
                build_next_bff_item_route(base, &kebab),
            );
        }

        files.insert(
            format!("src/app/admin/{kebab}/page.tsx"),
            build_next_admin_page(&kebab, &pascal),
        );
    }

    Ok(files)
}
