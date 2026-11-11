pub mod admin_props;
pub mod fullstack_db;
pub mod http_client;
pub mod nextjs;
pub mod nuxt;
pub mod sveltekit;
pub mod tanstack_query;

use crate::model::{
    codegen::{FrontendFramework, FullstackCodegenRequest},
    migration::DatabaseTable,
};
use std::collections::BTreeMap;

pub fn generate_frontend_files(
    req: &FullstackCodegenRequest,
    tables: &[DatabaseTable],
) -> Result<BTreeMap<String, String>, String> {
    match req.frontend_framework {
        FrontendFramework::Sveltekit => sveltekit::generate(req, tables),
        FrontendFramework::Nuxt => nuxt::generate(req, tables),
        FrontendFramework::Nextjs => nextjs::generate(req, tables),
    }
}

