pub mod nuxt;
pub mod nextjs;
pub mod sveltekit;

mod nuxt_direct;
mod nextjs_direct;
mod sveltekit_direct;

pub use nuxt::generate_nuxt_page;
pub use nextjs::generate_nextjs_page;
pub use sveltekit::generate_sveltekit_page;

use crate::model::codegen::{FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::to_kebab_case;

pub fn generate_openapi_admin_page(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> (String, String) {
    let kebab = to_kebab_case(&entity.name);
    match req.frontend_framework {
        FrontendFramework::Sveltekit => (
            format!("src/routes/(admin)/admin/{kebab}/+page.svelte"),
            generate_sveltekit_page(req, entity),
        ),
        FrontendFramework::Nuxt => (
            format!("pages/admin/{kebab}.vue"),
            generate_nuxt_page(req, entity),
        ),
        FrontendFramework::Nextjs => (
            format!("app/admin/{kebab}/page.tsx"),
            generate_nextjs_page(req, entity),
        ),
    }
}
