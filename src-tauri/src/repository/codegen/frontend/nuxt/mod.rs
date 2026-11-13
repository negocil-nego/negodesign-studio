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

use self::admin_page::build_nuxt_admin_page;
use self::routes::{
    build_nuxt_bff_item_route, build_nuxt_bff_route, build_nuxt_monolith_collection_route,
    build_nuxt_monolith_item_route,
};

pub fn generate(
    req: &FullstackCodegenRequest,
    tables: &[DatabaseTable],
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();

    files.insert(
        "composables/navigation/admin-sidebar.ts".to_string(),
        build_admin_sidebar_ts(tables),
    );

    if req.architecture_mode == ArchitectureMode::Monolith {
        files.insert(
            "server/utils/db.server.ts".to_string(),
            build_db_client_ts(&req.database_engine),
        );
    }

    for table in tables {
        let kebab = to_kebab_case(&table.name);
        let pascal = to_pascal_case(&table.name);

        files.insert(
            format!("composables/features/{kebab}/{kebab}.admin.ts"),
            build_admin_config_ts(table, tables),
        );
        files.insert(
            format!("composables/features/{kebab}/{kebab}.api.ts"),
            build_api_client_ts(req, table),
        );

        if req.use_tanstack_query {
            files.insert(
                format!("composables/features/{kebab}/{kebab}.query.ts"),
                build_tanstack_query_ts(&req.frontend_framework, table),
            );
        }

        if req.architecture_mode == ArchitectureMode::Monolith {
            files.insert(
                format!("server/repositories/{kebab}.repository.ts"),
                build_direct_table_repository_ts(&req.database_engine, table, "../utils/db.server"),
            );
            files.insert(
                format!("server/api/{kebab}/index.ts"),
                build_nuxt_monolith_collection_route(&kebab, &pascal),
            );
            files.insert(
                format!("server/api/{kebab}/[id].ts"),
                build_nuxt_monolith_item_route(&kebab, &pascal),
            );
        } else if req.api_call_pattern == ApiCallPattern::Bff {
            let base = req
                .backend_base_url
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("http://localhost:8080");
            files.insert(
                format!("server/api/{kebab}/index.ts"),
                build_nuxt_bff_route(base, &kebab),
            );
            files.insert(
                format!("server/api/{kebab}/[id].ts"),
                build_nuxt_bff_item_route(base, &kebab),
            );
        }

        files.insert(
            format!("pages/admin/{kebab}.vue"),
            build_nuxt_admin_page(&kebab, &pascal),
        );
    }

    Ok(files)
}
