pub(super) fn build_svelte_admin_page(kebab: &str, pascal: &str) -> String {
    format!(
        "<script lang=\"ts\">\n\
  import {{ onMount }} from \"svelte\";\n\
  import AdminCrud from \"../../../../negodesign/project/ui/pages/admin/ui/AdminCrud.svelte\";\n\
  import type {{ AdminProps }} from \"../../../../negodesign/project/ui/pages/admin/data/types\";\n\
  import {{ {pascal}Api }} from \"$lib/features/{kebab}/{kebab}.api\";\n\
  import {{ create{pascal}AdminProps }} from \"$lib/features/{kebab}/{kebab}.admin\";\n\n\
  let records = $state<Record<string, any>[]>([]);\n\
  let editingItem = $state<Record<string, any> | null>(null);\n\n\
  async function loadRecords() {{\n\
    try {{\n\
      const res = await {pascal}Api.findAll(0, 10);\n\
      records = Array.isArray(res?.data) ? res.data : (Array.isArray(res) ? res : []);\n\
    }} catch (err) {{\n\
      console.error(\"Failed to load {kebab}:\", err);\n\
    }}\n\
  }}\n\n\
  let adminProps: AdminProps = $derived(\n\
    create{pascal}AdminProps(records, {{\n\
      onRowAction: async (action, rowIndex) => {{\n\
        const target = records[rowIndex];\n\
        if (!target) return;\n\
        if (action.toLowerCase() === \"edit\") {{\n\
          editingItem = target;\n\
        }} else if (action.toLowerCase() === \"delete\") {{\n\
          await {pascal}Api.delete(target.id);\n\
          await loadRecords();\n\
        }}\n\
      }},\n\
      onSubmit: async (values) => {{\n\
        if (!values) return;\n\
        if (editingItem?.id != null) {{\n\
          await {pascal}Api.update(editingItem.id, values as Record<string, any>);\n\
          editingItem = null;\n\
        }} else {{\n\
          await {pascal}Api.insert(values as Record<string, any>);\n\
        }}\n\
        await loadRecords();\n\
      }},\n\
    }})\n\
  );\n\
  let {{ datatableProps, sidebarProps, modalFormProps, menuProps }}: AdminProps = $derived(adminProps);\n\n\
  onMount(loadRecords);\n\
</script>\n\n\
<AdminCrud {{datatableProps}} {{sidebarProps}} {{modalFormProps}} {{menuProps}} />\n"
    )
}
