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
            r#"import axios from "axios";

const api = axios.create({{ baseURL: "{endpoint}" }});

export const {pascal}Api = {{
  async findAll(page = 0, size = 10) {{
    const {{ data }} = await api.get("", {{ params: {{ page, size }} }});
    return data;
  }},
  async insert(payload: Record<string, any>) {{
    const {{ data }} = await api.post("", payload);
    return data;
  }},
  async update(id: string | number, payload: Record<string, any>) {{
    const {{ data }} = await api.put(`/${{id}}`, payload);
    return data;
  }},
  async delete(id: string | number) {{
    await api.delete(`/${{id}}`);
  }},
}};
"#
        );
    }

    if req.frontend_framework == FrontendFramework::Nuxt {
        return format!(
            r#"export const {pascal}Api = {{
  findAll(page = 0, size = 10) {{
    return useFetch("{endpoint}", {{ query: {{ page, size }} }});
  }},
  async insert(payload: Record<string, any>) {{
    return await $fetch("{endpoint}", {{ method: "POST", body: payload }});
  }},
  async update(id: string | number, payload: Record<string, any>) {{
    return await $fetch(`{endpoint}/${{id}}`, {{ method: "PUT", body: payload }});
  }},
  async delete(id: string | number) {{
    return await $fetch(`{endpoint}/${{id}}`, {{ method: "DELETE" }});
  }},
}};
"#
        );
    }

    format!(
        r#"export const {pascal}Api = {{
  async findAll(page = 0, size = 10) {{
    const res = await fetch(`{endpoint}?page=${{page}}&size=${{size}}`);
    if (!res.ok) throw new Error(`Failed to fetch {kebab}: ${{res.status}}`);
    return res.json();
  }},
  async insert(payload: Record<string, any>) {{
    const res = await fetch("{endpoint}", {{
      method: "POST",
      headers: {{ "Content-Type": "application/json" }},
      body: JSON.stringify(payload),
    }});
    if (!res.ok) throw new Error(`Failed to insert {kebab}: ${{res.status}}`);
    return res.json();
  }},
  async update(id: string | number, payload: Record<string, any>) {{
    const res = await fetch(`{endpoint}/${{id}}`, {{
      method: "PUT",
      headers: {{ "Content-Type": "application/json" }},
      body: JSON.stringify(payload),
    }});
    if (!res.ok) throw new Error(`Failed to update {kebab}: ${{res.status}}`);
    return res.json();
  }},
  async delete(id: string | number) {{
    const res = await fetch(`{endpoint}/${{id}}`, {{ method: "DELETE" }});
    if (!res.ok) throw new Error(`Failed to delete {kebab}: ${{res.status}}`);
  }},
}};
"#
    )
}
