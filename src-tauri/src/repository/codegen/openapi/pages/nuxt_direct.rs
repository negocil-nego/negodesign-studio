pub(super) fn generate_nuxt_direct_page(
    pascal: &str,
    camel: &str,
    kebab: &str,
    entity_name: &str,
) -> String {
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
