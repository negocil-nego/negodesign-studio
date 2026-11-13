pub(super) fn build_nuxt_monolith_collection_route(kebab: &str, pascal: &str) -> String {
    format!(
        "import {{ defineEventHandler, getQuery, readBody }} from \"h3\";\n\
import {{ {pascal}ServerRepository }} from \"../../repositories/{kebab}.repository\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  if (event.method === \"POST\") {{\n\
    const body = await readBody(event);\n\
    return {pascal}ServerRepository.insert(body);\n\
  }}\n\
  const query = getQuery(event);\n\
  return {pascal}ServerRepository.findAll(Number(query.page ?? 0), Number(query.size ?? 10));\n\
}});\n"
    )
}

pub(super) fn build_nuxt_monolith_item_route(kebab: &str, pascal: &str) -> String {
    format!(
        "import {{ defineEventHandler, getRouterParam, readBody }} from \"h3\";\n\
import {{ {pascal}ServerRepository }} from \"../../repositories/{kebab}.repository\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  const id = getRouterParam(event, \"id\") ?? \"\";\n\
  if (event.method === \"PUT\") {{\n\
    const body = await readBody(event);\n\
    return {pascal}ServerRepository.update(id, body);\n\
  }}\n\
  if (event.method === \"DELETE\") {{\n\
    await {pascal}ServerRepository.delete(id);\n\
    return null;\n\
  }}\n\
}});\n"
    )
}

pub(super) fn build_nuxt_bff_route(base_url: &str, kebab: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!(
        "import {{ defineEventHandler, getQuery, readBody }} from \"h3\";\n\n\
const BACKEND_URL = \"{base}/api/{kebab}\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  if (event.method === \"POST\") {{\n\
    const body = await readBody(event);\n\
    return $fetch(BACKEND_URL, {{ method: \"POST\", body }});\n\
  }}\n\
  const query = getQuery(event);\n\
  return $fetch(BACKEND_URL, {{ query }});\n\
}});\n"
    )
}

pub(super) fn build_nuxt_bff_item_route(base_url: &str, kebab: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!(
        "import {{ defineEventHandler, getRouterParam, readBody }} from \"h3\";\n\n\
const BACKEND_URL = \"{base}/api/{kebab}\";\n\n\
export default defineEventHandler(async (event) => {{\n\
  const id = getRouterParam(event, \"id\") ?? \"\";\n\
  if (event.method === \"PUT\") {{\n\
    const body = await readBody(event);\n\
    return $fetch(`${{BACKEND_URL}}/${{id}}`, {{ method: \"PUT\", body }});\n\
  }}\n\
  if (event.method === \"DELETE\") {{\n\
    return $fetch(`${{BACKEND_URL}}/${{id}}`, {{ method: \"DELETE\" }});\n\
  }}\n\
}});\n"
    )
}
