mod entity;
mod redirect;

use super::dtos;
use self::entity::insert_entity_files;
use self::redirect::insert_admin_index;
use crate::model::codegen::{ApiCallPattern, FrontendFramework, OpenApiFrontendCodegenRequest};
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

    // 2.5 Base URL única do backend para as rotas BFF em src/routes/api.
    // As rotas importam BACKEND_BASE_URL daqui — a URL do backend muda num só sítio.
    if matches!(req.frontend_framework, FrontendFramework::Sveltekit)
        && req.api_call_pattern == ApiCallPattern::Bff
    {
        let base = req
            .backend_base_url
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                active_entities
                    .first()
                    .and_then(|entity| entity.server_url.as_deref())
            })
            .unwrap_or("http://localhost:8080")
            .trim_end_matches('/')
            .to_string();
        files.insert(
            "src/lib/server/backend.ts".to_string(),
            format!(
                r#"// Base URL do backend (BFF) — fonte única para as rotas em src/routes/api.
// Actualize apenas aqui quando o backend mudar de URL.
export const BACKEND_BASE_URL = "{base}";
"#
            ),
        );
    }

    // 3. For each active entity: API client, TanStack queries, BFF routes, Admin Config, and Admin Page
    for entity in &active_entities {
        insert_entity_files(&mut files, req, entity, &active_entities);
    }

    // 4. Index / redirect page for admin
    insert_admin_index(&mut files, req, &active_entities);

    Ok(files)
}
