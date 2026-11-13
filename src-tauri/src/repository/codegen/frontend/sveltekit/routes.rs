pub(super) fn build_monolith_collection_route(kebab: &str, pascal: &str) -> String {
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

pub(super) fn build_monolith_item_route(kebab: &str, pascal: &str) -> String {
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

pub(super) fn build_bff_collection_route(base_url: &str, kebab: &str) -> String {
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

pub(super) fn build_bff_item_route(base_url: &str, kebab: &str) -> String {
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
