/// `folder` é a pasta da feature dentro de `src/lib/features/` (agrupamento
/// por domínio: `agent`, `messages`, …).
pub(super) fn build_svelte_admin_page(kebab: &str, pascal: &str, folder: &str) -> String {
    format!(
        r#"<script lang="ts">
  import {{ onMount }} from "svelte";
  import AdminCrud from "../../../negodesign/project/ui/pages/admin/ui/AdminCrud.svelte";
  import type {{ AdminProps }} from "../../../negodesign/project/ui/pages/admin/data/types";
  import {{ {pascal}Api }} from "$lib/features/{folder}/api/{kebab}.api";
  import {{ create{pascal}AdminProps }} from "$lib/features/{folder}/data/{kebab}.admin";

  let records = $state<Record<string, any>[]>([]);
  let editingItem = $state<Record<string, any> | null>(null);

  async function loadRecords() {{
    try {{
      const res = await {pascal}Api.findAll(0, 10);
      records = Array.isArray(res?.data) ? res.data : (Array.isArray(res) ? res : []);
    }} catch (err) {{
      console.error("Failed to load {kebab}:", err);
    }}
  }}

  let adminProps: AdminProps = $derived(
    create{pascal}AdminProps(records, {{
      onRowAction: async (action, rowIndex) => {{
        const target = records[rowIndex];
        if (!target) return;
        if (action.toLowerCase() === "edit") {{
          editingItem = target;
        }} else if (action.toLowerCase() === "delete") {{
          await {pascal}Api.delete(target.id);
          await loadRecords();
        }}
      }},
      onSubmit: async (values) => {{
        if (!values) return;
        if (editingItem?.id != null) {{
          await {pascal}Api.update(editingItem.id, values as Record<string, any>);
          editingItem = null;
        }} else {{
          await {pascal}Api.insert(values as Record<string, any>);
        }}
        await loadRecords();
      }},
    }})
  );
  let {{ datatableProps, sidebarProps, modalFormProps, menuProps }}: AdminProps = $derived(adminProps);

  onMount(loadRecords);
</script>

<AdminCrud {{datatableProps}} {{sidebarProps}} {{modalFormProps}} {{menuProps}} />
"#
    )
}
