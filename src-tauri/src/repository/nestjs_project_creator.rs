use crate::model::codegen::{BackendProjectResult, DatabaseEngine, NestjsProjectRequest};
use std::{fs, path::PathBuf};

const NESTJS_VERSION: &str = "^11.0.0";
const TYPESCRIPT_VERSION: &str = "^5.9.0";
const PRISMA_VERSION: &str = "^6.5.0";
const DRIZZLE_VERSION: &str = "^0.41.0";
const TYPEORM_VERSION: &str = "^0.3.21";

fn db_driver(engine: &DatabaseEngine) -> (&'static str, &'static str) {
    match engine {
        DatabaseEngine::Postgres => ("pg", "^8.15.0"),
        DatabaseEngine::Mysql => ("mysql2", "^3.14.0"),
        DatabaseEngine::Sqlite => ("better-sqlite3", "^11.8.0"),
    }
}

fn db_name(engine: &DatabaseEngine) -> &'static str {
    match engine {
        DatabaseEngine::Postgres => "postgres",
        DatabaseEngine::Mysql => "mysql",
        DatabaseEngine::Sqlite => "sqlite",
    }
}

fn db_port(engine: &DatabaseEngine) -> u16 {
    match engine {
        DatabaseEngine::Postgres => 5432,
        DatabaseEngine::Mysql => 3306,
        DatabaseEngine::Sqlite => 0,
    }
}

fn orm_deps(orm_strategy: &str, engine: &DatabaseEngine) -> Vec<(&'static str, &'static str)> {
    match orm_strategy {
        "prisma" => vec![
            ("@prisma/client", PRISMA_VERSION),
            ("prisma", PRISMA_VERSION),
        ],
        "drizzle" => {
            let (driver, _) = db_driver(engine);
            vec![
                ("drizzle-orm", DRIZZLE_VERSION),
                ("drizzle-kit", DRIZZLE_VERSION),
                (driver, driver_version(engine)),
            ]
        }
        "typeorm" => {
            let (driver, _) = db_driver(engine);
            vec![
                ("@nestjs/typeorm", "^11.0.0"),
                ("typeorm", TYPEORM_VERSION),
                (driver, driver_version(engine)),
            ]
        }
        _ => vec![],
    }
}

fn driver_version(engine: &DatabaseEngine) -> &'static str {
    match engine {
        DatabaseEngine::Postgres => "^8.15.0",
        DatabaseEngine::Mysql => "^3.14.0",
        DatabaseEngine::Sqlite => "^11.8.0",
    }
}

fn package_json(req: &NestjsProjectRequest, deps: &[(&str, &str)], dev_deps: &[(&str, &str)]) -> String {
    let dep_lines = deps
        .iter()
        .map(|(name, ver)| format!("    \"{name}\": \"{ver}\""))
        .collect::<Vec<_>>()
        .join(",\n");
    let dev_dep_lines = dev_deps
        .iter()
        .map(|(name, ver)| format!("    \"{name}\": \"{ver}\""))
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        r#"{{
  "name": "{name}",
  "version": "0.0.1",
  "description": "{description}",
  "author": "",
  "private": true,
  "license": "MIT",
  "scripts": {{
    "build": "nest build",
    "start": "nest start",
    "start:dev": "nest start --watch",
    "start:prod": "node dist/main"
  }},
  "dependencies": {{
{dep_lines}
  }},
  "devDependencies": {{
{dev_dep_lines}
  }}
}}
"#,
        name = req.name,
        description = req.description,
    )
}

fn tsconfig_json() -> String {
    r#"{
  "compilerOptions": {
    "module": "commonjs",
    "declaration": true,
    "removeComments": true,
    "emitDecoratorMetadata": true,
    "experimentalDecorators": true,
    "allowSyntheticDefaultImports": true,
    "target": "ES2023",
    "sourceMap": true,
    "outDir": "./dist",
    "baseUrl": "./",
    "incremental": true,
    "skipLibCheck": true,
    "strictNullChecks": true,
    "noImplicitAny": false,
    "strictBindCallApply": false,
    "forceConsistentCasingInFileNames": false,
    "noFallthroughCasesInSwitch": false
  }
}
"#
    .to_string()
}

fn nest_cli_json() -> String {
    r#"{
  "$schema": "https://json.schemastore.org/nest-cli",
  "collection": "@nestjs/schematics",
  "sourceRoot": "src",
  "compilerOptions": {
    "deleteOutDir": true
  }
}
"#
    .to_string()
}

fn main_ts(req: &NestjsProjectRequest) -> String {
    let swagger = if req.include_api_docs {
        "import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';\n\n"
    } else {
        ""
    };
    let swagger_setup = if req.include_api_docs {
        r#"
  const config = new DocumentBuilder()
    .setTitle('Negodesign API')
    .setDescription('API REST gerada pelo Negodesign')
    .setVersion('1.0')
    .build();
  const document = SwaggerModule.createDocument(app, config);
  SwaggerModule.setup('api', app, document);
"#
    } else {
        ""
    };

    format!(
        "{swagger}import {{ NestFactory }} from '@nestjs/core';\n\
import {{ AppModule }} from './app.module';\n\n\
async function bootstrap() {{\n\
  const app = await NestFactory.create(AppModule);\n\
  app.enableCors();{swagger_setup}\
  await app.listen(process.env.PORT ?? 3000);\n\
}}\n\
bootstrap();\n"
    )
}

fn app_module_ts(req: &NestjsProjectRequest) -> String {
    let orm_import = match req.orm_strategy.as_str() {
        "typeorm" => format!("import {{ TypeOrmModule }} from '@nestjs/typeorm';\n\n"),
        _ => String::new(),
    };
    let orm_setup = match req.orm_strategy.as_str() {
        "typeorm" => {
            let port = db_port(&req.database_engine);
            let url = match &req.database_engine {
                DatabaseEngine::Sqlite => "sqlite://negodesign.db".to_string(),
                _ => format!("{}://root:root@localhost:{}/negodesign", db_name(&req.database_engine), port),
            };
            format!(
                "    TypeOrmModule.forRoot({{\n      type: '{}',\n      url: '{}',\n      autoLoadEntities: true,\n      synchronize: true,\n    }}),\n",
                db_name(&req.database_engine),
                url
            )
        }
        _ => String::new(),
    };

    format!(
        "import {{ Module }} from '@nestjs/common';\n\
import {{ AppController }} from './app.controller';\n\
import {{ AppService }} from './app.service';\n\
{orm_import}\
\n\
@Module({{\n\
  imports: [\n{orm_setup}\
  ],\n\
  controllers: [AppController],\n\
  providers: [AppService],\n\
}})\n\
export class AppModule {{}}\n"
    )
}

fn app_controller_ts() -> String {
    r#"import { Controller, Get } from '@nestjs/common';
import { AppService } from './app.service';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Get()
  getHello(): string {
    return this.appService.getHello();
  }
}
"#
    .to_string()
}

fn app_service_ts() -> String {
    r#"import { Injectable } from '@nestjs/common';

@Injectable()
export class AppService {
  getHello(): string {
    return 'Negodesign API is running!';
  }
}
"#
    .to_string()
}

fn env_file(engine: &DatabaseEngine) -> String {
    let (_, port) = match engine {
        DatabaseEngine::Postgres => ("", 5432),
        DatabaseEngine::Mysql => ("", 3306),
        DatabaseEngine::Sqlite => ("", 0),
    };
    match engine {
        DatabaseEngine::Sqlite => {
            "PORT=3000\nDATABASE_URL=sqlite://negodesign.db\n".to_string()
        }
        _ => format!(
            "PORT=3000\nDATABASE_URL={}://root:root@localhost:{}/negodesign\n",
            db_name(engine),
            port
        ),
    }
}

fn prisma_schema(engine: &DatabaseEngine) -> String {
    let provider = match engine {
        DatabaseEngine::Postgres => "postgresql",
        DatabaseEngine::Mysql => "mysql",
        DatabaseEngine::Sqlite => "sqlite",
    };
    format!(
        r#"generator client {{
  provider = "prisma-client-js"
}}

datasource db {{
  provider = "{provider}"
  url      = env("DATABASE_URL")
}}
"#
    )
}

fn readme_md(req: &NestjsProjectRequest) -> String {
    format!(
        "# {name}\n\n{description}\n\n## Stack técnica\n\n- **NestJS** {NESTJS_VERSION}\n- **Linguagem**: TypeScript\n- **ORM**: {orm}\n- **Base de dados**: {db}\n{api_line}\n\n## Como executar\n\n```bash\nnpm install\nnpm run start:dev\n```\n\nA aplicação arranca em `http://localhost:3000`.\n\n{swagger_block}## Próximos passos\n\n1. Configure a ligação à base de dados em `.env`.\n2. Execute `npm run start:dev` para iniciar o servidor.\n3. Use o Negodesign para gerar os endpoints CRUD a partir das tabelas da base de dados.\n",
        name = req.name,
        description = req.description,
        orm = match req.orm_strategy.as_str() {
            "prisma" => "Prisma ORM",
            "drizzle" => "Drizzle ORM",
            "typeorm" => "TypeORM",
            _ => &req.orm_strategy,
        },
        db = match req.database_engine {
            DatabaseEngine::Postgres => "PostgreSQL",
            DatabaseEngine::Mysql => "MySQL",
            DatabaseEngine::Sqlite => "SQLite",
        },
        api_line = if req.include_api_docs { "- **API Docs**: Swagger (NestJS Swagger)" } else { "" },
        swagger_block = if req.include_api_docs {
            "## Documentação da API\n\n- Swagger UI: `http://localhost:3000/api`\n\n"
        } else {
            ""
        },
    )
}

pub fn create_nestjs_project(
    req: &NestjsProjectRequest,
) -> Result<BackendProjectResult, String> {
    let target = PathBuf::from(&req.target_dir).join(&req.name);
    fs::create_dir_all(&target).map_err(|e| e.to_string())?;
    fs::create_dir_all(target.join("src")).map_err(|e| e.to_string())?;

    let mut deps: Vec<(&str, &str)> = vec![
        ("@nestjs/common", NESTJS_VERSION),
        ("@nestjs/core", NESTJS_VERSION),
        ("@nestjs/platform-express", NESTJS_VERSION),
        ("reflect-metadata", "^0.2.2"),
        ("rxjs", "^7.8.1"),
    ];
    deps.extend(orm_deps(&req.orm_strategy, &req.database_engine));
    if req.include_api_docs {
        deps.push(("@nestjs/swagger", "^11.0.0"));
    }

    let dev_deps: Vec<(&str, &str)> = vec![
        ("@nestjs/cli", "^11.0.0"),
        ("@nestjs/schematics", "^11.0.0"),
        ("@nestjs/testing", NESTJS_VERSION),
        ("typescript", TYPESCRIPT_VERSION),
        ("@types/node", "^22.15.0"),
        ("ts-node", "^10.9.2"),
        ("tsconfig-paths", "^4.2.0"),
    ];

    let mut generated = 0usize;

    fs::write(
        target.join("package.json"),
        package_json(req, &deps, &dev_deps),
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("tsconfig.json"), tsconfig_json()).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("nest-cli.json"), nest_cli_json()).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("src/main.ts"), main_ts(req)).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("src/app.module.ts"), app_module_ts(req)).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("src/app.controller.ts"), app_controller_ts()).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("src/app.service.ts"), app_service_ts()).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join(".env"), env_file(&req.database_engine)).map_err(|e| e.to_string())?;
    generated += 1;

    if req.orm_strategy == "prisma" {
        fs::create_dir_all(target.join("prisma")).map_err(|e| e.to_string())?;
        fs::write(
            target.join("prisma/schema.prisma"),
            prisma_schema(&req.database_engine),
        )
        .map_err(|e| e.to_string())?;
        generated += 1;
    }

    fs::write(
        target.join(".gitignore"),
        "node_modules/\ndist/\n.env\n*.log\n",
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("README.md"), readme_md(req)).map_err(|e| e.to_string())?;
    generated += 1;

    let dep_display = deps
        .iter()
        .map(|(name, _)| name.to_string())
        .collect();

    let next_steps = vec![
        format!("cd {}", req.name),
        "npm install".to_string(),
        "npm run start:dev".to_string(),
        "Abrir http://localhost:3000".to_string(),
    ];

    Ok(BackendProjectResult {
        project_dir: target.to_string_lossy().to_string(),
        build_tool: "npm".to_string(),
        generated_files: generated,
        dependencies: dep_display,
        next_steps,
    })
}
