pub mod admin_props;
pub mod bff;
pub mod client;
pub mod dtos;
pub mod pages;
pub mod tanstack;

use crate::model::codegen::{ApiCallPattern, FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::to_kebab_case;
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

    // 4. Index / redirect page for admin
    if let Some(first) = active_entities.first() {
        let first_kebab = to_kebab_case(&first.name);
        match req.frontend_framework {
            FrontendFramework::Sveltekit => {
                files.insert(
                    "src/routes/(admin)/admin/+page.ts".to_string(),
                    format!(
                        "import {{ redirect }} from \"@sveltejs/kit\";\n\n\
export const load = () => {{\n\
  throw redirect(307, \"/admin/{first_kebab}\");\n\
}};\n"
                    ),
                );
            }
            FrontendFramework::Nuxt => {
                files.insert(
                    "pages/admin/index.vue".to_string(),
                    format!(
                        "<template>\n  <div />\n</template>\n\n\
<script setup lang=\"ts\">\n\
await navigateTo(\"/admin/{first_kebab}\");\n\
</script>\n"
                    ),
                );
            }
            FrontendFramework::Nextjs => {
                files.insert(
                    "app/admin/page.tsx".to_string(),
                    format!(
                        "import {{ redirect }} from \"next/navigation\";\n\n\
export default function AdminIndexPage() {{\n\
  redirect(\"/admin/{first_kebab}\");\n\
}}\n"
                    ),
                );
            }
        }
    }

    Ok(files)
}

