# NegoDesign Desktop — Rust Backend (`src-tauri/src`)

High-performance native Tauri v2 backend responsible for project management, recursive component dependency resolution, Bootswatch theme bundling, static asset relocation, database schema inspection, and OpenAPI entity extraction.

---

## Architecture & Module Structure

Every repository in `src-tauri/src` follows a strict single-responsibility modular layout to avoid monolithic "God files":

```text
src-tauri/src/
├── main.rs                              # Desktop binary entry point
├── lib.rs                               # Tauri Builder & IPC command registration
├── commands.rs                          # Thin #[tauri::command] IPC handlers
├── model/
│   ├── mod.rs
│   ├── project.rs                       # Project, ComponentFile, ComponentExport, ComponentCopyRequest
│   └── migration.rs                     # DatabaseConnection, DatabaseTable, OpenApiEntity
└── repository/
    ├── mod.rs
    ├── component_copy_repository.rs     # Orchestrator for Svelte, React, Next.js, Vue, Nuxt, Flutter copy
    ├── component_copy/
    │   ├── mod.rs
    │   ├── manifest.rs                  # OnceLock-cached dependence.json catalog & dependency resolver
    │   ├── collector.rs                 # Recursive component graph & i18n locale collector
    │   ├── assets.rs                    # Disk source reader, hook rewriter (.svelte.ts -> .ts), base64 fonts
    │   ├── theme.rs                     # Bootswatch theme CSS / Dart ThemeData bundler
    │   └── paths.rs                     # Path normalizer, base64 encoder, hook import transformer
    ├── project_repository.rs            # SQLite project CRUD & transactional component installer
    ├── project/
    │   ├── mod.rs
    │   ├── db.rs                        # SQLite WAL connection & schema migrations
    │   ├── detect.rs                    # Automatic framework detector (Svelte, React, Nextjs, Vue, Nuxt, Flutter)
    │   └── installer.rs                 # Path validator, file writer, pubspec.yaml updater, index generator
    ├── database_repository.rs           # Saved DB credentials & live schema inspection
    ├── database/
    │   ├── mod.rs
    │   └── drivers.rs                   # MySQL and PostgreSQL information_schema inspectors
    ├── openapi_repository.rs            # Remote URL & local YAML/JSON OpenAPI extractor
    └── openapi/
        ├── mod.rs
        └── parser.rs                    # OpenAPI/Swagger $ref, schema, field, and operation parsers
```

---

## Performance Optimizations

1. **Native Rust Component Copy Engine (`component_copy/`)**:
   - Eliminates the ~4 MB frontend `import.meta.glob` bundle by performing all `dependence.json` traversal, source file reading, `.woff2` base64 encoding, and Bootswatch CSS extraction natively in Rust.
2. **In-Memory Manifest Cache (`OnceLock<ManifestCatalog>`)**:
   - `ManifestCatalog::global` in `repository/component_copy/manifest.rs` scans `src/project/ui/**/dependence.json` once and caches the indexed graph in a thread-safe `OnceLock`, making subsequent copy operations instantaneous.
3. **SQLite WAL Mode & Atomic Transactions (`project/db.rs` & `project_repository.rs`)**:
   - Enables `PRAGMA journal_mode = WAL;` and `PRAGMA synchronous = NORMAL;`, wrapping project activation and multi-component installations in single SQLite transactions.

---

## Tauri IPC Commands (`commands.rs`)

### Project & Component Installation
| Command | Arguments | Returns | Description |
| :--- | :--- | :--- | :--- |
| `list_projects` | — | `Result<Vec<Project>, String>` | Lists all registered projects and auto-detects unknown project frameworks. |
| `save_project` | `path: String` | `Result<Project, String>` | Registers or updates a project folder, detects its framework, and marks it active. |
| `set_active_project` | `id: i64` | `Result<Project, String>` | Switches the currently active target project. |
| `list_active_project_components` | — | `Result<Vec<ComponentExport>, String>` | Lists all components installed in the active project. |
| `install_component` | `files: Vec<ComponentFile>`, `components: Vec<ComponentExport>` | `Result<ComponentInstallResult, String>` | Writes files to `negodesign/` and static assets to `static/`, `public/`, or `assets/`, then regenerates `index.ts` or `index.dart`. |

### Per-Technology High-Performance Copy Commands
| Command | Arguments | Target Language | Description |
| :--- | :--- | :--- | :--- |
| `copy_component_svelte` | `request: ComponentCopyRequest` | `Svelte` | Resolves component tree, relocates static assets to `static/nego`, exports selected Bootswatch theme, and installs into active Svelte project. |
| `copy_component_react` | `request: ComponentCopyRequest` | `React` | Resolves component tree, converts `.svelte.ts` hooks to `.ts`, exports Bootswatch theme, and installs into active React project. |
| `copy_component_nextjs` | `request: ComponentCopyRequest` | `React` (Next.js) | Uses React component entries and installs into active Next.js project (`public/` + `negodesign/`). |
| `copy_component_vue` | `request: ComponentCopyRequest` | `Vue` | Resolves Vue SFC dependency tree, transforms shared hooks, exports Bootswatch theme, and installs into active Vue project. |
| `copy_component_nuxt` | `request: ComponentCopyRequest` | `Vue` (Nuxt) | Uses Vue SFC entries and installs into active Nuxt project. |
| `copy_component_flutter` | `request: ComponentCopyRequest` | `Flutter` | Collects Flutter MVVM widgets + `*ViewModel` classes, bundles Dart Bootswatch palettes (`theme/dart/*.dart`), updates `pubspec.yaml`, and generates `negodesign/index.dart`. |

### Database & OpenAPI Inspection
| Command | Arguments | Returns | Description |
| :--- | :--- | :--- | :--- |
| `connect_database` | `connection: DatabaseConnection` | `Result<Vec<DatabaseTable>, String>` | Inspects MySQL or PostgreSQL tables and persists connection metadata in `TB_DATABASE_CONNECTIONS`. |
| `list_saved_database_connections` | — | `Result<Vec<SavedDatabaseConnection>, String>` | Returns up to 8 most recently used database connections. |
| `connect_saved_database` | `id: i64` | `Result<Vec<DatabaseTable>, String>` | Reconnects using stored credentials and returns inspected tables. |
| `fetch_openapi` | `url: String` | `Result<Vec<OpenApiEntity>, String>` | Downloads an OpenAPI/Swagger JSON or YAML specification over HTTP(S) and extracts schemas and operations. |
| `import_openapi_file` | `path: String` | `Result<Vec<OpenApiEntity>, String>` | Reads a local OpenAPI/Swagger JSON or YAML file from disk and extracts schemas and operations. |

