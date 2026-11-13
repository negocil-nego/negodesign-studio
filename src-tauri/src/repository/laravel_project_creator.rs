use crate::model::codegen::{BackendProjectResult, DatabaseEngine, LaravelProjectRequest};
use std::{fs, path::PathBuf};

const LARAVEL_VERSION: &str = "^12.0";
const PHP_VERSION: &str = "^8.2";

fn db_connection(engine: &DatabaseEngine) -> (&'static str, &'static str, u16) {
    match engine {
        DatabaseEngine::Postgres => ("pgsql", "pgsql", 5432),
        DatabaseEngine::Mysql => ("mysql", "mysql", 3306),
        DatabaseEngine::Sqlite => ("sqlite", "sqlite", 0),
    }
}

fn composer_json(req: &LaravelProjectRequest) -> String {
    format!(
        r#"{{
    "name": "negodesign/{name}",
    "type": "project",
    "description": "{description}",
    "keywords": ["laravel", "framework"],
    "license": "MIT",
    "require": {{
        "php": "{php}",
        "laravel/framework": "{laravel}"
    }},
    "autoload": {{
        "psr-4": {{
            "App\\": "app/",
            "Database\\Factories\\": "database/factories/",
            "Database\\Seeders\\": "database/seeders/"
        }}
    }},
    "autoload-dev": {{
        "psr-4": {{
            "Tests\\": "tests/"
        }}
    }},
    "scripts": {{
        "post-autoload-dump": [
            "Illuminate\\Foundation\\ComposerScripts::postAutoloadDump",
            "@php artisan package:discover --ansi"
        ]
    }},
    "config": {{
        "optimize-autoloader": true,
        "preferred-install": "dist",
        "sort-packages": true
    }},
    "minimum-stability": "stable",
    "prefer-stable": true
}}
"#,
        name = req.name,
        description = req.description,
        php = PHP_VERSION,
        laravel = LARAVEL_VERSION,
    )
}

fn env_file(engine: &DatabaseEngine) -> String {
    let (connection, _, port) = db_connection(engine);
    match engine {
        DatabaseEngine::Sqlite => {
            format!(
                "APP_NAME=Negodesign\nAPP_ENV=local\nAPP_KEY=\nAPP_DEBUG=true\nAPP_URL=http://localhost:8000\n\nDB_CONNECTION={connection}\nDB_DATABASE=database/negodesign.sqlite\n\nCACHE_STORE=file\nSESSION_DRIVER=file\nQUEUE_CONNECTION=sync\n"
            )
        }
        _ => format!(
            "APP_NAME=Negodesign\nAPP_ENV=local\nAPP_KEY=\nAPP_DEBUG=true\nAPP_URL=http://localhost:8000\n\nDB_CONNECTION={connection}\nDB_HOST=127.0.0.1\nDB_PORT={port}\nDB_DATABASE=negodesign\nDB_USERNAME=root\nDB_PASSWORD=root\n\nCACHE_STORE=file\nSESSION_DRIVER=file\nQUEUE_CONNECTION=sync\n"
        ),
    }
}

fn artisan_file() -> String {
    r#"#!/usr/bin/env php
<?php

use Illuminate\Foundation\Console\Kernel;
use Symfony\Component\Console\Input\ArgvInput;

define('LARAVEL_START', microtime(true));

require __DIR__.'/vendor/autoload.php';

$app = require_once __DIR__.'/bootstrap/app.php';

$kernel = $app->make(Kernel::class);

$status = $kernel->handle(
    $input = new ArgvInput,
    new Symfony\Component\Console\Output\ConsoleOutput
);

$kernel->terminate($input, $status);

exit($status);
"#
    .to_string()
}

fn bootstrap_app() -> String {
    r#"<?php

use Illuminate\Foundation\Application;
use Illuminate\Foundation\Configuration\Exceptions;
use Illuminate\Foundation\Configuration\Middleware;

return Application::configure(basePath: dirname(__DIR__))
    ->withRouting(
        web: __DIR__.'/../routes/web.php',
        api: __DIR__.'/../routes/api.php',
        commands: __DIR__.'/../routes/console.php',
        health: '/up',
    )
    ->withMiddleware(function (Middleware $middleware) {
        //
    })
    ->withExceptions(function (Exceptions $exceptions) {
        //
    })->create();
"#
    .to_string()
}

fn routes_api_php() -> String {
    r#"<?php

use Illuminate\Support\Facades\Route;

Route::get('/', function () {
    return response()->json(['message' => 'Negodesign API is running!']);
});
"#
    .to_string()
}

fn routes_web_php() -> String {
    r#"<?php

use Illuminate\Support\Facades\Route;

Route::get('/', function () {
    return view('welcome');
});
"#
    .to_string()
}

fn database_config(engine: &DatabaseEngine) -> String {
    let (connection, driver, port) = db_connection(engine);
    match engine {
        DatabaseEngine::Sqlite => {
            format!(
                r#"<?php

return [
    'default' => env('DB_CONNECTION', '{connection}'),
    'connections' => [
        'sqlite' => [
            'driver' => 'sqlite',
            'url' => env('DB_URL'),
            'database' => env('DB_DATABASE', database_path('negodesign.sqlite')),
            'prefix' => '',
            'foreign_key_constraints' => env('DB_FOREIGN_KEYS', true),
        ],
    ],
    'migrations' => [
        'table' => 'migrations',
        'update_date_on_publish' => true,
    ],
];
"#
            )
        }
        _ => format!(
            r#"<?php

return [
    'default' => env('DB_CONNECTION', '{connection}'),
    'connections' => [
        '{connection}' => [
            'driver' => '{driver}',
            'url' => env('DB_URL'),
            'host' => env('DB_HOST', '127.0.0.1'),
            'port' => env('DB_PORT', '{port}'),
            'database' => env('DB_DATABASE', 'negodesign'),
            'username' => env('DB_USERNAME', 'root'),
            'password' => env('DB_PASSWORD', 'root'),
            'unix_socket' => env('DB_SOCKET', ''),
            'charset' => 'utf8mb4',
            'collation' => 'utf8mb4_unicode_ci',
            'prefix' => '',
            'strict' => true,
            'engine' => null,
        ],
    ],
    'migrations' => [
        'table' => 'migrations',
        'update_date_on_publish' => true,
    ],
];
"#
        ),
    }
}

fn readme_md(req: &LaravelProjectRequest) -> String {
    let db_name = match req.database_engine {
        DatabaseEngine::Postgres => "PostgreSQL",
        DatabaseEngine::Mysql => "MySQL",
        DatabaseEngine::Sqlite => "SQLite",
    };
    let orm_label = match req.orm_strategy.as_str() {
        "eloquent" => "Eloquent ORM",
        "query-builder" => "Query Builder",
        "raw-sql" => "Raw SQL",
        _ => &req.orm_strategy,
    };
    format!(
        "# {name}\n\n{description}\n\n## Stack técnica\n\n- **Laravel** {LARAVEL_VERSION}\n- **PHP** {PHP_VERSION}\n- **ORM**: {orm}\n- **Base de dados**: {db}\n\n## Como executar\n\n```bash\ncomposer install\nphp artisan key:generate\nphp artisan serve\n```\n\nA aplicação arranca em `http://localhost:8000`.\n\n## Próximos passos\n\n1. Configure a ligação à base de dados em `.env`.\n2. Execute `php artisan key:generate` para gerar a chave da aplicação.\n3. Execute `php artisan serve` para iniciar o servidor.\n4. Use o Negodesign para gerar os endpoints CRUD a partir das tabelas da base de dados.\n",
        name = req.name,
        description = req.description,
        orm = orm_label,
        db = db_name,
    )
}

pub fn create_laravel_project(
    req: &LaravelProjectRequest,
) -> Result<BackendProjectResult, String> {
    let target = PathBuf::from(&req.target_dir).join(&req.name);
    fs::create_dir_all(&target).map_err(|e| e.to_string())?;

    let dirs = [
        "app/Http/Controllers",
        "app/Models",
        "bootstrap",
        "config",
        "database/migrations",
        "database/factories",
        "database/seeders",
        "routes",
        "resources/views",
        "storage/app",
        "storage/framework/cache",
        "storage/framework/sessions",
        "storage/framework/views",
        "storage/logs",
        "tests",
    ];
    for dir in dirs {
        fs::create_dir_all(target.join(dir)).map_err(|e| e.to_string())?;
    }

    let mut generated = 0usize;

    fs::write(target.join("composer.json"), composer_json(req)).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join(".env"), env_file(&req.database_engine)).map_err(|e| e.to_string())?;
    generated += 1;

    let artisan = artisan_file();
    fs::write(target.join("artisan"), artisan).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("bootstrap/app.php"), bootstrap_app()).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("routes/api.php"), routes_api_php()).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("routes/web.php"), routes_web_php()).map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(
        target.join("config/database.php"),
        database_config(&req.database_engine),
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(
        target.join("resources/views/welcome.blade.php"),
        "<h1>Negodesign Laravel API</h1>\n",
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(
        target.join(".gitignore"),
        "/vendor\nnode_modules\n.env\n/storage/*.key\n",
    )
    .map_err(|e| e.to_string())?;
    generated += 1;

    fs::write(target.join("README.md"), readme_md(req)).map_err(|e| e.to_string())?;
    generated += 1;

    let dep_display = vec![
        "laravel/framework".to_string(),
        "php".to_string(),
    ];

    let next_steps = vec![
        format!("cd {}", req.name),
        "composer install".to_string(),
        "php artisan key:generate".to_string(),
        "php artisan serve".to_string(),
        "Abrir http://localhost:8000".to_string(),
    ];

    Ok(BackendProjectResult {
        project_dir: target.to_string_lossy().to_string(),
        build_tool: "Composer".to_string(),
        generated_files: generated,
        dependencies: dep_display,
        next_steps,
    })
}
