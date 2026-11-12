use crate::model::codegen::{FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::to_kebab_case;
use crate::repository::codegen::openapi::client::{path_with_id_expr, resolve_entity_endpoints};
use std::collections::BTreeMap;

pub fn generate_bff_routes(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> BTreeMap<String, String> {
    let mut files = BTreeMap::new();
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

    let update_method = &resolved.update_method;
    let delete_method = &resolved.delete_method;

    match req.frontend_framework {
        FrontendFramework::Sveltekit => {
            files.insert(
                format!("src/routes/api/{kebab}/+server.ts"),
                format!(
                    "import {{ json, type RequestHandler }} from \"@sveltejs/kit\";\n\n\
const LIST_URL = \"{list_url}\";\n\
const CREATE_URL = \"{create_url}\";\n\n\
export const GET: RequestHandler = async ({{ fetch, url }}) => {{\n\
  const res = await fetch(`${{LIST_URL}}${{url.search}}`, {{\n\
    headers: {{ Accept: \"application/json\" }},\n\
  }});\n\
  const data = await res.json();\n\
  return json(data, {{ status: res.status }});\n\
}};\n\n\
export const POST: RequestHandler = async ({{ fetch, request }}) => {{\n\
  const body = await request.json();\n\
  const res = await fetch(CREATE_URL, {{\n\
    method: \"POST\",\n\
    headers: {{ \"Content-Type\": \"application/json\", Accept: \"application/json\" }},\n\
    body: JSON.stringify(body),\n\
  }});\n\
  const data = await res.json();\n\
  return json(data, {{ status: res.status }});\n\
}};\n"
                ),
            );
            files.insert(
                format!("src/routes/api/{kebab}/[id]/+server.ts"),
                format!(
                    "import {{ json, type RequestHandler }} from \"@sveltejs/kit\";\n\n\
export const GET: RequestHandler = async ({{ fetch, params }}) => {{\n\
  const res = await fetch(`{get_id_svelte}`);\n\
  const data = await res.json();\n\
  return json(data, {{ status: res.status }});\n\
}};\n\n\
export const PUT: RequestHandler = async ({{ fetch, params, request }}) => {{\n\
  const body = await request.json();\n\
  const res = await fetch(`{update_id_svelte}`, {{\n\
    method: \"{update_method}\",\n\
    headers: {{ \"Content-Type\": \"application/json\", Accept: \"application/json\" }},\n\
    body: JSON.stringify(body),\n\
  }});\n\
  const data = await res.json();\n\
  return json(data, {{ status: res.status }});\n\
}};\n\n\
export const DELETE: RequestHandler = async ({{ fetch, params }}) => {{\n\
  const res = await fetch(`{delete_id_svelte}`, {{ method: \"{delete_method}\" }});\n\
  return new Response(null, {{ status: res.status }});\n\
}};\n"
                ),
            );
        }
        FrontendFramework::Nuxt => {
            files.insert(
                format!("server/api/{kebab}/index.ts"),
                format!(
                    "import {{ defineEventHandler, getQuery, readBody }} from \"h3\";\n\n\
const LIST_URL = \"{list_url}\";\n\
const CREATE_URL = \"{create_url}\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  const method = event.node.req.method;\n\
  if (method === \"GET\") {{\n\
    const query = getQuery(event);\n\
    return $fetch(LIST_URL, {{ query }});\n\
  }}\n\
  if (method === \"POST\") {{\n\
    const body = await readBody(event);\n\
    return $fetch(CREATE_URL, {{ method: \"POST\", body }});\n\
  }}\n\
}});\n"
                ),
            );
            files.insert(
                format!("server/api/{kebab}/[id].ts"),
                format!(
                    "import {{ defineEventHandler, getRouterParam, readBody }} from \"h3\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  const id = getRouterParam(event, \"id\");\n\
  const method = event.node.req.method;\n\
  if (method === \"GET\") {{\n\
    return $fetch(`{get_id_generic}`);\n\
  }}\n\
  if (method === \"PUT\" || method === \"PATCH\") {{\n\
    const body = await readBody(event);\n\
    return $fetch(`{update_id_generic}`, {{ method: \"{update_method}\", body }});\n\
  }}\n\
  if (method === \"DELETE\") {{\n\
    return $fetch(`{delete_id_generic}`, {{ method: \"{delete_method}\" }});\n\
  }}\n\
}});\n"
                ),
            );
        }
        FrontendFramework::Nextjs => {
            files.insert(
                format!("app/api/{kebab}/route.ts"),
                format!(
                    "import {{ NextRequest, NextResponse }} from \"next/server\";\n\n\
const LIST_URL = \"{list_url}\";\n\
const CREATE_URL = \"{create_url}\";\n\n\
export async function GET(request: NextRequest) {{\n\
  const search = request.nextUrl.search;\n\
  const res = await fetch(`${{LIST_URL}}${{search}}`, {{\n\
    headers: {{ Accept: \"application/json\" }},\n\
  }});\n\
  const data = await res.json();\n\
  return NextResponse.json(data, {{ status: res.status }});\n\
}}\n\n\
export async function POST(request: NextRequest) {{\n\
  const body = await request.json();\n\
  const res = await fetch(CREATE_URL, {{\n\
    method: \"POST\",\n\
    headers: {{ \"Content-Type\": \"application/json\", Accept: \"application/json\" }},\n\
    body: JSON.stringify(body),\n\
  }});\n\
  const data = await res.json();\n\
  return NextResponse.json(data, {{ status: res.status }});\n\
}}\n"
                ),
            );
            files.insert(
                format!("app/api/{kebab}/[id]/route.ts"),
                format!(
                    "import {{ NextRequest, NextResponse }} from \"next/server\";\n\n\
export async function GET(\n\
  _request: NextRequest,\n\
  {{ params }}: {{ params: Promise<{{ id: string }}> }}\n\
) {{\n\
  const {{ id }} = await params;\n\
  const res = await fetch(`{get_id_generic}`);\n\
  const data = await res.json();\n\
  return NextResponse.json(data, {{ status: res.status }});\n\
}}\n\n\
export async function PUT(\n\
  request: NextRequest,\n\
  {{ params }}: {{ params: Promise<{{ id: string }}> }}\n\
) {{\n\
  const {{ id }} = await params;\n\
  const body = await request.json();\n\
  const res = await fetch(`{update_id_generic}`, {{\n\
    method: \"{update_method}\",\n\
    headers: {{ \"Content-Type\": \"application/json\", Accept: \"application/json\" }},\n\
    body: JSON.stringify(body),\n\
  }});\n\
  const data = await res.json();\n\
  return NextResponse.json(data, {{ status: res.status }});\n\
}}\n\n\
export async function DELETE(\n\
  _request: NextRequest,\n\
  {{ params }}: {{ params: Promise<{{ id: string }}> }}\n\
) {{\n\
  const {{ id }} = await params;\n\
  const res = await fetch(`{delete_id_generic}`, {{ method: \"{delete_method}\" }});\n\
  return new NextResponse(null, {{ status: res.status }});\n\
}}\n"
                ),
            );
        }
    }
    files
}
