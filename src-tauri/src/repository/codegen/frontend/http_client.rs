use crate::{
    model::{
        codegen::{
            ApiCallPattern, ArchitectureMode, FrontendFramework, FullstackCodegenRequest,
            HttpClientKind,
        },
        migration::DatabaseTable,
    },
    repository::codegen::naming::{to_kebab_case, to_pascal_case},
};

pub fn build_api_client_ts(req: &FullstackCodegenRequest, table: &DatabaseTable) -> String {
    let pascal = to_pascal_case(&table.name);
    let kebab = to_kebab_case(&table.name);

    let endpoint = if req.architecture_mode == ArchitectureMode::Monolith
        || req.api_call_pattern == ApiCallPattern::Bff
    {
        format!("/api/{kebab}")
    } else {
        let base = req
            .backend_base_url
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("http://localhost:8080");
        format!("{}/api/{kebab}", base.trim_end_matches('/'))
    };

    if req.http_client == HttpClientKind::Axios {
        return format!(
            "import axios from \"axios\";\n\n\
const api = axios.create({{ baseURL: \"{endpoint}\" }});\n\n\
export const {pascal}Api = {{\n\
  async findAll(page = 0, size = 10) {{\n\
    const {{ data }} = await api.get(\"\", {{ params: {{ page, size }} }});\n\
    return data;\n\
  }},\n\
  async insert(payload: Record<string, any>) {{\n\
    const {{ data }} = await api.post(\"\", payload);\n\
    return data;\n\
  }},\n\
  async update(id: string | number, payload: Record<string, any>) {{\n\
    const {{ data }} = await api.put(`/${{id}}`, payload);\n\
    return data;\n\
  }},\n\
  async delete(id: string | number) {{\n\
    await api.delete(`/${{id}}`);\n\
  }},\n\
}};\n"
        );
    }

    if req.frontend_framework == FrontendFramework::Nuxt {
        return format!(
            "export const {pascal}Api = {{\n\
  findAll(page = 0, size = 10) {{\n\
    return useFetch(\"{endpoint}\", {{ query: {{ page, size }} }});\n\
  }},\n\
  async insert(payload: Record<string, any>) {{\n\
    return await $fetch(\"{endpoint}\", {{ method: \"POST\", body: payload }});\n\
  }},\n\
  async update(id: string | number, payload: Record<string, any>) {{\n\
    return await $fetch(`{endpoint}/${{id}}`, {{ method: \"PUT\", body: payload }});\n\
  }},\n\
  async delete(id: string | number) {{\n\
    return await $fetch(`{endpoint}/${{id}}`, {{ method: \"DELETE\" }});\n\
  }},\n\
}};\n"
        );
    }

    format!(
        "export const {pascal}Api = {{\n\
  async findAll(page = 0, size = 10) {{\n\
    const res = await fetch(`{endpoint}?page=${{page}}&size=${{size}}`);\n\
    if (!res.ok) throw new Error(`Failed to fetch {kebab}: ${{res.status}}`);\n\
    return res.json();\n\
  }},\n\
  async insert(payload: Record<string, any>) {{\n\
    const res = await fetch(\"{endpoint}\", {{\n\
      method: \"POST\",\n\
      headers: {{ \"Content-Type\": \"application/json\" }},\n\
      body: JSON.stringify(payload),\n\
    }});\n\
    if (!res.ok) throw new Error(`Failed to insert {kebab}: ${{res.status}}`);\n\
    return res.json();\n\
  }},\n\
  async update(id: string | number, payload: Record<string, any>) {{\n\
    const res = await fetch(`{endpoint}/${{id}}`, {{\n\
      method: \"PUT\",\n\
      headers: {{ \"Content-Type\": \"application/json\" }},\n\
      body: JSON.stringify(payload),\n\
    }});\n\
    if (!res.ok) throw new Error(`Failed to update {kebab}: ${{res.status}}`);\n\
    return res.json();\n\
  }},\n\
  async delete(id: string | number) {{\n\
    const res = await fetch(`{endpoint}/${{id}}`, {{ method: \"DELETE\" }});\n\
    if (!res.ok) throw new Error(`Failed to delete {kebab}: ${{res.status}}`);\n\
  }},\n\
}};\n"
    )
}

