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

    fs::create_dir_all(project_dir.join("src/routes")).map_err(|e| e.to_string())?;

    let mut dependencies: Vec<String> = vec![
        "@sveltejs/kit@^2.18.0".to_string(),
        "svelte@^5.22.0".to_string(),
    ];
    let mut dev_dependencies: Vec<String> = vec![
        "@sveltejs/adapter-auto@^4.0.0".to_string(),
        "@sveltejs/vite-plugin-svelte@^5.0.0".to_string(),
        "vite@^6.2.0".to_string(),
        "typescript@^5.7.0".to_string(),
    ];

    if req.include_tailwind {
        dev_dependencies.push("@tailwindcss/vite@^4.0.0".to_string());
        dev_dependencies.push("tailwindcss@^4.0.0".to_string());
    }

    if matches!(req.http_client, HttpClientKind::Axios) {
        dependencies.push("axios@^1.8.0".to_string());
    }

    if req.use_tanstack_query {
        dependencies.push("@tanstack/svelte-query@^5.66.0".to_string());
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
    "dev": "vite dev",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json"
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

    // 2. svelte.config.js
    let svelte_config = r#"import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter()
  }
};

export default config;
"#;
    fs::write(project_dir.join("svelte.config.js"), svelte_config).map_err(|e| e.to_string())?;

    // 3. vite.config.ts
    let vite_config = if req.include_tailwind {
        r#"import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()]
});
"#
    } else {
        r#"import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()]
});
"#
    };
    fs::write(project_dir.join("vite.config.ts"), vite_config).map_err(|e| e.to_string())?;

    // 4. tsconfig.json
    let tsconfig = r#"{
  "extends": "./.svelte-kit/tsconfig.json",
  "compilerOptions": {
    "allowJs": true,
    "checkJs": true,
    "esModuleInterop": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "sourceMap": true,
    "strict": true,
    "moduleResolution": "bundler"
  }
}
"#;
    fs::write(project_dir.join("tsconfig.json"), tsconfig).map_err(|e| e.to_string())?;

    // 5. src/app.html
    let app_html = r#"<!doctype html>
<html lang="pt">
  <head>
    <meta charset="utf-8" />
    <link rel="icon" href="%sveltekit.assets%/favicon.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    %sveltekit.head%
  </head>
  <body data-sveltekit-preload-data="hover" class="min-h-screen bg-zinc-950 text-zinc-100">
    <div style="display: contents">%sveltekit.body%</div>
  </body>
</html>
"#;
    fs::write(project_dir.join("src/app.html"), app_html).map_err(|e| e.to_string())?;

    // 6. src/app.d.ts
    let app_d_ts = r#"declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export {};
"#;
    fs::write(project_dir.join("src/app.d.ts"), app_d_ts).map_err(|e| e.to_string())?;

    // 7. src/app.css
    let app_css = if req.include_tailwind {
        "@import \"tailwindcss\";\n"
    } else {
        "/* Estilos globais */\n"
    };
    fs::write(project_dir.join("src/app.css"), app_css).map_err(|e| e.to_string())?;

    // 8. src/routes/+layout.svelte
    let layout_svelte = if req.use_tanstack_query {
        r#"<script lang="ts">
  import '../app.css';
  import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query';

  let { children } = $props();
  const queryClient = new QueryClient();
</script>

<QueryClientProvider client={queryClient}>
  {@render children()}
</QueryClientProvider>
"#
    } else {
        r#"<script lang="ts">
  import '../app.css';
  let { children } = $props();
</script>

{@render children()}
"#
    };
    fs::write(project_dir.join("src/routes/+layout.svelte"), layout_svelte)
        .map_err(|e| e.to_string())?;

    // 9. src/routes/+page.svelte
    let page_svelte = format!(
        r#"<script lang="ts">
  let appName = "{name}";
</script>

<div class="flex min-h-screen flex-col items-center justify-center p-6 text-center">
  <div class="max-w-md rounded-2xl border border-zinc-800 bg-zinc-900/80 p-8 shadow-2xl">
    <div class="mb-4 inline-flex size-14 items-center justify-center rounded-2xl bg-orange-500/10 text-orange-400">
      <svg class="size-8" viewBox="0 0 24 24" fill="currentColor">
        <path d="M19.5 8.25l-7.5 4.5-7.5-4.5 7.5-4.5 7.5 4.5zm0 3.75l-7.5 4.5-7.5-4.5 7.5-4.5 7.5 4.5zm0 3.75l-7.5 4.5-7.5-4.5 7.5-4.5 7.5 4.5z"/>
      </svg>
    </div>
    <h1 class="text-2xl font-bold tracking-tight text-white">{{appName}}</h1>
    <p class="mt-2 text-sm text-zinc-400">Projecto SvelteKit pronto para receber os módulos AdminCrud gerados pelo NegoDesign Studio.</p>
    <div class="mt-6 flex justify-center gap-3">
      <a href="/admin" class="rounded-lg bg-orange-600 px-4 py-2 text-xs font-semibold text-white transition hover:bg-orange-500">Aceder ao Painel</a>
    </div>
  </div>
</div>
"#,
        name = req.name
    );
    fs::write(project_dir.join("src/routes/+page.svelte"), page_svelte)
        .map_err(|e| e.to_string())?;

    // 10. .gitignore
    let gitignore = r#"node_modules
.svelte-kit
build
.env
.env.*
!.env.example
vite.config.js.timestamp-*
vite.config.ts.timestamp-*
"#;
    fs::write(project_dir.join(".gitignore"), gitignore).map_err(|e| e.to_string())?;

    // 11. README.md
    let readme = format!(
        r#"# {name}

Projecto SvelteKit gerado automaticamente pelo **NegoDesign Studio**.

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
        framework: "SvelteKit".to_string(),
        build_tool: "pnpm / npm / vite".to_string(),
        generated_files: 11,
        dependencies: all_deps,
        next_steps: vec![
            format!("cd {}", project_dir.display()),
            "pnpm install (ou npm install)".to_string(),
            "pnpm dev".to_string(),
        ],
    })
}

