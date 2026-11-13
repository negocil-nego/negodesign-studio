pub mod aspnet;
pub mod laravel;
pub mod nestjs;
pub mod spring_boot;

use crate::model::{
    codegen::{BackendFramework, FullstackCodegenRequest, SpringLanguage},
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
            let detected_pkg = req
                .backend_path
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .and_then(|p| spring_boot::detect::detect_spring_base_package(std::path::Path::new(p)));

            let pkg = req
                .base_package
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty() && *s != "com.negocil.negoturismo.admin")
                .map(str::to_string)
                .or(detected_pkg)
                .unwrap_or_else(|| "com.nego.design".to_string());

            let language = req
                .spring_language
                .clone()
                .unwrap_or(SpringLanguage::Java);
            spring_boot::generate(
                tables,
                &pkg,
                orm,
                &language,
                req.database_connection.as_ref(),
                &req.database_engine,
                &req.frontend_framework,
            )
        }
        BackendFramework::Nestjs => nestjs::generate(tables, orm),
        BackendFramework::Laravel => laravel::generate(tables, orm),
        BackendFramework::AspnetCore => aspnet::generate(tables, orm),
    }
}

