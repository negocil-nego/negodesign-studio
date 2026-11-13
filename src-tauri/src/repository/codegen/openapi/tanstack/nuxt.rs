pub fn generate_nuxt_queries_ts(
    pascal: &str,
    camel: &str,
    kebab: &str,
    base_pascal: &str,
) -> String {
    format!(
            "import {{ useQuery, useMutation, useQueryClient }} from \"@tanstack/vue-query\";\n\
import {{ {camel}Api }} from \"../api/{kebab}.client\";\n\
import type {{ {pascal}, Create{base_pascal}Dto, Update{base_pascal}Dto }} from \"../types/openapi.dto\";\n\n\
export const {camel}Keys = {{\n\
  all: [\"{kebab}\"] as const,\n\
  lists: () => [...{camel}Keys.all, \"list\"] as const,\n\
  list: (params?: any) => [...{camel}Keys.lists(), params] as const,\n\
  details: () => [...{camel}Keys.all, \"detail\"] as const,\n\
  detail: (id: string | number) => [...{camel}Keys.details(), id] as const,\n\
}};\n\n\
export function use{pascal}ListQuery(params?: any) {{\n\
  return useQuery({{\n\
    queryKey: {camel}Keys.list(params),\n\
    queryFn: () => {camel}Api.getAll(params),\n\
  }});\n\
}}\n\n\
export function use{pascal}DetailQuery(id: string | number) {{\n\
  return useQuery({{\n\
    queryKey: {camel}Keys.detail(id),\n\
    queryFn: () => {camel}Api.getById(id),\n\
  }});\n\
}}\n\n\
export function useCreate{base_pascal}Mutation() {{\n\
  const queryClient = useQueryClient();\n\
  return useMutation({{\n\
    mutationFn: (data: Create{base_pascal}Dto) => {camel}Api.create(data),\n\
    onSuccess: () => {{\n\
      queryClient.invalidateQueries({{ queryKey: {camel}Keys.lists() }});\n\
    }},\n\
  }});\n\
}}\n\n\
export function useUpdate{base_pascal}Mutation() {{\n\
  const queryClient = useQueryClient();\n\
  return useMutation({{\n\
    mutationFn: ({{ id, data }}: {{ id: string | number; data: Update{base_pascal}Dto }}) => {camel}Api.update(id, data),\n\
    onSuccess: () => {{\n\
      queryClient.invalidateQueries({{ queryKey: {camel}Keys.all }});\n\
    }},\n\
  }});\n\
}}\n\n\
export function useDelete{base_pascal}Mutation() {{\n\
  const queryClient = useQueryClient();\n\
  return useMutation({{\n\
    mutationFn: (id: string | number) => {camel}Api.delete(id),\n\
    onSuccess: () => {{\n\
      queryClient.invalidateQueries({{ queryKey: {camel}Keys.lists() }});\n\
    }},\n\
  }});\n\
}}\n"
    )
}
