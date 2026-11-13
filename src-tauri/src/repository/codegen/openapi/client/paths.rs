use crate::model::codegen::{ApiCallPattern, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::to_kebab_case;
use super::resolve::ResolvedCrudEndpoints;

/// Converts an OpenAPI path template (e.g. `/v1/users/{id}` or `/v1/users/update`) into a JS template literal with `${id}`.
pub fn path_with_id_expr(path: &str, id_expr: &str) -> String {
    let mut out = String::new();
    let mut replaced = false;
    let mut chars = path.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            while let Some(&c) = chars.peek() {
                chars.next();
                if c == '}' {
                    break;
                }
            }
            if !replaced {
                out.push_str(&format!("${{{id_expr}}}"));
                replaced = true;
            }
        } else {
            out.push(ch);
        }
    }
    if !replaced {
        format!("{}/${{{id_expr}}}", out.trim_end_matches('/'))
    } else {
        out
    }
}

pub(super) fn resolve_client_urls(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
    resolved: &ResolvedCrudEndpoints,
) -> (String, String, String, String, String) {
    let kebab = to_kebab_case(&entity.name);
    match req.api_call_pattern {
        ApiCallPattern::Bff => (
            format!("/api/{kebab}"),
            format!("/api/{kebab}/${{id}}"),
            format!("/api/{kebab}"),
            format!("/api/{kebab}/${{id}}"),
            format!("/api/{kebab}/${{id}}"),
        ),
        ApiCallPattern::Direct => {
            let backend_url = req
                .backend_base_url
                .as_deref()
                .or(entity.server_url.as_deref())
                .unwrap_or("http://localhost:8080")
                .trim_end_matches('/');
            (
                format!("{backend_url}{}", resolved.list_path),
                format!("{backend_url}{}", path_with_id_expr(&resolved.get_path, "id")),
                format!("{backend_url}{}", resolved.create_path),
                format!(
                    "{backend_url}{}",
                    path_with_id_expr(&resolved.update_path, "id")
                ),
                format!(
                    "{backend_url}{}",
                    path_with_id_expr(&resolved.delete_path, "id")
                ),
            )
        }
    }
}
