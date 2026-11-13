pub(super) fn generate_sveltekit_direct_page(
    pascal: &str,
    camel: &str,
    kebab: &str,
    entity_name: &str,
) -> String {
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
