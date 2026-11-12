use crate::model::codegen::{FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::{to_camel_case, to_kebab_case, to_pascal_case};
use crate::repository::codegen::openapi::dtos::strip_entity_action_or_kind;

pub fn generate_tanstack_queries_ts(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> String {
    let pascal = to_pascal_case(&entity.name);
    let camel = to_camel_case(&entity.name);
    let kebab = to_kebab_case(&entity.name);

    let base_name = strip_entity_action_or_kind(&entity.name);
    let base_pascal = to_pascal_case(base_name);

    match req.frontend_framework {
        FrontendFramework::Sveltekit => format!(
            "import {{ createQuery, createMutation, useQueryClient }} from \"@tanstack/svelte-query\";\n\
import {{ {camel}Api }} from \"../api/{kebab}.client\";\n\
import type {{ {pascal}, Create{base_pascal}Dto, Update{base_pascal}Dto }} from \"../types/openapi.dto\";\n\n\
export const {camel}Keys = {{\n\
  all: [\"{kebab}\"] as const,\n\
  lists: () => [...{camel}Keys.all, \"list\"] as const,\n\
  list: (params?: Record<string, any>) => [...{camel}Keys.lists(), params] as const,\n\
  details: () => [...{camel}Keys.all, \"detail\"] as const,\n\
  detail: (id: string | number) => [...{camel}Keys.details(), id] as const,\n\
}};\n\n\
export function use{pascal}ListQuery(params?: () => Record<string, any> | undefined) {{\n\
  return createQuery(() => ({{\n\
    queryKey: {camel}Keys.list(params ? params() : undefined),\n\
    queryFn: () => {camel}Api.getAll(params ? params() : undefined),\n\
  }}));\n\
}}\n\n\
export function use{pascal}DetailQuery(id: () => string | number) {{\n\
  return createQuery(() => ({{\n\
    queryKey: {camel}Keys.detail(id()),\n\
    queryFn: () => {camel}Api.getById(id()),\n\
  }}));\n\
}}\n\n\
export function useCreate{base_pascal}Mutation() {{\n\
  const queryClient = useQueryClient();\n\
  return createMutation(() => ({{\n\
    mutationFn: (data: Create{base_pascal}Dto) => {camel}Api.create(data),\n\
    onSuccess: () => {{\n\
      queryClient.invalidateQueries({{ queryKey: {camel}Keys.lists() }});\n\
    }},\n\
  }}));\n\
}}\n\n\
export function useUpdate{base_pascal}Mutation() {{\n\
  const queryClient = useQueryClient();\n\
  return createMutation(() => ({{\n\
    mutationFn: ({{ id, data }}: {{ id: string | number; data: Update{base_pascal}Dto }}) => {camel}Api.update(id, data),\n\
    onSuccess: () => {{\n\
      queryClient.invalidateQueries({{ queryKey: {camel}Keys.all }});\n\
    }},\n\
  }}));\n\
}}\n\n\
export function useDelete{base_pascal}Mutation() {{\n\
  const queryClient = useQueryClient();\n\
  return createMutation(() => ({{\n\
    mutationFn: (id: string | number) => {camel}Api.delete(id),\n\
    onSuccess: () => {{\n\
      queryClient.invalidateQueries({{ queryKey: {camel}Keys.lists() }});\n\
    }},\n\
  }}));\n\
}}\n"
        ),
        FrontendFramework::Nuxt => format!(
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
        ),
        FrontendFramework::Nextjs => format!(
            "import {{ useQuery, useMutation, useQueryClient }} from \"@tanstack/react-query\";\n\
import {{ {camel}Api }} from \"../api/{kebab}.client\";\n\
import type {{ {pascal}, Create{base_pascal}Dto, Update{base_pascal}Dto }} from \"../types/openapi.dto\";\n\n\
export const {camel}Keys = {{\n\
  all: [\"{kebab}\"] as const,\n\
  lists: () => [...{camel}Keys.all, \"list\"] as const,\n\
  list: (params?: Record<string, any>) => [...{camel}Keys.lists(), params] as const,\n\
  details: () => [...{camel}Keys.all, \"detail\"] as const,\n\
  detail: (id: string | number) => [...{camel}Keys.details(), id] as const,\n\
}};\n\n\
export function use{pascal}List(params?: Record<string, any>) {{\n\
  return useQuery({{\n\
    queryKey: {camel}Keys.list(params),\n\
    queryFn: () => {camel}Api.getAll(params),\n\
  }});\n\
}}\n\n\
export function use{pascal}Detail(id: string | number) {{\n\
  return useQuery({{\n\
    queryKey: {camel}Keys.detail(id),\n\
    queryFn: () => {camel}Api.getById(id),\n\
    enabled: Boolean(id),\n\
  }});\n\
}}\n\n\
export function useCreate{base_pascal}() {{\n\
  const queryClient = useQueryClient();\n\
  return useMutation({{\n\
    mutationFn: (data: Create{base_pascal}Dto) => {camel}Api.create(data),\n\
    onSuccess: () => {{\n\
      queryClient.invalidateQueries({{ queryKey: {camel}Keys.lists() }});\n\
    }},\n\
  }});\n\
}}\n\n\
export function useUpdate{base_pascal}() {{\n\
  const queryClient = useQueryClient();\n\
  return useMutation({{\n\
    mutationFn: ({{ id, data }}: {{ id: string | number; data: Update{base_pascal}Dto }}) => {camel}Api.update(id, data),\n\
    onSuccess: () => {{\n\
      queryClient.invalidateQueries({{ queryKey: {camel}Keys.all }});\n\
    }},\n\
  }});\n\
}}\n\n\
export function useDelete{base_pascal}() {{\n\
  const queryClient = useQueryClient();\n\
  return useMutation({{\n\
    mutationFn: (id: string | number) => {camel}Api.delete(id),\n\
    onSuccess: () => {{\n\
      queryClient.invalidateQueries({{ queryKey: {camel}Keys.lists() }});\n\
    }},\n\
  }});\n\
}}\n"
        ),
    }
}
