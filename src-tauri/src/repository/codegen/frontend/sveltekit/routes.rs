pub(super) fn build_monolith_collection_route(kebab: &str, pascal: &str) -> String {
    format!(
        r#"import {{ {pascal}ServerRepository }} from "$lib/server/repositories/{kebab}.repository";

export async function GET({{ url }}) {{
  const page = Number(url.searchParams.get("page") ?? 0);
  const size = Number(url.searchParams.get("size") ?? 10);
  const result = await {pascal}ServerRepository.findAll(page, size);
  return Response.json(result);
}}

export async function POST({{ request }}) {{
  const payload = await request.json();
  const created = await {pascal}ServerRepository.insert(payload);
  return Response.json(created, {{ status: 201 }});
}}
"#
    )
}

pub(super) fn build_monolith_item_route(kebab: &str, pascal: &str) -> String {
    format!(
        r#"import {{ {pascal}ServerRepository }} from "$lib/server/repositories/{kebab}.repository";

export async function PUT({{ params, request }}) {{
  const payload = await request.json();
  const updated = await {pascal}ServerRepository.update(params.id, payload);
  return Response.json(updated);
}}

export async function DELETE({{ params }}) {{
  await {pascal}ServerRepository.delete(params.id);
  return new Response(null, {{ status: 204 }});
}}
"#
    )
}

pub(super) fn build_bff_collection_route(kebab: &str) -> String {
    format!(
        r#"import {{ BACKEND_BASE_URL }} from "$lib/server/backend";

const BACKEND_URL = `${{BACKEND_BASE_URL}}/{kebab}`;

export async function GET({{ url, fetch }}) {{
  const res = await fetch(`${{BACKEND_URL}}?${{url.searchParams.toString()}}`);
  return Response.json(await res.json(), {{ status: res.status }});
}}

export async function POST({{ request, fetch }}) {{
  const body = await request.text();
  const res = await fetch(BACKEND_URL, {{
    method: "POST",
    headers: {{ "Content-Type": "application/json" }},
    body,
  }});
  return Response.json(await res.json(), {{ status: res.status }});
}}
"#
    )
}

pub(super) fn build_bff_item_route(kebab: &str) -> String {
    format!(
        r#"import {{ BACKEND_BASE_URL }} from "$lib/server/backend";

const BACKEND_URL = `${{BACKEND_BASE_URL}}/{kebab}`;

export async function PUT({{ params, request, fetch }}) {{
  const body = await request.text();
  const res = await fetch(`${{BACKEND_URL}}/${{params.id}}`, {{
    method: "PUT",
    headers: {{ "Content-Type": "application/json" }},
    body,
  }});
  return Response.json(await res.json(), {{ status: res.status }});
}}

export async function DELETE({{ params, fetch }}) {{
  const res = await fetch(`${{BACKEND_URL}}/${{params.id}}`, {{ method: "DELETE" }});
  return new Response(null, {{ status: res.status }});
}}
"#
    )
}
