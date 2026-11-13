use crate::model::codegen::{ApiCallPattern, FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::to_kebab_case;
use crate::repository::codegen::openapi::{admin_props, bff, client, pages, tanstack};
use std::collections::BTreeMap;

pub(super) fn insert_entity_files(
    files: &mut BTreeMap<String, String>,
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
    active_entities: &[OpenApiEntity],
) {
    let kebab = to_kebab_case(&entity.name);

    // API Client
    let client_code = client::generate_client_ts(req, entity);
    let client_path = match req.frontend_framework {
        FrontendFramework::Sveltekit => format!("src/lib/api/{kebab}.client.ts"),
        FrontendFramework::Nuxt => format!("lib/api/{kebab}.client.ts"),
        FrontendFramework::Nextjs => format!("lib/api/{kebab}.client.ts"),
    };
    files.insert(client_path, client_code);

    // TanStack Query
    if req.use_tanstack_query {
        let query_code = tanstack::generate_tanstack_queries_ts(req, entity);
        let query_path = match req.frontend_framework {
            FrontendFramework::Sveltekit => format!("src/lib/queries/{kebab}.queries.ts"),
            FrontendFramework::Nuxt => format!("lib/queries/{kebab}.queries.ts"),
            FrontendFramework::Nextjs => format!("lib/queries/{kebab}.queries.ts"),
        };
        files.insert(query_path, query_code);
    }

    // BFF Routes
    if req.api_call_pattern == ApiCallPattern::Bff {
        let bff_files = bff::generate_bff_routes(req, entity);
        for (path, content) in bff_files {
            files.insert(path, content);
        }
    }

    // Admin Props Config
    let config_code =
        admin_props::build_openapi_admin_config_ts(entity, &active_entities, &req.entities);
    let config_path = match req.frontend_framework {
        FrontendFramework::Sveltekit => format!("src/lib/admin/{kebab}.config.ts"),
        FrontendFramework::Nuxt => format!("lib/admin/{kebab}.config.ts"),
        FrontendFramework::Nextjs => format!("lib/admin/{kebab}.config.ts"),
    };
    files.insert(config_path, config_code);

    // AdminCrud Page
    let (page_path, page_content) = pages::generate_openapi_admin_page(req, entity);
    files.insert(page_path, page_content);
}
