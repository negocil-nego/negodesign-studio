mod axios;
mod native;
mod native_nuxt;
mod paths;
mod resolve;

pub use paths::path_with_id_expr;
pub use resolve::{resolve_entity_endpoints, ResolvedCrudEndpoints};

use crate::model::codegen::{
    ApiCallPattern, FrontendFramework, HttpClientKind, OpenApiFrontendCodegenRequest,
};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::{to_camel_case, to_pascal_case};
use crate::repository::codegen::openapi::dtos::strip_entity_action_or_kind;
use axios::generate_axios_client;
use native::generate_fetch_native_client;
use native_nuxt::generate_nuxt_native_client;
use paths::resolve_client_urls;

pub fn generate_client_ts(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> String {
    let pascal = to_pascal_case(&entity.name);
    let camel = to_camel_case(&entity.name);

    let base_name = strip_entity_action_or_kind(&entity.name);
    let base_pascal = to_pascal_case(base_name);

    let resolved: ResolvedCrudEndpoints = resolve_entity_endpoints(req, entity);

    let (list_url, get_url_tpl, create_url, update_url_tpl, delete_url_tpl) =
        resolve_client_urls(req, entity, &resolved);

    let update_method = if req.api_call_pattern == ApiCallPattern::Bff {
        "PUT"
    } else {
        resolved.update_method.as_str()
    };
    let delete_method = if req.api_call_pattern == ApiCallPattern::Bff {
        "DELETE"
    } else {
        resolved.delete_method.as_str()
    };

    match req.http_client {
        HttpClientKind::Axios => generate_axios_client(
            &pascal,
            &base_pascal,
            &camel,
            &list_url,
            &get_url_tpl,
            &create_url,
            &update_url_tpl,
            update_method,
            &delete_url_tpl,
            delete_method,
        ),
        HttpClientKind::Native => match req.frontend_framework {
            FrontendFramework::Nuxt => generate_nuxt_native_client(
                &pascal,
                &base_pascal,
                &camel,
                &list_url,
                &get_url_tpl,
                &create_url,
                &update_url_tpl,
                update_method,
                &delete_url_tpl,
                delete_method,
            ),
            FrontendFramework::Sveltekit | FrontendFramework::Nextjs => generate_fetch_native_client(
                &pascal,
                &base_pascal,
                &camel,
                &list_url,
                &get_url_tpl,
                &create_url,
                &update_url_tpl,
                update_method,
                &delete_url_tpl,
                delete_method,
            ),
        },
    }
}
