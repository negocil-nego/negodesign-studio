pub(super) fn build_next_admin_page(kebab: &str, pascal: &str) -> String {
    format!(
        "\"use client\";\n\n\
import {{ useEffect, useState }} from \"react\";\n\
import AdminCrud from \"../../../../negodesign/project/ui/pages/admin/ui/AdminCrud\";\n\
import {{ {pascal}Api }} from \"../../../features/{kebab}/{kebab}.api\";\n\
import {{ create{pascal}AdminProps }} from \"../../../features/{kebab}/{kebab}.admin\";\n\n\
export default function {pascal}AdminPage() {{\n\
  const [records, setRecords] = useState<Record<string, any>[]>([]);\n\n\
  useEffect(() => {{\n\
    {pascal}Api.findAll(0, 10)\n\
      .then((res) => setRecords(Array.isArray(res?.data) ? res.data : (Array.isArray(res) ? res : [])))\n\
      .catch((err) => console.error(\"Failed to load {kebab}:\", err));\n\
  }}, []);\n\n\
  const adminProps = create{pascal}AdminProps(records);\n\
  return <AdminCrud {{...adminProps}} />;\n\
}}\n"
    )
}
