pub(super) fn generate_nextjs_direct_page(
    pascal: &str,
    camel: &str,
    kebab: &str,
    entity_name: &str,
) -> String {
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
