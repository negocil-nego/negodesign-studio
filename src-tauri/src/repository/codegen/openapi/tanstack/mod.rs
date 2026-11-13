pub mod nextjs;
pub mod nuxt;
pub mod sveltekit;

pub use nextjs::generate_nextjs_queries_ts;
pub use nuxt::generate_nuxt_queries_ts;
pub use sveltekit::generate_sveltekit_queries_ts;

use crate::model::codegen::{FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::{to_camel_case, to_kebab_case, to_pascal_case};
use crate::repository::codegen::openapi::dtos::strip_entity_action_or_kind;

pub fn generate_tanstack_queries_ts(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> String {
    let pascal = to_pascal_case(&entity.name);
    let camel = to_camel_case(&entity.name);
    let kebab = to_kebab_case(&entity.name);

    let base_name = strip_entity_action_or_kind(&entity.name);
    let base_pascal = to_pascal_case(base_name);

    match req.frontend_framework {
        FrontendFramework::Sveltekit => {
            generate_sveltekit_queries_ts(&pascal, &camel, &kebab, &base_pascal)
        }
        FrontendFramework::Nuxt => generate_nuxt_queries_ts(&pascal, &camel, &kebab, &base_pascal),
        FrontendFramework::Nextjs => {
            generate_nextjs_queries_ts(&pascal, &camel, &kebab, &base_pascal)
        }
    }
}
