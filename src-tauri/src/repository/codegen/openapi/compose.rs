mod entity;
mod redirect;

use super::dtos;
use self::entity::insert_entity_files;
use self::redirect::insert_admin_index;
use crate::model::codegen::{FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use std::collections::BTreeMap;

pub fn generate_openapi_frontend_files(
    req: &OpenApiFrontendCodegenRequest,
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();

    // 1. DTOs and Response types based on OpenAPI schemas
    let dtos_content = dtos::generate_dtos_ts(&req.entities);
    let dtos_path = match req.frontend_framework {
        FrontendFramework::Sveltekit => "src/lib/types/openapi.dto.ts",
        FrontendFramework::Nuxt => "types/openapi.dto.ts",
        FrontendFramework::Nextjs => "types/openapi.dto.ts",
    };
    files.insert(dtos_path.to_string(), dtos_content);

    // 2. Identify the active entities based on selected_endpoints
    let active_entities: Vec<OpenApiEntity> = if req.selected_endpoints.is_empty() {
        req.entities.clone()
    } else {
        req.entities
            .iter()
            .filter(|e| {
                req.selected_endpoints
                    .iter()
                    .any(|sel| sel.entity_name == e.name)
            })
            .cloned()
            .collect()
    };

    if active_entities.is_empty() {
        return Err("Nenhuma entidade OpenAPI seleccionada para integração.".to_string());
    }

    // 3. For each active entity: API client, TanStack queries, BFF routes, Admin Config, and Admin Page
    for entity in &active_entities {
        insert_entity_files(&mut files, req, entity, &active_entities);
    }

    // 4. Index / redirect page for admin
    insert_admin_index(&mut files, req, &active_entities);

    Ok(files)
}
