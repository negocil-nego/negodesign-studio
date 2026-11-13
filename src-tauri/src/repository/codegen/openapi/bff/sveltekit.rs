use super::BffRouteCtx;
use std::collections::BTreeMap;

pub(super) fn generate_sveltekit_bff_routes(ctx: &BffRouteCtx) -> BTreeMap<String, String> {
    let BffRouteCtx {
        kebab,
        list_url,
        create_url,
        get_id_svelte,
        update_id_svelte,
        delete_id_svelte,
        update_method,
        delete_method,
        ..
    } = ctx;
    let mut files = BTreeMap::new();
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
    files
}
