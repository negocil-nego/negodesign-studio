use crate::model::codegen::{FrontendFramework, OpenApiFrontendCodegenRequest};
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::to_kebab_case;
use std::collections::BTreeMap;

pub(super) fn insert_admin_index(
    files: &mut BTreeMap<String, String>,
    req: &OpenApiFrontendCodegenRequest,
    active_entities: &[OpenApiEntity],
) {
    if let Some(first) = active_entities.first() {
        let first_kebab = to_kebab_case(&first.name);
        match req.frontend_framework {
            FrontendFramework::Sveltekit => {
                files.insert(
                    "src/routes/(admin)/admin/+page.ts".to_string(),
                    format!(
                        "import {{ redirect }} from \"@sveltejs/kit\";\n\n\
export const load = () => {{\n\
  throw redirect(307, \"/admin/{first_kebab}\");\n\
}};\n"
                    ),
                );
            }
            FrontendFramework::Nuxt => {
                files.insert(
                    "pages/admin/index.vue".to_string(),
                    format!(
                        "<template>\n  <div />\n</template>\n\n\
<script setup lang=\"ts\">\n\
await navigateTo(\"/admin/{first_kebab}\");\n\
</script>\n"
                    ),
                );
            }
            FrontendFramework::Nextjs => {
                files.insert(
                    "app/admin/page.tsx".to_string(),
                    format!(
                        "import {{ redirect }} from \"next/navigation\";\n\n\
export default function AdminIndexPage() {{\n\
  redirect(\"/admin/{first_kebab}\");\n\
}}\n"
                    ),
                );
            }
        }
    }
}
