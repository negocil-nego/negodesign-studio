use super::patterns::{
    CREATE_NAMED_SEGMENTS, DELETE_NAMED_SEGMENTS, GET_ONE_NAMED_SEGMENTS,
    IGNORED_PATH_PREFIXES, LIST_NAMED_SEGMENTS, UPDATE_NAMED_SEGMENTS,
};
use crate::model::migration::OpenApiOperation;

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
    if refs.iter().any(|r| canonical_entity_stem(r) == entity_stem) {
        return true;
    }
    extract_resource_stem_from_path(&op.path)
        .map(|stem| stem == entity_stem)
        .unwrap_or(false)
}

fn extract_resource_stem_from_path(path: &str) -> Option<String> {
    normalized_segments(path).into_iter().find_map(|seg| {
        if IGNORED_PATH_PREFIXES.contains(&seg.as_str())
            || [
                LIST_NAMED_SEGMENTS, GET_ONE_NAMED_SEGMENTS, CREATE_NAMED_SEGMENTS,
                UPDATE_NAMED_SEGMENTS, DELETE_NAMED_SEGMENTS,
            ]
            .iter()
            .any(|c| c.contains(&seg.as_str()))
        {
            return None;
        }
        let cleaned = seg.replace(['-', '_'], "");
        (!cleaned.is_empty()).then(|| singularize(&cleaned))
    })
}

pub(super) fn normalized_segments(path: &str) -> Vec<String> {
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
