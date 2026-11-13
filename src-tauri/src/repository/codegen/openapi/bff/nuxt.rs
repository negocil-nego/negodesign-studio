use super::BffRouteCtx;
use std::collections::BTreeMap;

pub(super) fn generate_nuxt_bff_routes(ctx: &BffRouteCtx) -> BTreeMap<String, String> {
    let BffRouteCtx {
        kebab,
        list_url,
        create_url,
        get_id_generic,
        update_id_generic,
        delete_id_generic,
        update_method,
        delete_method,
        ..
    } = ctx;
    let mut files = BTreeMap::new();
    files.insert(
        format!("server/api/{kebab}/index.ts"),
        format!(
            "import {{ defineEventHandler, getQuery, readBody }} from \"h3\";\n\n\
const LIST_URL = \"{list_url}\";\n\
const CREATE_URL = \"{create_url}\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  const method = event.node.req.method;\n\
  if (method === \"GET\") {{\n\
    const query = getQuery(event);\n\
    return $fetch(LIST_URL, {{ query }});\n\
  }}\n\
  if (method === \"POST\") {{\n\
    const body = await readBody(event);\n\
    return $fetch(CREATE_URL, {{ method: \"POST\", body }});\n\
  }}\n\
}});\n"
        ),
    );
    files.insert(
        format!("server/api/{kebab}/[id].ts"),
        format!(
            "import {{ defineEventHandler, getRouterParam, readBody }} from \"h3\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  const id = getRouterParam(event, \"id\");\n\
  const method = event.node.req.method;\n\
  if (method === \"GET\") {{\n\
    return $fetch(`{get_id_generic}`);\n\
  }}\n\
  if (method === \"PUT\" || method === \"PATCH\") {{\n\
    const body = await readBody(event);\n\
    return $fetch(`{update_id_generic}`, {{ method: \"{update_method}\", body }});\n\
  }}\n\
  if (method === \"DELETE\") {{\n\
    return $fetch(`{delete_id_generic}`, {{ method: \"{delete_method}\" }});\n\
  }}\n\
}});\n"
        ),
    );
    files
}
