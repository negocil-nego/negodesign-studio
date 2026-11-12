use crate::model::migration::OpenApiOperation;

const LIST_NAMED_SEGMENTS: &[&str] = &[
    "list",
    "all",
    "find-all",
    "findall",
    "find_all",
    "get-all",
    "getall",
    "get_all",
    "search",
    "index",
    "page",
    "paginate",
    "paginated",
    "query",
    "fetch",
    "items",
    "catalog",
    "records",
];

const GET_ONE_NAMED_SEGMENTS: &[&str] = &[
    "detail",
    "details",
    "by-id",
    "byid",
    "by_id",
    "find-by-id",
    "get-by-id",
    "show",
    "view",
    "one",
];

const CREATE_NAMED_SEGMENTS: &[&str] = &[
    "create",
    "add",
    "insert",
    "new",
    "store",
    "register",
    "post",
];

const UPDATE_NAMED_SEGMENTS: &[&str] = &[
    "update",
    "edit",
    "save",
    "modify",
    "change",
    "patch",
    "upsert",
    "put",
];

const DELETE_NAMED_SEGMENTS: &[&str] = &[
    "delete",
    "remove",
    "destroy",
    "erase",
    "del",
    "drop",
    "archive",
];

const IGNORED_PATH_PREFIXES: &[&str] = &[
    "api", "v1", "v2", "v3", "v4", "admin", "public", "private", "internal", "rest", "web",
];

/// Classifies an OpenAPI endpoint into `(crud_role, pattern_style)` where:
/// - `crud_role`: `"list"` | `"get"` | `"create"` | `"update"` | `"delete"`
/// - `pattern_style`: `"restful"` | `"named"`
pub fn classify_operation(
    method: &str,
    path: &str,
    operation_id: Option<&str>,
) -> (String, String) {
    let m = method.trim().to_ascii_uppercase();
    let segments = normalized_segments(path);
    let op_lower = operation_id.unwrap_or("").to_ascii_lowercase();

    let has_seg = |candidates: &[&str]| {
        segments
            .iter()
            .any(|seg| candidates.iter().any(|c| seg == c))
    };

    let has_op_token = |candidates: &[&str]| {
        !op_lower.is_empty()
            && candidates.iter().any(|c| {
                op_lower == *c
                    || op_lower.starts_with(&format!("{c}_"))
                    || op_lower.starts_with(c)
                    || op_lower.ends_with(c)
            })
    };

    let ends_with_param = path
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .map(|s| s.starts_with('{') && s.ends_with('}'))
        .unwrap_or(false);

    match m.as_str() {
        "GET" => {
            if has_seg(LIST_NAMED_SEGMENTS) {
                ("list".to_string(), "named".to_string())
            } else if has_seg(GET_ONE_NAMED_SEGMENTS) {
                ("get".to_string(), "named".to_string())
            } else if ends_with_param {
                ("get".to_string(), "restful".to_string())
            } else if has_op_token(&["list", "findall", "find_all", "getall", "get_all", "search", "paginate"]) {
                ("list".to_string(), "restful".to_string())
            } else {
                ("list".to_string(), "restful".to_string())
            }
        }
        "POST" => {
            if has_seg(DELETE_NAMED_SEGMENTS) {
                ("delete".to_string(), "named".to_string())
            } else if has_seg(UPDATE_NAMED_SEGMENTS) {
                ("update".to_string(), "named".to_string())
            } else if has_seg(LIST_NAMED_SEGMENTS) {
                ("list".to_string(), "named".to_string())
            } else if has_seg(CREATE_NAMED_SEGMENTS) {
                ("create".to_string(), "named".to_string())
            } else {
                ("create".to_string(), "restful".to_string())
            }
        }
        "PUT" | "PATCH" => {
            if has_seg(UPDATE_NAMED_SEGMENTS) {
                ("update".to_string(), "named".to_string())
            } else {
                ("update".to_string(), "restful".to_string())
            }
        }
        "DELETE" => {
            if has_seg(DELETE_NAMED_SEGMENTS) {
                ("delete".to_string(), "named".to_string())
            } else {
                ("delete".to_string(), "restful".to_string())
            }
        }
        _ => ("list".to_string(), "restful".to_string()),
    }
}

/// Normalizes a schema/entity name to its canonical singular stem (e.g., `CustomerResponse` -> `customer`).
pub fn canonical_entity_stem(name: &str) -> String {
    let mut s = name.trim().to_ascii_lowercase();
    for prefix in [
        "create_", "update_", "delete_", "list_", "get_", "response_", "request_", "dto_",
        "create", "update", "delete",
    ] {
        if s.len() > prefix.len() + 2 && s.starts_with(prefix) {
            s = s[prefix.len()..].to_string();
            break;
        }
    }
    for suffix in [
        "_response", "response", "_request", "request", "_dto", "dto", "_model", "model",
        "_entity", "entity", "_schema", "schema", "_input", "input", "_output", "output",
        "_payload", "payload", "_vo", "vo", "_item", "item",
    ] {
        if s.len() > suffix.len() + 1 && s.ends_with(suffix) {
            s = s[..s.len() - suffix.len()].to_string();
            break;
        }
    }
    singularize(&s.replace(['-', '_', ' '], ""))
}

/// Checks if an operation belongs to the given entity schema name either via `$ref` or via URL path resource stem.
pub fn operation_matches_entity(
    schema_name: &str,
    refs: &[String],
    op: &OpenApiOperation,
) -> bool {
    if refs.iter().any(|r| r == schema_name) {
        return true;
    }
    let entity_stem = canonical_entity_stem(schema_name);
    if entity_stem.is_empty() {
        return false;
    }
    if refs
        .iter()
        .any(|r| canonical_entity_stem(r) == entity_stem)
    {
        return true;
    }
    if let Some(path_stem) = extract_resource_stem_from_path(&op.path) {
        if path_stem == entity_stem {
            return true;
        }
    }
    false
}

fn extract_resource_stem_from_path(path: &str) -> Option<String> {
    let segments = normalized_segments(path);
    for seg in segments {
        if IGNORED_PATH_PREFIXES.contains(&seg.as_str()) {
            continue;
        }
        if LIST_NAMED_SEGMENTS.contains(&seg.as_str())
            || GET_ONE_NAMED_SEGMENTS.contains(&seg.as_str())
            || CREATE_NAMED_SEGMENTS.contains(&seg.as_str())
            || UPDATE_NAMED_SEGMENTS.contains(&seg.as_str())
            || DELETE_NAMED_SEGMENTS.contains(&seg.as_str())
        {
            continue;
        }
        let cleaned = seg.replace(['-', '_'], "");
        if !cleaned.is_empty() {
            return Some(singularize(&cleaned));
        }
    }
    None
}

fn normalized_segments(path: &str) -> Vec<String> {
    path.split('/')
        .map(str::trim)
        .filter(|s| !s.is_empty() && !(s.starts_with('{') && s.ends_with('}')))
        .map(|s| s.to_ascii_lowercase())
        .collect()
}

fn singularize(word: &str) -> String {
    if let Some(stripped) = word.strip_suffix("ies") {
        if !stripped.is_empty() {
            return format!("{stripped}y");
        }
    }
    if let Some(stripped) = word.strip_suffix("ses") {
        if !stripped.is_empty() {
            return stripped.to_string();
        }
    }
    if word.len() > 3 && word.ends_with('s') && !word.ends_with("ss") && !word.ends_with("us") {
        return word[..word.len() - 1].to_string();
    }
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_restful_endpoints() {
        assert_eq!(
            classify_operation("GET", "/api/users", None),
            ("list".to_string(), "restful".to_string())
        );
        assert_eq!(
            classify_operation("GET", "/api/users/{id}", None),
            ("get".to_string(), "restful".to_string())
        );
        assert_eq!(
            classify_operation("POST", "/api/users", None),
            ("create".to_string(), "restful".to_string())
        );
        assert_eq!(
            classify_operation("PUT", "/api/users/{id}", None),
            ("update".to_string(), "restful".to_string())
        );
        assert_eq!(
            classify_operation("PATCH", "/api/users/{id}", None),
            ("update".to_string(), "restful".to_string())
        );
        assert_eq!(
            classify_operation("DELETE", "/api/users/{id}", None),
            ("delete".to_string(), "restful".to_string())
        );
    }

    #[test]
    fn classifies_named_method_endpoints() {
        assert_eq!(
            classify_operation("GET", "/api/users/list", None),
            ("list".to_string(), "named".to_string())
        );
        assert_eq!(
            classify_operation("GET", "/api/users/all", None),
            ("list".to_string(), "named".to_string())
        );
        assert_eq!(
            classify_operation("POST", "/api/users/create", None),
            ("create".to_string(), "named".to_string())
        );
        assert_eq!(
            classify_operation("POST", "/api/users/add", None),
            ("create".to_string(), "named".to_string())
        );
        assert_eq!(
            classify_operation("PUT", "/api/users/update/{id}", None),
            ("update".to_string(), "named".to_string())
        );
        assert_eq!(
            classify_operation("PATCH", "/api/users/edit/{id}", None),
            ("update".to_string(), "named".to_string())
        );
        assert_eq!(
            classify_operation("PUT", "/api/users/save", None),
            ("update".to_string(), "named".to_string())
        );
        assert_eq!(
            classify_operation("DELETE", "/api/users/delete/{id}", None),
            ("delete".to_string(), "named".to_string())
        );
        assert_eq!(
            classify_operation("DELETE", "/api/users/remove/{id}", None),
            ("delete".to_string(), "named".to_string())
        );
    }
}
