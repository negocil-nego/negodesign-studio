use crate::{
    model::{codegen::FrontendFramework, migration::DatabaseTable},
    repository::codegen::naming::{to_kebab_case, to_pascal_case},
};

pub fn build_tanstack_query_ts(framework: &FrontendFramework, table: &DatabaseTable) -> String {
    let pascal = to_pascal_case(&table.name);
    let kebab = to_kebab_case(&table.name);

    let pkg = match framework {
        FrontendFramework::Sveltekit => "@tanstack/svelte-query",
        FrontendFramework::Nuxt => "@tanstack/vue-query",
        FrontendFramework::Nextjs => "@tanstack/react-query",
    };

    let (query_fn, mutation_fn) = match framework {
        FrontendFramework::Sveltekit => ("createQuery", "createMutation"),
        FrontendFramework::Nuxt | FrontendFramework::Nextjs => ("useQuery", "useMutation"),
    };

    format!(
        "import {{ {query_fn}, {mutation_fn}, useQueryClient }} from \"{pkg}\";\n\
import {{ {pascal}Api }} from \"./{kebab}.api\";\n\n\
export const {pascal}QueryKey = [\"{kebab}\"] as const;\n\n\
export function use{pascal}ListQuery(page = 0, size = 10) {{\n\
  return {query_fn}({{\n\
    queryKey: [...{pascal}QueryKey, page, size],\n\
    queryFn: () => {pascal}Api.findAll(page, size),\n\
  }});\n\
}}\n\n\
export function useInsert{pascal}Mutation() {{\n\
  const queryClient = useQueryClient();\n\
  return {mutation_fn}({{\n\
    mutationFn: (payload: Record<string, any>) => {pascal}Api.insert(payload),\n\
    onSuccess: () => queryClient.invalidateQueries({{ queryKey: {pascal}QueryKey }}),\n\
  }});\n\
}}\n\n\
export function useUpdate{pascal}Mutation() {{\n\
  const queryClient = useQueryClient();\n\
  return {mutation_fn}({{\n\
    mutationFn: ({{ id, payload }}: {{ id: string | number; payload: Record<string, any> }}) =>\n\
      {pascal}Api.update(id, payload),\n\
    onSuccess: () => queryClient.invalidateQueries({{ queryKey: {pascal}QueryKey }}),\n\
  }});\n\
}}\n\n\
export function useDelete{pascal}Mutation() {{\n\
  const queryClient = useQueryClient();\n\
  return {mutation_fn}({{\n\
    mutationFn: (id: string | number) => {pascal}Api.delete(id),\n\
    onSuccess: () => queryClient.invalidateQueries({{ queryKey: {pascal}QueryKey }}),\n\
  }});\n\
}}\n"
    )
}

