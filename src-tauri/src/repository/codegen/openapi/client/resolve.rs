use crate::model::codegen::OpenApiFrontendCodegenRequest;
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::to_kebab_case;
use crate::repository::codegen::openapi::dtos::strip_entity_action_or_kind;
use crate::repository::openapi::crud_matcher::classify_operation;

#[derive(Debug, Clone)]
pub struct ResolvedCrudEndpoints {
    pub list_path: String,
    pub get_path: String,
    pub create_path: String,
    pub update_method: String,
    pub update_path: String,
    pub delete_method: String,
    pub delete_path: String,
}

pub fn resolve_entity_endpoints(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> ResolvedCrudEndpoints {
    let base_name = strip_entity_action_or_kind(&entity.name);
    let kebab = to_kebab_case(base_name);

    let mut list = ("GET".to_string(), format!("/api/{kebab}"));
    let mut get = ("GET".to_string(), format!("/api/{kebab}/{{id}}"));
    let mut create = ("POST".to_string(), format!("/api/{kebab}"));
    let mut update = ("PUT".to_string(), format!("/api/{kebab}/{{id}}"));
    let mut delete = ("DELETE".to_string(), format!("/api/{kebab}/{{id}}"));

    let mut apply = |method: &str, path: &str, role_opt: Option<&str>, op_id: Option<&str>| {
        let role = role_opt
            .map(str::to_string)
            .unwrap_or_else(|| classify_operation(method, path, op_id).0);
        let m = method.to_uppercase();
        let p = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{path}")
        };
        match role.as_str() {
            "list" => list = (m, p),
            "get" => get = (m, p),
            "create" => create = (m, p),
            "update" => update = (m, p),
            "delete" => delete = (m, p),
            _ => {}
        }
    };

    let selected: Vec<_> = req
        .selected_endpoints
        .iter()
        .filter(|s| s.entity_name == entity.name)
        .collect();

    if !selected.is_empty() {
        for sel in selected {
            apply(
                &sel.method,
                &sel.path,
                sel.crud_role.as_deref(),
                sel.operation_id.as_deref(),
            );
        }
    } else {
        for op in &entity.operations {
            apply(
                &op.method,
                &op.path,
                op.crud_role.as_deref(),
                op.operation_id.as_deref(),
            );
        }
    }

    ResolvedCrudEndpoints {
        list_path: list.1,
        get_path: get.1,
        create_path: create.1,
        update_method: update.0,
        update_path: update.1,
        delete_method: delete.0,
        delete_path: delete.1,
    }
}
