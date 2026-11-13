#[allow(clippy::too_many_arguments)]
pub(super) fn generate_fetch_native_client(
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
