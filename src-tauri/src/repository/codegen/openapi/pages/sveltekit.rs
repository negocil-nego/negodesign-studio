use crate::model::codegen::OpenApiFrontendCodegenRequest;
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::{to_camel_case, to_kebab_case, to_pascal_case};
use crate::repository::codegen::openapi::dtos::strip_entity_action_or_kind;

use super::sveltekit_direct::generate_sveltekit_direct_page;

pub fn generate_sveltekit_page(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> String {
    let pascal = to_pascal_case(&entity.name);
    let camel = to_camel_case(&entity.name);
    let kebab = to_kebab_case(&entity.name);

    let base_name = strip_entity_action_or_kind(&entity.name);
    let base_pascal = to_pascal_case(base_name);
    let entity_name = &entity.name;

    if req.use_tanstack_query {
        format!(
            "<script lang=\"ts\">\n\
  import AdminCrud from \"../../../../negodesign/project/ui/pages/admin/ui/AdminCrud.svelte\";\n\
  import type {{ AdminProps }} from \"../../../../negodesign/project/ui/pages/admin/data/types\";\n\
  import {{ create{pascal}AdminProps }} from \"$lib/admin/{kebab}.config\";\n\
  import {{\n\
    use{pascal}ListQuery,\n\
    useCreate{base_pascal}Mutation,\n\
    useUpdate{base_pascal}Mutation,\n\
    useDelete{base_pascal}Mutation,\n\
  }} from \"$lib/queries/{kebab}.queries\";\n\n\
  const listQuery = use{pascal}ListQuery();\n\
  const createMutation = useCreate{base_pascal}Mutation();\n\
  const updateMutation = useUpdate{base_pascal}Mutation();\n\
  const deleteMutation = useDelete{base_pascal}Mutation();\n\n\
  let editingItem = $state<Record<string, any> | null>(null);\n\n\
  const items = $derived(listQuery.data ?? []);\n\
  const adminProps: AdminProps = $derived(\n\
    create{pascal}AdminProps(items, {{\n\
      onRowAction: async (action, rowIndex) => {{\n\
        const target = items[rowIndex];\n\
        if (!target) return;\n\
        if (action.toLowerCase() === \"edit\") {{\n\
          editingItem = target;\n\
        }} else if (action.toLowerCase() === \"delete\") {{\n\
          await deleteMutation.mutateAsync(target.id);\n\
        }}\n\
      }},\n\
      onSubmit: async (values) => {{\n\
        if (!values) return;\n\
        if (editingItem?.id != null) {{\n\
          await updateMutation.mutateAsync({{ id: editingItem.id, data: values as any }});\n\
          editingItem = null;\n\
        }} else {{\n\
          await createMutation.mutateAsync(values as any);\n\
        }}\n\
      }},\n\
    }})\n\
  );\n\
  let {{ datatableProps, sidebarProps, modalFormProps, menuProps }}: AdminProps = $derived(adminProps);\n\
</script>\n\n\
<AdminCrud {{datatableProps}} {{sidebarProps}} {{modalFormProps}} {{menuProps}} />\n"
        )
    } else {
        generate_sveltekit_direct_page(&pascal, &camel, &kebab, entity_name)
    }
}
