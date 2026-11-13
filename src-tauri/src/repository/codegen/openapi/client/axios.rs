#[allow(clippy::too_many_arguments)]
pub(super) fn generate_axios_client(
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
