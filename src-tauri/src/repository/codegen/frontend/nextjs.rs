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
            format!("src/features/{kebab}/{kebab}.admin.ts"),
            build_admin_config_ts(table, tables),
        );
        files.insert(
            format!("src/features/{kebab}/{kebab}.api.ts"),
            build_api_client_ts(req, table),
        );

        if req.use_tanstack_query {
            files.insert(
                format!("src/features/{kebab}/{kebab}.query.ts"),
                build_tanstack_query_ts(&req.frontend_framework, table),
            );
        }

        if req.architecture_mode == ArchitectureMode::Monolith {
            files.insert(
                format!("src/lib/server/repositories/{kebab}.repository.ts"),
                build_direct_table_repository_ts(&req.database_engine, table, "../db.server"),
            );
            files.insert(
                format!("src/app/api/{kebab}/route.ts"),
                build_next_monolith_collection_route(&kebab, &pascal),
            );
            files.insert(
                format!("src/app/api/{kebab}/[id]/route.ts"),
                build_next_monolith_item_route(&kebab, &pascal),
            );
        } else if req.api_call_pattern == ApiCallPattern::Bff {
            let base = req
                .backend_base_url
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("http://localhost:8080");
            files.insert(
                format!("src/app/api/{kebab}/route.ts"),
                build_next_bff_collection_route(base, &kebab),
            );
        }

        files.insert(
            format!("src/app/admin/{kebab}/page.tsx"),
            build_next_admin_page(&kebab, &pascal),
        );
    }

    Ok(files)
}

fn build_next_admin_page(kebab: &str, pascal: &str) -> String {
    format!(
        "\"use client\";\n\n\
import {{ useEffect, useState }} from \"react\";\n\
import AdminCrud from \"../../../../negodesign/project/ui/pages/admin/ui/AdminCrud\";\n\
import {{ {pascal}Api }} from \"../../../features/{kebab}/{kebab}.api\";\n\
import {{ create{pascal}AdminProps }} from \"../../../features/{kebab}/{kebab}.admin\";\n\n\
export default function {pascal}AdminPage() {{\n\
  const [records, setRecords] = useState<Record<string, any>[]>([]);\n\n\
  useEffect(() => {{\n\
    {pascal}Api.findAll(0, 10)\n\
      .then((res) => setRecords(Array.isArray(res?.data) ? res.data : (Array.isArray(res) ? res : [])))\n\
      .catch((err) => console.error(\"Failed to load {kebab}:\", err));\n\
  }}, []);\n\n\
  const adminProps = create{pascal}AdminProps(records);\n\
  return <AdminCrud {{...adminProps}} />;\n\
}}\n"
    )
}

fn build_next_monolith_collection_route(kebab: &str, pascal: &str) -> String {
    format!(
        "import {{ NextResponse }} from \"next/server\";\n\
import {{ {pascal}ServerRepository }} from \"@/lib/server/repositories/{kebab}.repository\";\n\n\
export async function GET(request: Request) {{\n\
  const {{ searchParams }} = new URL(request.url);\n\
  const page = Number(searchParams.get(\"page\") ?? 0);\n\
  const size = Number(searchParams.get(\"size\") ?? 10);\n\
  const result = await {pascal}ServerRepository.findAll(page, size);\n\
  return NextResponse.json(result);\n\
}}\n\n\
export async function POST(request: Request) {{\n\
  const payload = await request.json();\n\
  const created = await {pascal}ServerRepository.insert(payload);\n\
  return NextResponse.json(created, {{ status: 201 }});\n\
}}\n"
    )
}

fn build_next_monolith_item_route(kebab: &str, pascal: &str) -> String {
    format!(
        "import {{ NextResponse }} from \"next/server\";\n\
import {{ {pascal}ServerRepository }} from \"@/lib/server/repositories/{kebab}.repository\";\n\n\
export async function PUT(request: Request, {{ params }}: {{ params: Promise<{{ id: string }}> }}) {{\n\
  const {{ id }} = await params;\n\
  const payload = await request.json();\n\
  const updated = await {pascal}ServerRepository.update(id, payload);\n\
  return NextResponse.json(updated);\n\
}}\n\n\
export async function DELETE(_request: Request, {{ params }}: {{ params: Promise<{{ id: string }}> }}) {{\n\
  const {{ id }} = await params;\n\
  await {pascal}ServerRepository.delete(id);\n\
  return new NextResponse(null, {{ status: 204 }});\n\
}}\n"
    )
}

fn build_next_bff_collection_route(base_url: &str, kebab: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!(
        "import {{ NextResponse }} from \"next/server\";\n\n\
const BACKEND_URL = \"{base}/api/{kebab}\";\n\n\
export async function GET(request: Request) {{\n\
  const {{ searchParams }} = new URL(request.url);\n\
  const res = await fetch(`${{BACKEND_URL}}?${{searchParams.toString()}}`);\n\
  return NextResponse.json(await res.json(), {{ status: res.status }});\n\
}}\n\n\
export async function POST(request: Request) {{\n\
  const body = await request.text();\n\
  const res = await fetch(BACKEND_URL, {{\n\
    method: \"POST\",\n\
    headers: {{ \"Content-Type\": \"application/json\" }},\n\
    body,\n\
  }});\n\
  return NextResponse.json(await res.json(), {{ status: res.status }});\n\
}}\n"
    )
}

