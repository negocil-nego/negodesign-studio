pub(super) fn build_next_monolith_collection_route(kebab: &str, pascal: &str) -> String {
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

pub(super) fn build_next_monolith_item_route(kebab: &str, pascal: &str) -> String {
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

pub(super) fn build_next_bff_collection_route(base_url: &str, kebab: &str) -> String {
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

pub(super) fn build_next_bff_item_route(base_url: &str, kebab: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!(
        "import {{ NextResponse }} from \"next/server\";\n\n\
const BACKEND_URL = \"{base}/api/{kebab}\";\n\n\
export async function PUT(request: Request, {{ params }}: {{ params: Promise<{{ id: string }}> }}) {{\n\
  const {{ id }} = await params;\n\
  const body = await request.text();\n\
  const res = await fetch(`${{BACKEND_URL}}/${{id}}`, {{\n\
    method: \"PUT\",\n\
    headers: {{ \"Content-Type\": \"application/json\" }},\n\
    body,\n\
  }});\n\
  return NextResponse.json(await res.json(), {{ status: res.status }});\n\
}}\n\n\
export async function DELETE(_request: Request, {{ params }}: {{ params: Promise<{{ id: string }}> }}) {{\n\
  const {{ id }} = await params;\n\
  const res = await fetch(`${{BACKEND_URL}}/${{id}}`, {{ method: \"DELETE\" }});\n\
  return new NextResponse(null, {{ status: res.status }});\n\
}}\n"
    )
}
