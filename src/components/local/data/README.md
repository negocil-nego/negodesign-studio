# NegoDesign Studio — Local Data & Copy Engine (`src/components/local/data`)

This directory contains the state stores, component catalog definitions, Bootswatch theme controllers, database/OpenAPI migration helpers, and the modular per-framework component copy bridge that communicates with the native Rust Tauri backend.

---

## Directory Structure

```text
src/components/local/data/
├── README.md                    # Documentation for all modules and functions in this folder
├── projects.ts                  # Active project stores & Tauri IPC wrappers for project management
├── component-copy.ts            # High-level entry point to copy a component to the active project
├── copy/
│   ├── core.ts                  # Barrel re-exporting all per-framework copy functions & types
│   ├── shared/
│   │   ├── types.ts             # Shared TypeScript types (ComponentCopyOptions, ComponentCopyResult, etc.)
│   │   └── ipc.ts               # Tauri IPC dispatcher (invokeRustComponentCopy)
│   ├── svelte/index.ts          # copyComponentForProjectSvelte (invokes Rust `copy_component_svelte`)
│   ├── react/index.ts           # copyComponentForProjectReact (invokes Rust `copy_component_react`)
│   ├── nextjs/index.ts          # copyComponentForProjectNextjs (invokes Rust `copy_component_nextjs`)
│   ├── vue/index.ts             # copyComponentForProjectVue (invokes Rust `copy_component_vue`)
│   ├── nuxt/index.ts            # copyComponentForProjectNuxt (invokes Rust `copy_component_nuxt`)
│   └── flutter/index.ts         # copyComponentForProjectFlutter (invokes Rust `copy_component_flutter`)
├── bootswatch-ui.svelte.ts      # Reactive Svelte 5 controller for Bootswatch theme, scope, and dark/light mode
├── components.ts                # Component catalog (`componentCatalog`) and sidebar navigation groups
├── component-props.ts           # Interactive playground property definitions per component ID
├── migration.ts                 # Tauri IPC wrappers for SQL database inspection and OpenAPI extraction
├── style.component.ts           # Local UI theme token definitions (Blender dark/light tokens)
└── type.ts                      # Shared TypeScript interfaces for navigation and menus
```

---

## Module & Function Reference

### 1. `projects.ts` — Project Management & Stores

Manages the list of registered target projects and the components installed in the active project via Tauri IPC.

- **`activeProject: Writable<ProjectRecord | null>`**: Reactive Svelte store holding the currently active target project.
- **`activeProjectComponents: Writable<ComponentExport[]>`**: Reactive Svelte store holding all components installed in the active project.
- **`listProjects(): Promise<ProjectRecord[]>`**: Invokes `list_projects` in Rust, updates `activeProject`, and refreshes installed components.
- **`saveProject(path: string): Promise<ProjectRecord>`**: Invokes `save_project` in Rust to register or activate a project directory and detect its framework (`Svelte`, `React`, `Nextjs`, `Vue`, `Nuxt`, `Flutter`).
- **`setActiveProject(id: number): Promise<ProjectRecord>`**: Invokes `set_active_project` in Rust to switch the active project.
- **`listActiveProjectComponents(): Promise<ComponentExport[]>`**: Invokes `list_active_project_components` in Rust.
- **`refreshActiveProjectComponents(): Promise<ComponentExport[]>`**: Fetches installed components for the active project and updates `activeProjectComponents`.
- **`installComponent(files: ComponentFile[], components: ComponentExport[]): Promise<ComponentInstallResult>`**: Low-level IPC wrapper around `install_component`.

---

### 2. `component-copy.ts` & `copy/*` — Modular Rust IPC Component Copy Engine

Routes component copy requests to technology-specific modules that invoke dedicated native Rust Tauri commands.

#### `component-copy.ts`
- **`copyComponentToActiveProject(componentId: string, options?: ComponentCopyOptions): Promise<ComponentCopyResult>`**:
  - Reads the current `activeProject` (or loads projects if not yet initialized).
  - Dispatches the copy operation to the matching technology module based on `project.language`:
    - `Svelte` -> `copyComponentForProjectSvelte(componentId, project, { ...options, staticDirectory: "nego" })`
    - `React` -> `copyComponentForProjectReact(componentId, project, options)`
    - `Nextjs` -> `copyComponentForProjectNextjs(componentId, project, options)`
    - `Vue` -> `copyComponentForProjectVue(componentId, project, options)`
    - `Nuxt` -> `copyComponentForProjectNuxt(componentId, project, options)`
    - `Flutter` -> `copyComponentForProjectFlutter(componentId, project, options)`

#### `copy/shared/ipc.ts`
- **`invokeRustComponentCopy(command: RustCopyCommand, componentId: string, project: ProjectRecord, options?: ComponentCopyOptions): Promise<ComponentCopyResult>`**:
  - Looks up the component's `dependence.json` path from `componentCatalog`.
  - Calls the corresponding Rust Tauri command (`copy_component_svelte`, `copy_component_react`, `copy_component_nextjs`, `copy_component_vue`, `copy_component_nuxt`, or `copy_component_flutter`) with `{ componentId, manifestPath, locales, theme, staticDirectory }`.
  - Calls `refreshActiveProjectComponents()` after installation completes and returns `{ copiedFiles, exportedComponents, project }`.

#### Technology Copy Functions (`copy/[framework]/index.ts`)
- **`copyComponentForProjectSvelte(componentId, project, options)`** (`copy/svelte/index.ts`)
- **`copyComponentForProjectReact(componentId, project, options)`** (`copy/react/index.ts`)
- **`copyComponentForProjectNextjs(componentId, project, options)`** (`copy/nextjs/index.ts`)
- **`copyComponentForProjectVue(componentId, project, options)`** (`copy/vue/index.ts`)
- **`copyComponentForProjectNuxt(componentId, project, options)`** (`copy/nuxt/index.ts`)
- **`copyComponentForProjectFlutter(componentId, project, options)`** (`copy/flutter/index.ts`)

---

### 3. `bootswatch-ui.svelte.ts` — Bootswatch Theme & Scope Controller

Reactive Svelte 5 state manager (`bootswatchUi`) used by `BootswatchThemeDropdown.svelte`, `MenuNavigation.svelte`, and `ComponentsSidebar.svelte`.

- **`bootswatchUi.theme`**: Active theme ID (`"default"` or any of the 26 Bootswatch v5.3.8 themes).
- **`bootswatchUi.scope`**: Theme application scope (`"components"` for `src/project` preview only, or `"app"` for the entire application).
- **`bootswatchUi.colorMode`**: Current color mode (`"dark"` or `"light"`).
- **`bootswatchUi.init()`**: Restores persisted theme, scope, and color mode from `localStorage` and synchronizes DOM attributes.
- **`bootswatchUi.setTheme(next: BootswatchThemeSelection)`**: Applies and persists the selected Bootswatch theme.
- **`bootswatchUi.setScope(next: ThemeApplicationScope)`**: Switches between component-only preview scope (`[data-project-theme-scope]`) and global application scope (`document.documentElement`).
- **`bootswatchUi.toggleColorMode()`**: Toggles between dark and light mode.

---

### 4. `components.ts` & `component-props.ts` — Component Catalog & Playground Props

- **`componentCatalog: ComponentConfig[]`** (`components.ts`): Master list of all UI components in `src/project/ui` (Atoms, Molecules, Organisms, Cards, Menu, DataTable, Login/Auth), including their `id`, `name`, `category`, `dependence` manifest path, and default preview props.
- **`componentNavigationGroups: ComponentNavigationGroup[]`** (`components.ts`): Grouped navigation tree used by `ComponentsSidebar.svelte`.
- **`getComponentProps(componentId: string)`** (`component-props.ts`): Returns interactive property descriptors for the component editor inspector.

---

### 5. `migration.ts` — Database & OpenAPI Migration Helpers

- **`connectDatabase(connection: DatabaseConnectionInput): Promise<DatabaseTableSchema[]>`**: Invokes `connect_database` in Rust to inspect MySQL/PostgreSQL tables.
- **`listSavedDatabaseConnections(): Promise<SavedDatabaseConnection[]>`**: Invokes `list_saved_database_connections` in Rust.
- **`connectSavedDatabase(id: number): Promise<DatabaseTableSchema[]>`**: Invokes `connect_saved_database` in Rust.
- **`fetchOpenApiEntities(url: string): Promise<OpenApiEntitySchema[]>`**: Invokes `fetch_openapi` in Rust.
- **`importOpenApiFile(path: string): Promise<OpenApiEntitySchema[]>`**: Invokes `import_openapi_file` in Rust.

