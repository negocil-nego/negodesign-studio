pub mod aspnet;
pub mod laravel;
pub mod nestjs;
pub mod spring_boot;

use crate::model::{
    codegen::{BackendFramework, FullstackCodegenRequest},
    migration::DatabaseTable,
};
use std::collections::BTreeMap;

pub fn generate_backend_files(
    req: &FullstackCodegenRequest,
    tables: &[DatabaseTable],
) -> Result<BTreeMap<String, String>, String> {
    let Some(framework) = req.backend_framework.as_ref() else {
        return Ok(BTreeMap::new());
    };
    let orm = req.orm_strategy.as_deref().unwrap_or("default");

    match framework {
        BackendFramework::SpringBoot => {
            let pkg = req
                .base_package
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("com.negocil.negoturismo.admin");
            spring_boot::generate(tables, pkg, orm)
        }
        BackendFramework::Nestjs => nestjs::generate(tables, orm),
        BackendFramework::Laravel => laravel::generate(tables, orm),
        BackendFramework::AspnetCore => aspnet::generate(tables, orm),
    }
}

