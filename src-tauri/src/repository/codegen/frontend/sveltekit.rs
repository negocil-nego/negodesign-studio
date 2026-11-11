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
            "src/lib/server/db.server.ts".to_string(),
            build_db_client_ts(&req.database_engine),
        );
    }

    for table in tables {
        let kebab = to_kebab_case(&table.name);
        let pascal = to_pascal_case(&table.name);

        files.insert(
            format!("src/lib/features/{kebab}/{kebab}.admin.ts"),
            build_admin_config_ts(table, tables),
        );
        files.insert(
            format!("src/lib/features/{kebab}/{kebab}.api.ts"),
            build_api_client_ts(req, table),
        );

        if req.use_tanstack_query {
            files.insert(
                format!("src/lib/features/{kebab}/{kebab}.query.ts"),
                build_tanstack_query_ts(&req.frontend_framework, table),
            );
        }

        if req.architecture_mode == ArchitectureMode::Monolith {
            files.insert(
                format!("src/lib/server/repositories/{kebab}.repository.ts"),
                build_direct_table_repository_ts(&req.database_engine, table, "../db.server"),
            );
            files.insert(
                format!("src/routes/api/{kebab}/+server.ts"),
                build_monolith_collection_route(&kebab, &pascal),
            );
            files.insert(
                format!("src/routes/api/{kebab}/[id]/+server.ts"),
                build_monolith_item_route(&kebab, &pascal),
            );
        } else if req.api_call_pattern == ApiCallPattern::Bff {
            let base = req
                .backend_base_url
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("http://localhost:8080");
            files.insert(
                format!("src/routes/api/{kebab}/+server.ts"),
                build_bff_collection_route(base, &kebab),
            );
            files.insert(
                format!("src/routes/api/{kebab}/[id]/+server.ts"),
                build_bff_item_route(base, &kebab),
            );
        }

        files.insert(
            format!("src/routes/admin/{kebab}/+page.svelte"),
            build_svelte_admin_page(&kebab, &pascal),
        );
    }

    Ok(files)
}

fn build_svelte_admin_page(kebab: &str, pascal: &str) -> String {
    format!(
        "<script lang=\"ts\">\n\
  import {{ onMount }} from \"svelte\";\n\
  import AdminCrud from \"../../../../negodesign/project/ui/pages/admin/ui/AdminCrud.svelte\";\n\
  import type {{ AdminProps }} from \"../../../../negodesign/project/ui/pages/admin/data/types\";\n\
  import {{ {pascal}Api }} from \"$lib/features/{kebab}/{kebab}.api\";\n\
  import {{ create{pascal}AdminProps }} from \"$lib/features/{kebab}/{kebab}.admin\";\n\n\
  let records = $state<Record<string, any>[]>([]);\n\
  let adminProps: AdminProps = $derived(create{pascal}AdminProps(records));\n\
  let {{ datatableProps, sidebarProps, modalFormProps, menuProps }}: AdminProps = $derived(adminProps);\n\n\
  async function loadRecords() {{\n\
    try {{\n\
      const res = await {pascal}Api.findAll(0, 10);\n\
      records = Array.isArray(res?.data) ? res.data : (Array.isArray(res) ? res : []);\n\
    }} catch (err) {{\n\
      console.error(\"Failed to load {kebab}:\", err);\n\
    }}\n\
  }}\n\n\
  onMount(loadRecords);\n\
</script>\n\n\
<AdminCrud {{datatableProps}} {{sidebarProps}} {{modalFormProps}} {{menuProps}} />\n"
    )
}

fn build_monolith_collection_route(kebab: &str, pascal: &str) -> String {
    format!(
        "import {{ json }} from \"@sveltejs/kit\";\n\
import {{ {pascal}ServerRepository }} from \"$lib/server/repositories/{kebab}.repository\";\n\n\
export async function GET({{ url }}) {{\n\
  const page = Number(url.searchParams.get(\"page\") ?? 0);\n\
  const size = Number(url.searchParams.get(\"size\") ?? 10);\n\
  const result = await {pascal}ServerRepository.findAll(page, size);\n\
  return json(result);\n\
}}\n\n\
export async function POST({{ request }}) {{\n\
  const payload = await request.json();\n\
  const created = await {pascal}ServerRepository.insert(payload);\n\
  return json(created, {{ status: 201 }});\n\
}}\n"
    )
}

fn build_monolith_item_route(kebab: &str, pascal: &str) -> String {
    format!(
        "import {{ json }} from \"@sveltejs/kit\";\n\
import {{ {pascal}ServerRepository }} from \"$lib/server/repositories/{kebab}.repository\";\n\n\
export async function PUT({{ params, request }}) {{\n\
  const payload = await request.json();\n\
  const updated = await {pascal}ServerRepository.update(params.id, payload);\n\
  return json(updated);\n\
}}\n\n\
export async function DELETE({{ params }}) {{\n\
  await {pascal}ServerRepository.delete(params.id);\n\
  return new Response(null, {{ status: 204 }});\n\
}}\n"
    )
}

fn build_bff_collection_route(base_url: &str, kebab: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!(
        "import {{ json }} from \"@sveltejs/kit\";\n\n\
const BACKEND_URL = \"{base}/api/{kebab}\";\n\n\
export async function GET({{ url, fetch }}) {{\n\
  const res = await fetch(`${{BACKEND_URL}}?${{url.searchParams.toString()}}`);\n\
  return json(await res.json(), {{ status: res.status }});\n\
}}\n\n\
export async function POST({{ request, fetch }}) {{\n\
  const body = await request.text();\n\
  const res = await fetch(BACKEND_URL, {{\n\
    method: \"POST\",\n\
    headers: {{ \"Content-Type\": \"application/json\" }},\n\
    body,\n\
  }});\n\
  return json(await res.json(), {{ status: res.status }});\n\
}}\n"
    )
}

fn build_bff_item_route(base_url: &str, kebab: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!(
        "import {{ json }} from \"@sveltejs/kit\";\n\n\
const BACKEND_URL = \"{base}/api/{kebab}\";\n\n\
export async function PUT({{ params, request, fetch }}) {{\n\
  const body = await request.text();\n\
  const res = await fetch(`${{BACKEND_URL}}/${{params.id}}`, {{\n\
    method: \"PUT\",\n\
    headers: {{ \"Content-Type\": \"application/json\" }},\n\
    body,\n\
  }});\n\
  return json(await res.json(), {{ status: res.status }});\n\
}}\n\n\
export async function DELETE({{ params, fetch }}) {{\n\
  const res = await fetch(`${{BACKEND_URL}}/${{params.id}}`, {{ method: \"DELETE\" }});\n\
  return new Response(null, {{ status: res.status }});\n\
}}\n"
    )
}

