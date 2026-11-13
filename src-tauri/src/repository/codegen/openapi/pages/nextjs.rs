use crate::model::codegen::OpenApiFrontendCodegenRequest;
use crate::model::migration::OpenApiEntity;
use crate::repository::codegen::naming::{to_camel_case, to_kebab_case, to_pascal_case};
use crate::repository::codegen::openapi::dtos::strip_entity_action_or_kind;

use super::nextjs_direct::generate_nextjs_direct_page;

pub fn generate_nextjs_page(
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
        generate_nextjs_direct_page(&pascal, &camel, &kebab, entity_name)
    }
}
