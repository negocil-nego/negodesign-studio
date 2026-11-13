use crate::model::codegen::{AspnetProjectRequest, BackendProjectResult, DatabaseEngine};
use std::{fs, path::PathBuf};

const DOTNET_VERSION: &str = "net9.0";
const EF_CORE_VERSION: &str = "9.0.0";
const DAPPER_VERSION: &str = "2.1.35";
const SWASHBUCKLE_VERSION: &str = "7.2.0";

fn db_packages(engine: &DatabaseEngine, orm: &str) -> Vec<(&'static str, &'static str)> {
    let is_ef = orm == "ef-core";
    match engine {
        DatabaseEngine::Postgres => {
            if is_ef {
                vec![("Npgsql.EntityFrameworkCore.PostgreSQL", EF_CORE_VERSION)]
            } else {
                vec![("Npgsql", "9.0.2")]
            }
        }
        DatabaseEngine::Mysql => {
            if is_ef {
                vec![("MySql.EntityFrameworkCore", EF_CORE_VERSION)]
            } else {
                vec![("MySqlConnector", "2.4.3")]
            }
        }
        DatabaseEngine::Sqlite => {
            if is_ef {
                vec![("Microsoft.EntityFrameworkCore.Sqlite", EF_CORE_VERSION)]
            } else {
                vec![("Microsoft.Data.Sqlite", "9.0.0")]
            }
        }
    }
}

fn orm_packages(orm: &str) -> Vec<(&'static str, &'static str)> {
    match orm {
        "ef-core" => vec![
            ("Microsoft.EntityFrameworkCore", EF_CORE_VERSION),
            ("Microsoft.EntityFrameworkCore.Design", EF_CORE_VERSION),
        ],
        "dapper" => vec![("Dapper", DAPPER_VERSION)],
        _ => vec![],
    }
}

fn db_connection_string(engine: &DatabaseEngine) -> String {
    match engine {
        DatabaseEngine::Postgres => "Host=localhost;Port=5432;Database=negodesign;Username=postgres;Password=postgres".to_string(),
        DatabaseEngine::Mysql => "Server=localhost;Port=3306;Database=negodesign;Uid=root;Pwd=root".to_string(),
        DatabaseEngine::Sqlite => "Data Source=negodesign.db".to_string(),
    }
}

fn csproj(req: &AspnetProjectRequest, packages: &[(&str, &str)]) -> String {
    let pkg_refs = packages
        .iter()
        .map(|(name, ver)| format!("    <PackageReference Include=\"{name}\" Version=\"{ver}\" />"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"<Project Sdk="Microsoft.NET.Sdk.Web">

  <PropertyGroup>
    <TargetFramework>{dotnet}</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
    <RootNamespace>{ns}</RootNamespace>
  </PropertyGroup>

  <ItemGroup>
{pkg_refs}
  </ItemGroup>

</Project>
"#,
        dotnet = DOTNET_VERSION,
        ns = req.name,
        pkg_refs = pkg_refs,
    )
}

fn program_cs(req: &AspnetProjectRequest) -> String {
    let use_ef = req.orm_strategy == "ef-core";
    let db_context = if use_ef {
        format!(
            "builder.Services.AddDbContext<AppDbContext>(options =>\n    options.Use{}(\n        builder.Configuration.GetConnectionString(\"DefaultConnection\")));\n\n",
            match req.database_engine {
                DatabaseEngine::Postgres => "Npgsql",
                DatabaseEngine::Mysql => "MySQL",
                DatabaseEngine::Sqlite => "Sqlite",
            }
        )
    } else {
        String::new()
    };
    let using_ef = if use_ef {
        "using Microsoft.EntityFrameworkCore;\n"
    } else {
        ""
    };
    let swagger_setup = if req.include_api_docs {
        "builder.Services.AddEndpointsApiExplorer();\nbuilder.Services.AddSwaggerGen();\n\n"
    } else {
        ""
    };
    let swagger_middleware = if req.include_api_docs {
        "app.UseSwagger();\napp.UseSwaggerUI();\n\n"
    } else {
        ""
    };

    format!(
        "{using_ef}using {namespace}.Data;\n\nvar builder = WebApplication.CreateBuilder(args);\n\n{swagger_setup}{db_context}var app = builder.Build();\n\n{swagger_middleware}app.MapGet(\"/\", () => new {{ message = \"Negodesign API is running!\" }});\n\napp.Run();\n"
    ,
        using_ef = using_ef,
        namespace = req.name,
        swagger_setup = swagger_setup,
        db_context = db_context,
        swagger_middleware = swagger_middleware,
    )
}

fn appsettings_json(engine: &DatabaseEngine) -> String {
    let conn = db_connection_string(engine);
    format!(
        r#"{{
  "Logging": {{
    "LogLevel": {{
      "Default": "Information",
      "Microsoft.AspNetCore": "Warning"
    }}
  }},
  "ConnectionStrings": {{
    "DefaultConnection": "{conn}"
  }},
  "AllowedHosts": "*"
}}
"#
    )
}

fn app_db_context_cs() -> String {
    r#"using Microsoft.EntityFrameworkCore;

namespace Negodesign.Data;

public class AppDbContext : DbContext
{
    public AppDbContext(DbContextOptions<AppDbContext> options) : base(options)
    {
    }

    // DbSets serão adicionados aqui pelo Negodesign
}
"#
    .to_string()
}

fn readme_md(req: &AspnetProjectRequest) -> String {
    let db_name = match req.database_engine {
        DatabaseEngine::Postgres => "PostgreSQL",
        DatabaseEngine::Mysql => "MySQL",
        DatabaseEngine::Sqlite => "SQLite",
    };
    let orm_label = match req.orm_strategy.as_str() {
        "ef-core" => "Entity Framework Core",
        "dapper" => "Dapper",
        "raw-sql" => "Raw SQL (ADO.NET)",
        _ => &req.orm_strategy,
    };
    let swagger_block = if req.include_api_docs {
        "## Documentação da API\n\n- Swagger UI: `http://localhost:5000/swagger`\n\n"
    } else {
        ""
    };
    format!(
        "# {name}\n\n{description}\n\n## Stack técnica\n\n- **.NET** {dotnet}\n- **Linguagem**: C#\n- **ORM**: {orm}\n- **Base de dados**: {db}\n{api_line}\n\n## Como executar\n\n```bash\ndotnet restore\ndotnet run\n```\n\nA aplicação arranca em `http://localhost:5000`.\n\n{swagger_block}## Próximos passos\n\n1. Configure a ligação à base de dados em `appsettings.json`.\n2. Execute `dotnet ef migrations add Initial` (se usar EF Core).\n3. Execute `dotnet run` para iniciar o servidor.\n4. Use o Negodesign para gerar os endpoints CRUD a partir das tabelas da base de dados.\n",
        name = req.name,
        description = req.description,
        dotnet = DOTNET_VERSION,
        orm = orm_label,
        db = db_name,
        api_line = if req.include_api_docs { "- **API Docs**: Swagger (Swashbuckle)" } else { "" },
        swagger_block = swagger_block,
    )
}

pub fn create_aspnet_project(
    req: &AspnetProjectRequest,
) -> Result<BackendProjectResult, String> {
    let target = PathBuf::from(&req.target_dir).join(&req.name);
    fs::create_dir_all(&target).map_err(|e| e.to_string())?;
    if req.orm_strategy == "ef-core" {
        fs::create_dir_all(target.join("Data")).map_err(|e| e.to_string())?;
    }

    let mut packages: Vec<(&str, &str)> = vec![];
    packages.extend(orm_packages(&req.orm_strategy));
    packages.extend(db_packages(&req.database_engine, &req.orm_strategy));
    if req.include_api_docs {
        packages.push(("Swashbuckle.AspNetCore", SWASHBUCKLE_VERSION));
    }

    let mut generated = 0usize;

    fs::write(
        target.join(format!("{}.csproj", req.name)),
        csproj(req, &packages),
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("Program.cs"), program_cs(req)).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(
        target.join("appsettings.json"),
        appsettings_json(&req.database_engine),
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(
        target.join("appsettings.Development.json"),
        appsettings_json(&req.database_engine),
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    if req.orm_strategy == "ef-core" {
        fs::write(target.join("Data/AppDbContext.cs"), app_db_context_cs())
            .map_err(|e| e.to_string())?;
        generated += 1;
    }

    fs::write(
        target.join(".gitignore"),
        "bin/\nobj/\n*.user\n*.suo\n.vs/\n",
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("README.md"), readme_md(req)).map_err(|e| e.to_string())?;
    generated += 1;

    let dep_display = packages
        .iter()
        .map(|(name, _)| name.to_string())
        .collect();

    let next_steps = vec![
        format!("cd {}", req.name),
        "dotnet restore".to_string(),
        "dotnet run".to_string(),
        "Abrir http://localhost:5000".to_string(),
    ];

    Ok(BackendProjectResult {
        project_dir: target.to_string_lossy().to_string(),
        build_tool: "dotnet".to_string(),
        generated_files: generated,
        dependencies: dep_display,
        next_steps,
    })
}
