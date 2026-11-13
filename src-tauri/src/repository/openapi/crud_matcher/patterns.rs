use super::entity::normalized_segments;

pub(super) const LIST_NAMED_SEGMENTS: &[&str] = &[
    "list", "all", "find-all", "findall", "find_all", "get-all", "getall", "get_all",
    "search", "index", "page", "paginate", "paginated", "query", "fetch", "items",
    "catalog", "records",
];

pub(super) const GET_ONE_NAMED_SEGMENTS: &[&str] = &[
    "detail", "details", "by-id", "byid", "by_id", "find-by-id", "get-by-id", "show",
    "view", "one",
];

pub(super) const CREATE_NAMED_SEGMENTS: &[&str] = &["create", "add", "insert", "new", "store", "register", "post"];

pub(super) const UPDATE_NAMED_SEGMENTS: &[&str] = &[
    "update", "edit", "save", "modify", "change", "patch", "upsert", "put",
];

pub(super) const DELETE_NAMED_SEGMENTS: &[&str] = &[
    "delete", "remove", "destroy", "erase", "del", "drop", "archive",
];

pub(super) const IGNORED_PATH_PREFIXES: &[&str] = &[
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

    let has_seg = |c: &[&str]| segments.iter().any(|seg| c.iter().any(|x| seg == x));
    let has_op_token = |c: &[&str]| {
        !op_lower.is_empty()
            && c.iter().any(|t| {
                op_lower == *t || op_lower.starts_with(&format!("{t}_"))
                    || op_lower.starts_with(t) || op_lower.ends_with(t)
            })
    };
    let ends_with_param = path.trim_end_matches('/').rsplit('/').next()
        .map(|s| s.starts_with('{') && s.ends_with('}')).unwrap_or(false);

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
