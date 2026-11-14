use super::BffRouteCtx;
use std::collections::BTreeMap;

pub(super) fn generate_sveltekit_bff_routes(ctx: &BffRouteCtx) -> BTreeMap<String, String> {
    let BffRouteCtx {
        kebab,
        backend_base,
        list_url,
        create_url,
        get_id_svelte,
        update_id_svelte,
        delete_id_svelte,
        update_method,
        delete_method,
        ..
    } = ctx;

    // Expressão que compõe a URL a partir do BACKEND_BASE_URL partilhado
    // (src/lib/server/backend.ts). Se a URL não começar na base esperada
    // (ex.: entidade com server_url própria), mantém o literal completo.
    let url_expr = |url: &str| -> String {
        match url.strip_prefix(backend_base) {
            Some(path) => format!("`${{BACKEND_BASE_URL}}{path}`"),
            None => format!("`{url}`"),
        }
    };

    let list_expr = url_expr(list_url);
    let create_expr = url_expr(create_url);
    let get_expr = url_expr(get_id_svelte);
    let update_expr = url_expr(update_id_svelte);
    let delete_expr = url_expr(delete_id_svelte);

    let mut files = BTreeMap::new();

    files.insert(
        format!("src/routes/api/{kebab}/+server.ts"),
        format!(
            r#"import type {{ RequestHandler }} from "@sveltejs/kit";
import {{ BACKEND_BASE_URL }} from "$lib/server/backend";

const LIST_URL = {list_expr};
const CREATE_URL = {create_expr};

export const GET: RequestHandler = async ({{ fetch, url }}) => {{
  const res = await fetch(`${{LIST_URL}}${{url.search}}`, {{
    headers: {{ Accept: "application/json" }},
  }});
  const data = await res.json();
  return Response.json(data, {{ status: res.status }});
}};

export const POST: RequestHandler = async ({{ fetch, request }}) => {{
  const body = await request.json();
  const res = await fetch(CREATE_URL, {{
    method: "POST",
    headers: {{ "Content-Type": "application/json", Accept: "application/json" }},
    body: JSON.stringify(body),
  }});
  const data = await res.json();
  return Response.json(data, {{ status: res.status }});
}};
"#
        ),
    );

    files.insert(
        format!("src/routes/api/{kebab}/[id]/+server.ts"),
        format!(
            r#"import type {{ RequestHandler }} from "@sveltejs/kit";
import {{ BACKEND_BASE_URL }} from "$lib/server/backend";

export const GET: RequestHandler = async ({{ fetch, params }}) => {{
  const res = await fetch({get_expr}, {{
    headers: {{ Accept: "application/json" }},
  }});
  const data = await res.json();
  return Response.json(data, {{ status: res.status }});
}};

export const PUT: RequestHandler = async ({{ fetch, params, request }}) => {{
  const body = await request.json();
  const res = await fetch({update_expr}, {{
    method: "{update_method}",
    headers: {{ "Content-Type": "application/json", Accept: "application/json" }},
    body: JSON.stringify(body),
  }});
  const data = await res.json();
  return Response.json(data, {{ status: res.status }});
}};

export const DELETE: RequestHandler = async ({{ fetch, params }}) => {{
  const res = await fetch({delete_expr}, {{ method: "{delete_method}" }});
  return new Response(null, {{ status: res.status }});
}};
"#
        ),
    );

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_use_response_json_and_shared_base_url() {
        let kebab = "products".to_string();
        let backend_base = "http://localhost:8080";
        let list_url = "http://localhost:8080/api/v1/admin/products".to_string();
        let create_url = "http://localhost:8080/api/v1/admin/products".to_string();
        let get_id = "http://localhost:8080/api/v1/admin/products/${params.id}".to_string();
        let update_id = "http://localhost:8080/api/v1/admin/products/${params.id}".to_string();
        let delete_id = "http://localhost:8080/api/v1/admin/products/${params.id}".to_string();

        let ctx = BffRouteCtx {
            kebab: &kebab,
            backend_base,
            list_url: &list_url,
            create_url: &create_url,
            get_id_svelte: &get_id,
            update_id_svelte: &update_id,
            delete_id_svelte: &delete_id,
            get_id_generic: &get_id,
            update_id_generic: &update_id,
            delete_id_generic: &delete_id,
            update_method: "PUT",
            delete_method: "DELETE",
        };

        let files = generate_sveltekit_bff_routes(&ctx);

        let list = &files["src/routes/api/products/+server.ts"];
        assert!(
            !list.contains("import { json"),
            "json do @sveltejs/kit está depreciado"
        );
        assert!(list.contains("import type { RequestHandler } from \"@sveltejs/kit\";"));
        assert!(list.contains("import { BACKEND_BASE_URL } from \"$lib/server/backend\";"));
        assert!(
            list.contains("const LIST_URL = `${BACKEND_BASE_URL}/api/v1/admin/products`;"),
            "a URL deve reutilizar a base partilhada: {list}"
        );
        assert!(list.contains("return Response.json(data, { status: res.status });"));
        assert!(list.contains("\n  const res = await fetch("), "código sem indentação: {list}");

        let item = &files["src/routes/api/products/[id]/+server.ts"];
        assert!(!item.contains("import { json"));
        assert!(item.contains("return Response.json(data, { status: res.status });"));
        assert!(item.contains("return new Response(null, { status: res.status });"));
        // A URL com params.id tem de ficar dentro do handler (params não existe no escopo do módulo)
        assert!(item.contains("\n  const res = await fetch(`${BACKEND_BASE_URL}/api/v1/admin/products/${params.id}`, {"));
        assert!(item.contains("async ({ fetch, params }) => {"));
    }

    #[test]
    fn keeps_full_url_when_entity_uses_own_server() {
        let kebab = "products".to_string();
        let backend_base = "http://localhost:8080";
        let own = "https://api.exemplo.pt/v1/products".to_string();
        let none = "https://api.exemplo.pt/v1/products".to_string();

        let ctx = BffRouteCtx {
            kebab: &kebab,
            backend_base,
            list_url: &own,
            create_url: &own,
            get_id_svelte: &none,
            update_id_svelte: &none,
            delete_id_svelte: &none,
            get_id_generic: &none,
            update_id_generic: &none,
            delete_id_generic: &none,
            update_method: "PUT",
            delete_method: "DELETE",
        };

        let files = generate_sveltekit_bff_routes(&ctx);
        let list = &files["src/routes/api/products/+server.ts"];
        assert!(
            list.contains("const LIST_URL = `https://api.exemplo.pt/v1/products`;"),
            "sem correspondência na base, mantém o literal completo: {list}"
        );
    }
}
