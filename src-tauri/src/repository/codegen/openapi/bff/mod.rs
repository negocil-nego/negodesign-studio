use crate::model::codegen::{FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::to_kebab_case;
use crate::repository::codegen::openapi::client::{path_with_id_expr, resolve_entity_endpoints};
use std::collections::BTreeMap;

mod nextjs;
mod nuxt;
mod sveltekit;

use nextjs::generate_nextjs_bff_routes;
use nuxt::generate_nuxt_bff_routes;
use sveltekit::generate_sveltekit_bff_routes;

pub(super) struct BffRouteCtx<'a> {
    pub kebab: &'a str,
    pub list_url: &'a str,
    pub create_url: &'a str,
    pub get_id_svelte: &'a str,
    pub update_id_svelte: &'a str,
    pub delete_id_svelte: &'a str,
    pub get_id_generic: &'a str,
    pub update_id_generic: &'a str,
    pub delete_id_generic: &'a str,
    pub update_method: &'a str,
    pub delete_method: &'a str,
}

pub fn generate_bff_routes(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> BTreeMap<String, String> {
    let kebab = to_kebab_case(&entity.name);
    let backend_base = req
        .backend_base_url
        .as_deref()
        .or(entity.server_url.as_deref())
        .unwrap_or("http://localhost:8080")
        .trim_end_matches('/');

    let resolved = resolve_entity_endpoints(req, entity);
    let list_url = format!("{backend_base}{}", resolved.list_path);
    let create_url = format!("{backend_base}{}", resolved.create_path);
    let get_id_svelte = format!(
        "{backend_base}{}",
        path_with_id_expr(&resolved.get_path, "params.id")
    );
    let update_id_svelte = format!(
        "{backend_base}{}",
        path_with_id_expr(&resolved.update_path, "params.id")
    );
    let delete_id_svelte = format!(
        "{backend_base}{}",
        path_with_id_expr(&resolved.delete_path, "params.id")
    );

    let get_id_generic = format!(
        "{backend_base}{}",
        path_with_id_expr(&resolved.get_path, "id")
    );
    let update_id_generic = format!(
        "{backend_base}{}",
        path_with_id_expr(&resolved.update_path, "id")
    );
    let delete_id_generic = format!(
        "{backend_base}{}",
        path_with_id_expr(&resolved.delete_path, "id")
    );

    let ctx = BffRouteCtx {
        kebab: &kebab,
        list_url: &list_url,
        create_url: &create_url,
        get_id_svelte: &get_id_svelte,
        update_id_svelte: &update_id_svelte,
        delete_id_svelte: &delete_id_svelte,
        get_id_generic: &get_id_generic,
        update_id_generic: &update_id_generic,
        delete_id_generic: &delete_id_generic,
        update_method: &resolved.update_method,
        delete_method: &resolved.delete_method,
    };

    match req.frontend_framework {
        FrontendFramework::Sveltekit => generate_sveltekit_bff_routes(&ctx),
        FrontendFramework::Nuxt => generate_nuxt_bff_routes(&ctx),
        FrontendFramework::Nextjs => generate_nextjs_bff_routes(&ctx),
    }
}
