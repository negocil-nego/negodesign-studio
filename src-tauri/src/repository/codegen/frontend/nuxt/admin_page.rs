pub(super) fn build_nuxt_admin_page(kebab: &str, pascal: &str) -> String {
    format!(
        "<script setup lang=\"ts\">\n\
import {{ computed, ref, onMounted }} from \"vue\";\n\
import AdminCrud from \"../../negodesign/project/ui/pages/admin/ui/AdminCrud.vue\";\n\
import {{ {pascal}Api }} from \"../../composables/features/{kebab}/{kebab}.api\";\n\
import {{ create{pascal}AdminProps }} from \"../../composables/features/{kebab}/{kebab}.admin\";\n\n\
const records = ref<Record<string, any>[]>([]);\n\
const adminProps = computed(() => create{pascal}AdminProps(records.value));\n\n\
onMounted(async () => {{\n\
  try {{\n\
    const res: any = await {pascal}Api.findAll(0, 10);\n\
    const payload = res?.data?.value ?? res?.data ?? res;\n\
    records.value = Array.isArray(payload?.data) ? payload.data : (Array.isArray(payload) ? payload : []);\n\
  }} catch (err) {{\n\
    console.error(\"Failed to load {kebab}:\", err);\n\
  }}\n\
}});\n\
</script>\n\n\
<template>\n\
  <AdminCrud v-bind=\"adminProps\" />\n\
</template>\n"
    )
}
