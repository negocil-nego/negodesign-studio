use super::BffRouteCtx;
use std::collections::BTreeMap;

pub(super) fn generate_nextjs_bff_routes(ctx: &BffRouteCtx) -> BTreeMap<String, String> {
    let BffRouteCtx {
        kebab,
        list_url,
        create_url,
        get_id_generic,
        update_id_generic,
        delete_id_generic,
        update_method,
        delete_method,
        ..
    } = ctx;
    let mut files = BTreeMap::new();
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
    files
}
