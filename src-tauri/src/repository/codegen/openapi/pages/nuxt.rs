use crate::model::codegen::OpenApiFrontendCodegenRequest;
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::{to_camel_case, to_kebab_case, to_pascal_case};
use crate::repository::codegen::openapi::dtos::strip_entity_action_or_kind;

use super::nuxt_direct::generate_nuxt_direct_page;

pub fn generate_nuxt_page(
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
            "<template>\n\
  <AdminCrud v-bind=\"adminProps\" />\n\
</template>\n\n\
<script setup lang=\"ts\">\n\
import {{ ref, computed }} from \"vue\";\n\
import AdminCrud from \"../../negodesign/project/ui/pages/admin/ui/AdminCrud.vue\";\n\
import {{ create{pascal}AdminProps }} from \"../../lib/admin/{kebab}.config\";\n\
import {{\n\
  use{pascal}ListQuery,\n\
  useCreate{base_pascal}Mutation,\n\
  useUpdate{base_pascal}Mutation,\n\
  useDelete{base_pascal}Mutation,\n\
}} from \"../../lib/queries/{kebab}.queries\";\n\n\
const {{ data: items }} = use{pascal}ListQuery();\n\
const createMutation = useCreate{base_pascal}Mutation();\n\
const updateMutation = useUpdate{base_pascal}Mutation();\n\
const deleteMutation = useDelete{base_pascal}Mutation();\n\n\
const editingItem = ref<Record<string, any> | null>(null);\n\n\
const adminProps = computed(() => {{\n\
  const records = items.value ?? [];\n\
  return create{pascal}AdminProps(records, {{\n\
    onRowAction: async (action, rowIndex) => {{\n\
      const target = records[rowIndex];\n\
      if (!target) return;\n\
      if (action.toLowerCase() === \"edit\") {{\n\
        editingItem.value = target;\n\
      }} else if (action.toLowerCase() === \"delete\") {{\n\
        await deleteMutation.mutateAsync(target.id);\n\
      }}\n\
    }},\n\
    onSubmit: async (values) => {{\n\
      if (!values) return;\n\
      if (editingItem.value?.id != null) {{\n\
        await updateMutation.mutateAsync({{ id: editingItem.value.id, data: values as any }});\n\
        editingItem.value = null;\n\
      }} else {{\n\
        await createMutation.mutateAsync(values as any);\n\
      }}\n\
    }},\n\
  }});\n\
}});\n\
</script>\n"
        )
    } else {
        generate_nuxt_direct_page(&pascal, &camel, &kebab, entity_name)
    }
}
