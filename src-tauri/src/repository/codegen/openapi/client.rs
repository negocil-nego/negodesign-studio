use crate::model::codegen::{
    ApiCallPattern, FrontendFramework, HttpClientKind, OpenApiFrontendCodegenRequest,
};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::{to_camel_case, to_kebab_case, to_pascal_case};
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

pub fn generate_client_ts(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> String {
    let pascal = to_pascal_case(&entity.name);
    let camel = to_camel_case(&entity.name);
    let kebab = to_kebab_case(&entity.name);

    let base_name = strip_entity_action_or_kind(&entity.name);
    let base_pascal = to_pascal_case(base_name);

    let resolved = resolve_entity_endpoints(req, entity);

    let (list_url, get_url_tpl, create_url, update_url_tpl, delete_url_tpl) =
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
        };

    let update_method = if req.api_call_pattern == ApiCallPattern::Bff {
        "PUT"
    } else {
        resolved.update_method.as_str()
    };
    let delete_method = if req.api_call_pattern == ApiCallPattern::Bff {
        "DELETE"
    } else {
        resolved.delete_method.as_str()
    };

    match req.http_client {
        HttpClientKind::Axios => generate_axios_client(
            &pascal,
            &base_pascal,
            &camel,
            &list_url,
            &get_url_tpl,
            &create_url,
            &update_url_tpl,
            update_method,
            &delete_url_tpl,
            delete_method,
        ),
        HttpClientKind::Native => match req.frontend_framework {
            FrontendFramework::Nuxt => generate_nuxt_native_client(
                &pascal,
                &base_pascal,
                &camel,
                &list_url,
                &get_url_tpl,
                &create_url,
                &update_url_tpl,
                update_method,
                &delete_url_tpl,
                delete_method,
            ),
            FrontendFramework::Sveltekit | FrontendFramework::Nextjs => generate_fetch_native_client(
                &pascal,
                &base_pascal,
                &camel,
                &list_url,
                &get_url_tpl,
                &create_url,
                &update_url_tpl,
                update_method,
                &delete_url_tpl,
                delete_method,
            ),
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_axios_client(
    pascal: &str,
    base_pascal: &str,
    camel: &str,
    list_url: &str,
    get_url_tpl: &str,
    create_url: &str,
    update_url_tpl: &str,
    update_method: &str,
    delete_url_tpl: &str,
    delete_method: &str,
) -> String {
    let axios_update = if update_method.eq_ignore_ascii_case("PATCH") {
        "patch"
    } else {
        "put"
    };
    let axios_delete = if delete_method.eq_ignore_ascii_case("POST") {
        "post"
    } else {
        "delete"
    };
    format!(
        "import axios from \"axios\";\n\
import type {{ {pascal}, Create{base_pascal}Dto, Update{base_pascal}Dto }} from \"../types/openapi.dto\";\n\n\
const apiClient = axios.create({{\n\
  headers: {{\n\
    \"Content-Type\": \"application/json\",\n\
    Accept: \"application/json\",\n\
  }},\n\
}});\n\n\
export const {camel}Api = {{\n\
  async getAll(params?: Record<string, any>): Promise<{pascal}[]> {{\n\
    const response = await apiClient.get<{pascal}[] | {{ data?: {pascal}[]; items?: {pascal}[] }}>(\"{list_url}\", {{ params }});\n\
    if (Array.isArray(response.data)) return response.data;\n\
    return (response.data as any)?.data ?? (response.data as any)?.items ?? [];\n\
  }},\n\n\
  async findAll(page = 0, size = 10): Promise<{{ data: {pascal}[]; total: number }}> {{\n\
    const items = await this.getAll({{ page, size }});\n\
    return {{ data: items, total: items.length }};\n\
  }},\n\n\
  async getById(id: string | number): Promise<{pascal}> {{\n\
    const response = await apiClient.get<{pascal}>(`{get_url_tpl}`);\n\
    return response.data;\n\
  }},\n\n\
  async create(data: Create{base_pascal}Dto): Promise<{pascal}> {{\n\
    const response = await apiClient.post<{pascal}>(\"{create_url}\", data);\n\
    return response.data;\n\
  }},\n\n\
  async update(id: string | number, data: Update{base_pascal}Dto): Promise<{pascal}> {{\n\
    const response = await apiClient.{axios_update}<{pascal}>(`{update_url_tpl}`, data);\n\
    return response.data;\n\
  }},\n\n\
  async delete(id: string | number): Promise<void> {{\n\
    await apiClient.{axios_delete}(`{delete_url_tpl}`);\n\
  }},\n\
}};\n"
    )
}

#[allow(clippy::too_many_arguments)]
fn generate_nuxt_native_client(
    pascal: &str,
    base_pascal: &str,
    camel: &str,
    list_url: &str,
    get_url_tpl: &str,
    create_url: &str,
    update_url_tpl: &str,
    update_method: &str,
    delete_url_tpl: &str,
    delete_method: &str,
) -> String {
    format!(
        "import type {{ {pascal}, Create{base_pascal}Dto, Update{base_pascal}Dto }} from \"../types/openapi.dto\";\n\n\
export const use{pascal}Api = () => {{\n\
  const useList = (params?: Record<string, any>) => {{\n\
    return useFetch<{pascal}[] | {{ data?: {pascal}[]; items?: {pascal}[] }}>(\"{list_url}\", {{\n\
      method: \"GET\",\n\
      query: params,\n\
    }});\n\
  }};\n\n\
  const getAll = async (params?: Record<string, any>): Promise<{pascal}[]> => {{\n\
    const res = await $fetch<{pascal}[] | {{ data?: {pascal}[]; items?: {pascal}[] }}>(\"{list_url}\", {{\n\
      method: \"GET\",\n\
      query: params,\n\
    }});\n\
    if (Array.isArray(res)) return res;\n\
    return (res as any)?.data ?? (res as any)?.items ?? [];\n\
  }};\n\n\
  const findAll = (page = 0, size = 10) => {{\n\
    return useFetch(\"{list_url}\", {{ query: {{ page, size }} }});\n\
  }};\n\n\
  const getById = async (id: string | number): Promise<{pascal}> => {{\n\
    return $fetch<{pascal}>(`{get_url_tpl}`, {{ method: \"GET\" }});\n\
  }};\n\n\
  const create = async (data: Create{base_pascal}Dto): Promise<{pascal}> => {{\n\
    return $fetch<{pascal}>(\"{create_url}\", {{\n\
      method: \"POST\",\n\
      body: data,\n\
    }});\n\
  }};\n\n\
  const update = async (id: string | number, data: Update{base_pascal}Dto): Promise<{pascal}> => {{\n\
    return $fetch<{pascal}>(`{update_url_tpl}`, {{\n\
      method: \"{update_method}\",\n\
      body: data,\n\
    }});\n\
  }};\n\n\
  const remove = async (id: string | number): Promise<void> => {{\n\
    await $fetch(`{delete_url_tpl}`, {{ method: \"{delete_method}\" }});\n\
  }};\n\n\
  return {{\n\
    useList,\n\
    getAll,\n\
    findAll,\n\
    getById,\n\
    create,\n\
    update,\n\
    delete: remove,\n\
  }};\n\
}};\n\n\
export const {camel}Api = use{pascal}Api();\n"
    )
}

#[allow(clippy::too_many_arguments)]
fn generate_fetch_native_client(
    pascal: &str,
    base_pascal: &str,
    camel: &str,
    list_url: &str,
    get_url_tpl: &str,
    create_url: &str,
    update_url_tpl: &str,
    update_method: &str,
    delete_url_tpl: &str,
    delete_method: &str,
) -> String {
    format!(
        "import type {{ {pascal}, Create{base_pascal}Dto, Update{base_pascal}Dto }} from \"../types/openapi.dto\";\n\n\
async function request<T>(path: string, options: RequestInit = {{}}): Promise<T> {{\n\
  const headers = new Headers(options.headers || {{}});\n\
  if (!headers.has(\"Content-Type\")) {{\n\
    headers.set(\"Content-Type\", \"application/json\");\n\
  }}\n\
  if (!headers.has(\"Accept\")) {{\n\
    headers.set(\"Accept\", \"application/json\");\n\
  }}\n\
  const response = await fetch(path, {{ ...options, headers }});\n\
  if (!response.ok) {{\n\
    const message = await response.text();\n\
    throw new Error(message || `Request failed with status ${{response.status}}`);\n\
  }}\n\
  if (response.status === 204) {{\n\
    return undefined as unknown as T;\n\
  }}\n\
  return response.json();\n\
}}\n\n\
export const {camel}Api = {{\n\
  async getAll(params?: Record<string, any>): Promise<{pascal}[]> {{\n\
    const query = params ? `?${{new URLSearchParams(params as any).toString()}}` : \"\";\n\
    const res = await request<{pascal}[] | {{ data?: {pascal}[]; items?: {pascal}[] }}>(`{list_url}${{query}}`, {{ method: \"GET\" }});\n\
    if (Array.isArray(res)) return res;\n\
    return (res as any)?.data ?? (res as any)?.items ?? [];\n\
  }},\n\n\
  async findAll(page = 0, size = 10): Promise<{{ data: {pascal}[]; total: number }}> {{\n\
    const items = await this.getAll({{ page: String(page), size: String(size) }});\n\
    return {{ data: items, total: items.length }};\n\
  }},\n\n\
  async getById(id: string | number): Promise<{pascal}> {{\n\
    return request<{pascal}>(`{get_url_tpl}`, {{ method: \"GET\" }});\n\
  }},\n\n\
  async create(data: Create{base_pascal}Dto): Promise<{pascal}> {{\n\
    return request<{pascal}>(\"{create_url}\", {{\n\
      method: \"POST\",\n\
      body: JSON.stringify(data),\n\
    }});\n\
  }},\n\n\
  async update(id: string | number, data: Update{base_pascal}Dto): Promise<{pascal}> {{\n\
    return request<{pascal}>(`{update_url_tpl}`, {{\n\
      method: \"{update_method}\",\n\
      body: JSON.stringify(data),\n\
    }});\n\
  }},\n\n\
  async delete(id: string | number): Promise<void> {{\n\
    await request<void>(`{delete_url_tpl}`, {{ method: \"{delete_method}\" }});\n\
  }},\n\
}};\n"
    )
}
