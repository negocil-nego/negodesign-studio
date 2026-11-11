use super::{
    admin_props::build_admin_config_ts,
    fullstack_db::{build_db_client_ts, build_direct_table_repository_ts},
    http_client::build_api_client_ts,
    tanstack_query::build_tanstack_query_ts,
};
use crate::{
    model::{
        codegen::{ApiCallPattern, ArchitectureMode, FullstackCodegenRequest},
        migration::DatabaseTable,
    },
    repository::codegen::naming::{to_kebab_case, to_pascal_case},
};
use std::collections::BTreeMap;

pub fn generate(
    req: &FullstackCodegenRequest,
    tables: &[DatabaseTable],
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();

    if req.architecture_mode == ArchitectureMode::Monolith {
        files.insert(
            "server/utils/db.server.ts".to_string(),
            build_db_client_ts(&req.database_engine),
        );
    }

    for table in tables {
        let kebab = to_kebab_case(&table.name);
        let pascal = to_pascal_case(&table.name);

        files.insert(
            format!("composables/features/{kebab}/{kebab}.admin.ts"),
            build_admin_config_ts(table, tables),
        );
        files.insert(
            format!("composables/features/{kebab}/{kebab}.api.ts"),
            build_api_client_ts(req, table),
        );

        if req.use_tanstack_query {
            files.insert(
                format!("composables/features/{kebab}/{kebab}.query.ts"),
                build_tanstack_query_ts(&req.frontend_framework, table),
            );
        }

        if req.architecture_mode == ArchitectureMode::Monolith {
            files.insert(
                format!("server/repositories/{kebab}.repository.ts"),
                build_direct_table_repository_ts(&req.database_engine, table, "../utils/db.server"),
            );
            files.insert(
                format!("server/api/{kebab}/index.ts"),
                build_nuxt_monolith_collection_route(&kebab, &pascal),
            );
            files.insert(
                format!("server/api/{kebab}/[id].ts"),
                build_nuxt_monolith_item_route(&kebab, &pascal),
            );
        } else if req.api_call_pattern == ApiCallPattern::Bff {
            let base = req
                .backend_base_url
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("http://localhost:8080");
            files.insert(
                format!("server/api/{kebab}/index.ts"),
                build_nuxt_bff_route(base, &kebab),
            );
        }

        files.insert(
            format!("pages/admin/{kebab}.vue"),
            build_nuxt_admin_page(&kebab, &pascal),
        );
    }

    Ok(files)
}

fn build_nuxt_admin_page(kebab: &str, pascal: &str) -> String {
    format!(
        "<script setup lang=\"ts\">\n\
import {{ computed, ref, onMounted }} from \"vue\";\n\
import AdminCrud from \"../../negodesign/project/ui/pages/admin/ui/AdminCrud.vue\";\n\
import {{ {pascal}Api }} from \"../../composables/features/{kebab}/{kebab}.api\";\n\
import {{ create{pascal}AdminProps }} from \"../../composables/features/{kebab}/{kebab}.admin\";\n\n\
const records = ref<Record<string, any>[]>([]);\n\
const adminProps = computed(() => create{pascal}AdminProps(records.value));\n\n\
onMounted(async () => {{\n\
  try {{\n\
    const res: any = await {pascal}Api.findAll(0, 10);\n\
    const payload = res?.data?.value ?? res?.data ?? res;\n\
    records.value = Array.isArray(payload?.data) ? payload.data : (Array.isArray(payload) ? payload : []);\n\
  }} catch (err) {{\n\
    console.error(\"Failed to load {kebab}:\", err);\n\
  }}\n\
}});\n\
</script>\n\n\
<template>\n\
  <AdminCrud v-bind=\"adminProps\" />\n\
</template>\n"
    )
}

fn build_nuxt_monolith_collection_route(kebab: &str, pascal: &str) -> String {
    format!(
        "import {{ defineEventHandler, getQuery, readBody }} from \"h3\";\n\
import {{ {pascal}ServerRepository }} from \"../../repositories/{kebab}.repository\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  if (event.method === \"POST\") {{\n\
    const body = await readBody(event);\n\
    return {pascal}ServerRepository.insert(body);\n\
  }}\n\
  const query = getQuery(event);\n\
  return {pascal}ServerRepository.findAll(Number(query.page ?? 0), Number(query.size ?? 10));\n\
}});\n"
    )
}

fn build_nuxt_monolith_item_route(kebab: &str, pascal: &str) -> String {
    format!(
        "import {{ defineEventHandler, getRouterParam, readBody }} from \"h3\";\n\
import {{ {pascal}ServerRepository }} from \"../../repositories/{kebab}.repository\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  const id = getRouterParam(event, \"id\") ?? \"\";\n\
  if (event.method === \"PUT\") {{\n\
    const body = await readBody(event);\n\
    return {pascal}ServerRepository.update(id, body);\n\
  }}\n\
  if (event.method === \"DELETE\") {{\n\
    await {pascal}ServerRepository.delete(id);\n\
    return null;\n\
  }}\n\
}});\n"
    )
}

fn build_nuxt_bff_route(base_url: &str, kebab: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!(
        "import {{ defineEventHandler, getQuery, readBody }} from \"h3\";\n\n\
const BACKEND_URL = \"{base}/api/{kebab}\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  if (event.method === \"POST\") {{\n\
    const body = await readBody(event);\n\
    return $fetch(BACKEND_URL, {{ method: \"POST\", body }});\n\
  }}\n\
  const query = getQuery(event);\n\
  return $fetch(BACKEND_URL, {{ query }});\n\
}});\n"
    )
}

