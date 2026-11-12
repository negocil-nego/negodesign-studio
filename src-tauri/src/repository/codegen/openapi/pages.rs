use crate::model::codegen::{FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::{to_camel_case, to_kebab_case, to_pascal_case};
use crate::repository::codegen::openapi::dtos::strip_entity_action_or_kind;

pub fn generate_openapi_admin_page(
    req: &OpenApiFrontendCodegenRequest,
    entity: &OpenApiEntity,
) -> (String, String) {
    let kebab = to_kebab_case(&entity.name);
    match req.frontend_framework {
        FrontendFramework::Sveltekit => (
            format!("src/routes/(admin)/admin/{kebab}/+page.svelte"),
            generate_sveltekit_page(req, entity),
        ),
        FrontendFramework::Nuxt => (
            format!("pages/admin/{kebab}.vue"),
            generate_nuxt_page(req, entity),
        ),
        FrontendFramework::Nextjs => (
            format!("app/admin/{kebab}/page.tsx"),
            generate_nextjs_page(req, entity),
        ),
    }
}

fn generate_sveltekit_page(
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
        format!(
            "<script lang=\"ts\">\n\
  import {{ onMount }} from \"svelte\";\n\
  import AdminCrud from \"../../../../negodesign/project/ui/pages/admin/ui/AdminCrud.svelte\";\n\
  import type {{ AdminProps }} from \"../../../../negodesign/project/ui/pages/admin/data/types\";\n\
  import {{ create{pascal}AdminProps }} from \"$lib/admin/{kebab}.config\";\n\
  import {{ {camel}Api }} from \"$lib/api/{kebab}.client\";\n\n\
  let items = $state<Record<string, any>[]>([]);\n\
  let editingItem = $state<Record<string, any> | null>(null);\n\n\
  async function loadData() {{\n\
    try {{\n\
      items = await {camel}Api.getAll();\n\
    }} catch (e) {{\n\
      console.error(\"Failed to load {entity_name}:\", e);\n\
    }}\n\
  }}\n\n\
  onMount(loadData);\n\n\
  const adminProps: AdminProps = $derived(\n\
    create{pascal}AdminProps(items, {{\n\
      onRowAction: async (action, rowIndex) => {{\n\
        const target = items[rowIndex];\n\
        if (!target) return;\n\
        if (action.toLowerCase() === \"edit\") {{\n\
          editingItem = target;\n\
        }} else if (action.toLowerCase() === \"delete\") {{\n\
          await {camel}Api.delete(target.id);\n\
          await loadData();\n\
        }}\n\
      }},\n\
      onSubmit: async (values) => {{\n\
        if (!values) return;\n\
        if (editingItem?.id != null) {{\n\
          await {camel}Api.update(editingItem.id, values as any);\n\
          editingItem = null;\n\
        }} else {{\n\
          await {camel}Api.create(values as any);\n\
        }}\n\
        await loadData();\n\
      }},\n\
    }})\n\
  );\n\
  let {{ datatableProps, sidebarProps, modalFormProps, menuProps }}: AdminProps = $derived(adminProps);\n\
</script>\n\n\
<AdminCrud {{datatableProps}} {{sidebarProps}} {{modalFormProps}} {{menuProps}} />\n"
        )
    }
}

fn generate_nuxt_page(
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
        format!(
            "<template>\n\
  <AdminCrud v-bind=\"adminProps\" />\n\
</template>\n\n\
<script setup lang=\"ts\">\n\
import {{ ref, computed, onMounted }} from \"vue\";\n\
import AdminCrud from \"../../negodesign/project/ui/pages/admin/ui/AdminCrud.vue\";\n\
import {{ create{pascal}AdminProps }} from \"../../lib/admin/{kebab}.config\";\n\
import {{ {camel}Api }} from \"../../lib/api/{kebab}.client\";\n\n\
const items = ref<Record<string, any>[]>([]);\n\
const editingItem = ref<Record<string, any> | null>(null);\n\n\
async function loadData() {{\n\
  try {{\n\
    items.value = await {camel}Api.getAll();\n\
  }} catch (e) {{\n\
    console.error(\"Failed to load {entity_name}:\", e);\n\
  }}\n\
}}\n\n\
onMounted(() => {{\n\
  void loadData();\n\
}});\n\n\
const adminProps = computed(() =>\n\
  create{pascal}AdminProps(items.value, {{\n\
    onRowAction: async (action, rowIndex) => {{\n\
      const target = items.value[rowIndex];\n\
      if (!target) return;\n\
      if (action.toLowerCase() === \"edit\") {{\n\
        editingItem.value = target;\n\
      }} else if (action.toLowerCase() === \"delete\") {{\n\
        await {camel}Api.delete(target.id);\n\
        await loadData();\n\
      }}\n\
    }},\n\
    onSubmit: async (values) => {{\n\
      if (!values) return;\n\
      if (editingItem.value?.id != null) {{\n\
        await {camel}Api.update(editingItem.value.id, values as any);\n\
        editingItem.value = null;\n\
      }} else {{\n\
        await {camel}Api.create(values as any);\n\
      }}\n\
      await loadData();\n\
    }},\n\
  }})\n\
);\n\
</script>\n"
        )
    }
}

fn generate_nextjs_page(
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
            "\"use client\";\n\n\
import React, {{ useState, useMemo }} from \"react\";\n\
import AdminCrud from \"../../../negodesign/project/ui/pages/admin/ui/AdminCrud\";\n\
import {{ create{pascal}AdminProps }} from \"../../../lib/admin/{kebab}.config\";\n\
import {{\n\
  use{pascal}List,\n\
  useCreate{base_pascal},\n\
  useUpdate{base_pascal},\n\
  useDelete{base_pascal},\n\
}} from \"../../../lib/queries/{kebab}.queries\";\n\n\
export default function {pascal}AdminPage() {{\n\
  const {{ data: items = [] }} = use{pascal}List();\n\
  const createMutation = useCreate{base_pascal}();\n\
  const updateMutation = useUpdate{base_pascal}();\n\
  const deleteMutation = useDelete{base_pascal}();\n\
  const [editingItem, setEditingItem] = useState<Record<string, any> | null>(null);\n\n\
  const adminProps = useMemo(\n\
    () =>\n\
      create{pascal}AdminProps(items, {{\n\
        onRowAction: async (action, rowIndex) => {{\n\
          const target = items[rowIndex];\n\
          if (!target) return;\n\
          if (action.toLowerCase() === \"edit\") {{\n\
            setEditingItem(target);\n\
          }} else if (action.toLowerCase() === \"delete\") {{\n\
            await deleteMutation.mutateAsync(target.id);\n\
          }}\n\
        }},\n\
        onSubmit: async (values) => {{\n\
          if (!values) return;\n\
          if (editingItem?.id != null) {{\n\
            await updateMutation.mutateAsync({{ id: editingItem.id, data: values as any }});\n\
            setEditingItem(null);\n\
          }} else {{\n\
            await createMutation.mutateAsync(values as any);\n\
          }}\n\
        }},\n\
      }}),\n\
    [items, editingItem, createMutation, updateMutation, deleteMutation]\n\
  );\n\n\
  return <AdminCrud {{...adminProps}} />;\n\
}}\n"
        )
    } else {
        format!(
            "\"use client\";\n\n\
import React, {{ useState, useEffect, useMemo }} from \"react\";\n\
import AdminCrud from \"../../../negodesign/project/ui/pages/admin/ui/AdminCrud\";\n\
import {{ create{pascal}AdminProps }} from \"../../../lib/admin/{kebab}.config\";\n\
import {{ {camel}Api }} from \"../../../lib/api/{kebab}.client\";\n\n\
export default function {pascal}AdminPage() {{\n\
  const [items, setItems] = useState<Record<string, any>[]>([]);\n\
  const [editingItem, setEditingItem] = useState<Record<string, any> | null>(null);\n\n\
  const loadData = async () => {{\n\
    try {{\n\
      const res = await {camel}Api.getAll();\n\
      setItems(res);\n\
    }} catch (e) {{\n\
      console.error(\"Failed to load {entity_name}:\", e);\n\
    }}\n\
  }};\n\n\
  useEffect(() => {{\n\
    void loadData();\n\
  }}, []);\n\n\
  const adminProps = useMemo(\n\
    () =>\n\
      create{pascal}AdminProps(items, {{\n\
        onRowAction: async (action, rowIndex) => {{\n\
          const target = items[rowIndex];\n\
          if (!target) return;\n\
          if (action.toLowerCase() === \"edit\") {{\n\
            setEditingItem(target);\n\
          }} else if (action.toLowerCase() === \"delete\") {{\n\
            await {camel}Api.delete(target.id);\n\
            await loadData();\n\
          }}\n\
        }},\n\
        onSubmit: async (values) => {{\n\
          if (!values) return;\n\
          if (editingItem?.id != null) {{\n\
            await {camel}Api.update(editingItem.id, values as any);\n\
            setEditingItem(null);\n\
          }} else {{\n\
            await {camel}Api.create(values as any);\n\
          }}\n\
          await loadData();\n\
        }},\n\
      }}),\n\
    [items, editingItem]\n\
  );\n\n\
  return <AdminCrud {{...adminProps}} />;\n\
}}\n"
        )
    }
}
