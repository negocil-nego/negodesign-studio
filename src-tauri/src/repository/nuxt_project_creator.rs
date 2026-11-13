use crate::model::codegen::{FrontendProjectRequest, FrontendProjectResult, HttpClientKind};
use std::{fs, path::PathBuf};

pub fn create(req: &FrontendProjectRequest) -> Result<FrontendProjectResult, String> {
    let project_dir = PathBuf::from(&req.target_dir).join(&req.name);
    if project_dir.exists() && project_dir.read_dir().map_or(false, |mut d| d.next().is_some()) {
        return Err(format!(
            "O directório '{}' já existe e não está vazio.",
            project_dir.display()
        ));
    }

    fs::create_dir_all(project_dir.join("pages")).map_err(|e| e.to_string())?;
    fs::create_dir_all(project_dir.join("assets/css")).map_err(|e| e.to_string())?;

    let mut dependencies: Vec<String> = vec![
        "nuxt@^3.15.0".to_string(),
        "vue@^3.5.0".to_string(),
        "vue-router@^4.5.0".to_string(),
    ];
    let mut dev_dependencies: Vec<String> = vec![
        "typescript@^5.7.0".to_string(),
    ];

    if req.include_tailwind {
        dev_dependencies.push("@nuxtjs/tailwindcss@^6.13.0".to_string());
    }

    if matches!(req.http_client, HttpClientKind::Axios) {
        dependencies.push("axios@^1.8.0".to_string());
    }

    if req.use_tanstack_query {
        dependencies.push("@tanstack/vue-query@^5.66.0".to_string());
    }

    // 1. package.json
    let package_json = format!(
        r#"{{
  "name": "{name}",
  "version": "0.0.1",
  "description": "{description}",
  "private": true,
  "type": "module",
  "scripts": {{
    "build": "nuxt build",
    "dev": "nuxt dev",
    "generate": "nuxt generate",
    "preview": "nuxt preview",
    "postinstall": "nuxt prepare"
  }},
  "dependencies": {{
{deps}
  }},
  "devDependencies": {{
{dev_deps}
  }}
}}
"#,
        name = req.name,
        description = req.description,
        deps = dependencies
            .iter()
            .map(|d| {
                let parts: Vec<&str> = d.split('@').collect();
                format!("    \"{}\": \"{}\"", parts[0], parts[1])
            })
            .collect::<Vec<_>>()
            .join(",\n"),
        dev_deps = dev_dependencies
            .iter()
            .map(|d| {
                let parts: Vec<&str> = d.split('@').collect();
                format!("    \"{}\": \"{}\"", parts[0], parts[1])
            })
            .collect::<Vec<_>>()
            .join(",\n")
    );
    fs::write(project_dir.join("package.json"), package_json).map_err(|e| e.to_string())?;

    // 2. nuxt.config.ts
    let nuxt_config = if req.include_tailwind {
        r#"// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-01-01',
  devtools: { enabled: true },
  modules: ['@nuxtjs/tailwindcss'],
  css: ['~/assets/css/main.css']
})
"#
    } else {
        r#"// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-01-01',
  devtools: { enabled: true }
})
"#
    };
    fs::write(project_dir.join("nuxt.config.ts"), nuxt_config).map_err(|e| e.to_string())?;

    // 3. tsconfig.json
    let tsconfig = r#"{
  "extends": "./.nuxt/tsconfig.json"
}
"#;
    fs::write(project_dir.join("tsconfig.json"), tsconfig).map_err(|e| e.to_string())?;

    // 4. assets/css/main.css
    let main_css = if req.include_tailwind {
        "@tailwind base;\n@tailwind components;\n@tailwind utilities;\n"
    } else {
        "/* Estilos globais */\n"
    };
    fs::write(project_dir.join("assets/css/main.css"), main_css).map_err(|e| e.to_string())?;

    // 5. app.vue
    let app_vue = r#"<template>
  <div class="min-h-screen bg-zinc-950 text-zinc-100">
    <NuxtRouteAnnouncer />
    <NuxtPage />
  </div>
</template>
"#;
    fs::write(project_dir.join("app.vue"), app_vue).map_err(|e| e.to_string())?;

    // 6. pages/index.vue
    let index_vue = format!(
        r#"<template>
  <div class="flex min-h-screen flex-col items-center justify-center p-6 text-center">
    <div class="max-w-md rounded-2xl border border-zinc-800 bg-zinc-900/80 p-8 shadow-2xl">
      <div class="mb-4 inline-flex size-14 items-center justify-center rounded-2xl bg-emerald-500/10 text-emerald-400">
        <svg class="size-8" viewBox="0 0 24 24" fill="currentColor">
          <path d="M11 2L2 19h7l2-4 2 4h7L11 2z"/>
        </svg>
      </div>
      <h1 class="text-2xl font-bold tracking-tight text-white">{name}</h1>
      <p class="mt-2 text-sm text-zinc-400">Projecto Nuxt pronto para receber as páginas e componentes AdminCrud do NegoDesign Studio.</p>
      <div class="mt-6 flex justify-center gap-3">
        <NuxtLink to="/admin" class="rounded-lg bg-emerald-600 px-4 py-2 text-xs font-semibold text-white transition hover:bg-emerald-500">Aceder ao Painel</NuxtLink>
      </div>
    </div>
  </div>
</template>
"#,
        name = req.name
    );
    fs::write(project_dir.join("pages/index.vue"), index_vue).map_err(|e| e.to_string())?;

    // 7. .gitignore
    let gitignore = r#".nuxt
.output
node_modules
*.local
.env
.env.*
!.env.example
"#;
    fs::write(project_dir.join(".gitignore"), gitignore).map_err(|e| e.to_string())?;

    // 8. README.md
    let readme = format!(
        r#"# {name}

Projecto Nuxt gerado automaticamente pelo **NegoDesign Studio**.

## Execução

```bash
pnpm install
pnpm dev
```
"#,
        name = req.name
    );
    fs::write(project_dir.join("README.md"), readme).map_err(|e| e.to_string())?;

    let mut all_deps = dependencies;
    all_deps.extend(dev_dependencies);

    Ok(FrontendProjectResult {
        project_dir: project_dir.to_string_lossy().to_string(),
        framework: "Nuxt".to_string(),
        build_tool: "pnpm / npm / nuxt".to_string(),
        generated_files: 8,
        dependencies: all_deps,
        next_steps: vec![
            format!("cd {}", project_dir.display()),
            "pnpm install (ou npm install)".to_string(),
            "pnpm dev".to_string(),
        ],
    })
}

