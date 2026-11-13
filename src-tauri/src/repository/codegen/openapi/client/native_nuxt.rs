#[allow(clippy::too_many_arguments)]
pub(super) fn generate_nuxt_native_client(
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
