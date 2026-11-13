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

    fs::create_dir_all(project_dir.join("app")).map_err(|e| e.to_string())?;

    let mut dependencies: Vec<String> = vec![
        "next@^15.2.0".to_string(),
        "react@^19.0.0".to_string(),
        "react-dom@^19.0.0".to_string(),
    ];
    let mut dev_dependencies: Vec<String> = vec![
        "typescript@^5.7.0".to_string(),
        "@types/node@^22.0.0".to_string(),
        "@types/react@^19.0.0".to_string(),
        "@types/react-dom@^19.0.0".to_string(),
    ];

    if req.include_tailwind {
        dependencies.push("tailwindcss@^4.0.0".to_string());
        dev_dependencies.push("@tailwindcss/postcss@^4.0.0".to_string());
        dev_dependencies.push("postcss@^8.5.0".to_string());
    }

    if matches!(req.http_client, HttpClientKind::Axios) {
        dependencies.push("axios@^1.8.0".to_string());
    }

    if req.use_tanstack_query {
        dependencies.push("@tanstack/react-query@^5.66.0".to_string());
    }

    // 1. package.json
    let package_json = format!(
        r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "description": "{description}",
  "private": true,
  "scripts": {{
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint"
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

    // 2. next.config.ts
    let next_config = r#"import type { NextConfig } from "next";

const nextConfig: NextConfig = {};

export default nextConfig;
"#;
    fs::write(project_dir.join("next.config.ts"), next_config).map_err(|e| e.to_string())?;

    // 3. postcss.config.mjs (if tailwind)
    if req.include_tailwind {
        let postcss = r#"export default {
  plugins: {
    "@tailwindcss/postcss": {},
  },
};
"#;
        fs::write(project_dir.join("postcss.config.mjs"), postcss).map_err(|e| e.to_string())?;
    }

    // 4. tsconfig.json
    let tsconfig = r#"{
  "compilerOptions": {
    "target": "ES2017",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "paths": {
      "@/*": ["./*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
"#;
    fs::write(project_dir.join("tsconfig.json"), tsconfig).map_err(|e| e.to_string())?;

    // 5. app/globals.css
    let globals_css = if req.include_tailwind {
        "@import \"tailwindcss\";\n"
    } else {
        "/* Estilos globais */\n"
    };
    fs::write(project_dir.join("app/globals.css"), globals_css).map_err(|e| e.to_string())?;

    // 6. app/layout.tsx
    let layout_tsx = if req.use_tanstack_query {
        format!(
            r#"'use client';

import "./globals.css";
import React, {{ useState }} from "react";
import {{ QueryClient, QueryClientProvider }} from "@tanstack/react-query";

export default function RootLayout({{
  children,
}}: Readonly<{{
  children: React.ReactNode;
}}>) {{
  const [queryClient] = useState(() => new QueryClient());

  return (
    <html lang="pt">
      <body class="min-h-screen bg-zinc-950 text-zinc-100 antialiased">
        <QueryClientProvider client={{queryClient}}>
          {{children}}
        </QueryClientProvider>
      </body>
    </html>
  );
}}
"#
        )
    } else {
        format!(
            r#"import "./globals.css";
import type {{ Metadata }} from "next";

export const metadata: Metadata = {{
  title: "{name}",
  description: "{description}",
}};

export default function RootLayout({{
  children,
}}: Readonly<{{
  children: React.ReactNode;
}}>) {{
  return (
    <html lang="pt">
      <body class="min-h-screen bg-zinc-950 text-zinc-100 antialiased">
        {{children}}
      </body>
    </html>
  );
}}
"#,
            name = req.name,
            description = req.description
        )
    };
    fs::write(project_dir.join("app/layout.tsx"), layout_tsx).map_err(|e| e.to_string())?;

    // 7. app/page.tsx
    let page_tsx = format!(
        r#"import React from "react";
import Link from "next/link";

export default function Home() {{
  return (
    <div className="flex min-h-screen flex-col items-center justify-center p-6 text-center">
      <div className="max-w-md rounded-2xl border border-zinc-800 bg-zinc-900/80 p-8 shadow-2xl">
        <div className="mb-4 inline-flex size-14 items-center justify-center rounded-2xl bg-zinc-800 text-zinc-100">
          <span className="text-xl font-bold">N</span>
        </div>
        <h1 className="text-2xl font-bold tracking-tight text-white">{name}</h1>
        <p className="mt-2 text-sm text-zinc-400">Projecto Next.js (App Router) pronto para receber os ecrãs e módulos AdminCrud gerados pelo NegoDesign Studio.</p>
        <div className="mt-6 flex justify-center gap-3">
          <Link href="/admin" className="rounded-lg bg-zinc-100 px-4 py-2 text-xs font-semibold text-zinc-900 transition hover:bg-zinc-200">Aceder ao Painel</Link>
        </div>
      </div>
    </div>
  );
}}
"#,
        name = req.name
    );
    fs::write(project_dir.join("app/page.tsx"), page_tsx).map_err(|e| e.to_string())?;

    // 8. .gitignore
    let gitignore = r#"/node_modules
/.next/
/out/
/build
.DS_Store
*.pem
.env*.local
.vercel
*.tsbuildinfo
next-env.d.ts
"#;
    fs::write(project_dir.join(".gitignore"), gitignore).map_err(|e| e.to_string())?;

    // 9. README.md
    let readme = format!(
        r#"# {name}

Projecto Next.js gerado automaticamente pelo **NegoDesign Studio**.

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
        framework: "Next.js".to_string(),
        build_tool: "pnpm / npm / next".to_string(),
        generated_files: 9,
        dependencies: all_deps,
        next_steps: vec![
            format!("cd {}", project_dir.display()),
            "pnpm install (ou npm install)".to_string(),
            "pnpm dev".to_string(),
        ],
    })
}

