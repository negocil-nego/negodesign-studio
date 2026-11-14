use crate::{
    model::{codegen::FrontendFramework, migration::DatabaseTable},
    repository::codegen::naming::{to_kebab_case, to_pascal_case},
};

/// `api_import` é o specifier do ficheiro `.api` da mesma tabela — depende da
/// estrutura de pastas (SvelteKit: `../api/<kebab>.api`; Nuxt/Next: `./<kebab>.api`).
pub fn build_tanstack_query_ts(
    framework: &FrontendFramework,
    table: &DatabaseTable,
    api_import: &str,
) -> String {
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
        r#"import {{ {query_fn}, {mutation_fn}, useQueryClient }} from "{pkg}";
import {{ {pascal}Api }} from "{api_import}";

export const {pascal}QueryKey = ["{kebab}"] as const;

export function use{pascal}ListQuery(page = 0, size = 10) {{
  return {query_fn}({{
    queryKey: [...{pascal}QueryKey, page, size],
    queryFn: () => {pascal}Api.findAll(page, size),
  }});
}}

export function useInsert{pascal}Mutation() {{
  const queryClient = useQueryClient();
  return {mutation_fn}({{
    mutationFn: (payload: Record<string, any>) => {pascal}Api.insert(payload),
    onSuccess: () => queryClient.invalidateQueries({{ queryKey: {pascal}QueryKey }}),
  }});
}}

export function useUpdate{pascal}Mutation() {{
  const queryClient = useQueryClient();
  return {mutation_fn}({{
    mutationFn: ({{ id, payload }}: {{ id: string | number; payload: Record<string, any> }}) =>
      {pascal}Api.update(id, payload),
    onSuccess: () => queryClient.invalidateQueries({{ queryKey: {pascal}QueryKey }}),
  }});
}}

export function useDelete{pascal}Mutation() {{
  const queryClient = useQueryClient();
  return {mutation_fn}({{
    mutationFn: (id: string | number) => {pascal}Api.delete(id),
    onSuccess: () => queryClient.invalidateQueries({{ queryKey: {pascal}QueryKey }}),
  }});
}}
"#
    )
}
