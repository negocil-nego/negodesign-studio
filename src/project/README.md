# NegoDesign Component System — Architecture & AI Training Guide

Welcome to the **NegoDesign** multi-platform UI architecture (`src/project`). This document serves both as developer documentation and as the **primary training specification for AI agents** building, refactoring, or extending components in **Svelte 5**, **React (TSX)**, **Vue 3**, and **Flutter (Dart)**.

---

## 1. Core Architectural Principles

### 1.1 Maximum 100 Lines Per File
Every code file (`.svelte`, `.tsx`, `.vue`, `.dart`, `.ts`) inside `src/project` **must not exceed 100 lines** (ideally 60–90 lines).
- If a Web component grows beyond 100 lines, extract state/view-model logic into `src/project/hooks/<feature>.hook.ts` or split sub-components into `shared/`.
- If a Flutter component has state, controllers, or internal logic, extract it into a **ViewModel** class in `data/viewmodel.dart` following **MVVM**, and place any internal sub-widgets in `ui/flutter/`.

### 1.2 Declarative Composition & Object Calisthenics (`*Props`)
All components follow **Flutter's declarative widget tree philosophy** and **Object Calisthenics**:
- Avoid primitive parameter lists with many loose arguments.
- Encapsulate related configuration into typed `*Props` objects (modeled after `CardMediaBaseProps` in `src/project/ui/molecules/card/media/data/types.ts`).
- Compose complex molecules and organisms by passing child `*Props` down to atomic components (`TextProps`, `IconProps`, `ButtonProps`, `CardLogoProps`, etc.).

### 1.3 Cross-Framework Web Equivalence & Shared Hooks (`src/project/hooks/`)
Svelte, React, and Vue components must share:
1. **Identical Props & Types**: Defined once in `data/types.ts` inside the component folder.
2. **Shared Pure TypeScript Hooks (`src/project/hooks/*.hook.ts`)**: All filtering, sorting, formatting, calendar calculations, time parsing, OTP slot logic, and state transitions live in framework-agnostic TypeScript hooks so logic is written once and reused across Svelte, React, and Vue.

### 1.4 Flutter Mobile Equivalence, MVVM (`data/viewmodel.dart`) & `ui/flutter/`
- **Strict MVVM Separation**: Flutter widgets (`ui/<Component>.dart`) are **purely responsible for building the UI tree**. All state fields, `TextEditingController`/`ScrollController`/`FocusNode` instances, `initState` initialization, `didUpdateWidget` synchronization, validation, and internal helper methods must live in `<Component>ViewModel` inside `data/viewmodel.dart` (typically extending `ChangeNotifier` and consumed via `ListenableBuilder`).
- **Example (`SidebarViewModel`, `InputOtpViewModel`, `DataTableViewModel`)**:
  - `data/viewmodel.dart`: Holds `SidebarViewModel` / `InputOtpViewModel` / `DataTableViewModel`.
  - `ui/<Component>.dart`: Instantiates the ViewModel, delegates `didUpdateWidget` and `dispose`, and renders the widget tree with `ListenableBuilder`.
- **Internal Sub-Widgets (`ui/flutter/`)**: Never place multiple helper widgets in the main `.dart` file if it increases complexity. Place Flutter-only internal widgets in `ui/flutter/<sub_widget>.dart` and register both `data/viewmodel.dart` and `ui/flutter/*.dart` in `dependence.json` under `files.Flutter`.

---

## 2. External Theme & Styling System (`ThemeData` / `StyleComponent`)

Components **never hardcode raw style blocks** when a theme token exists. Styling is controlled externally from `src/project/data/theme/` so consumers can re-theme the entire library without touching component files:

```
src/project/data/
├── style.component.ts           # Main Web theme entrypoint (StyleComponent & extendStyleComponent)
└── theme/
    ├── tokens.ts                # Intermediate cross-platform DesignTokens (Blender dark palette, radii)
    ├── theme.builder.ts         # Deep-merge theme factory (createStyleTheme — like Flutter copyWith)
    ├── theme_data.dart          # Flutter NegoThemeData + Material ThemeData converter
    ├── atoms.style.ts           # Button, input, select, badge, progress, skeleton, toast, tooltip
    ├── inputs.style.ts          # Date, time, password, OTP, number, phone, search, file styles
    ├── cards.style.ts           # Media, product, and shared card sub-component styles
    ├── cards_profile.style.ts   # Profile card variants (01, 02, 03)
    ├── datatable.style.ts       # DataTable & DataTableFilter Blender surface styles
    └── organisms.style.ts       # Carousel, form, modal, menu, sidebar styles
```

---

## 3. Unified External Icon Configuration (`src/project/data/icons/`)

Icons are configured centrally in `src/project/data/icons/` so icon sets can be swapped or extended without modifying `<Icon />`:
- **Web (`icons.config.ts`)**: Maps semantic icon names (`Search`, `FilterHorizontal`, `Columns`, `MoreVertical`, etc.) to **Hugeicons** CSS classes (`hgi-stroke hgi-...`) and SVG asset paths.
- **Flutter (`icons_config.dart`)**: Resolves semantic icon names to SVG asset paths (`assets/icons/<name>.svg`) via `NegoIconsConfig.resolveSvgPath(name)`.

---

## 4. Standard Component Folder Layout (`negodesign`)

When exported to a target project (`/src/negodesign` for Web or `lib/negodesign` for Flutter), every component follows this structure:
```
<component-folder>/
├── README.md                    # English documentation (Overview, Props, MVVM/Hooks, Theme, Examples)
├── dependence.json              # Component manifest (entry, files, dependencies, assets, props)
├── data/
│   ├── types.ts                 # TypeScript Props & types (shared by Svelte, React, Vue)
│   ├── types.dart               # Dart Props & models (for Flutter)
│   └── viewmodel.dart           # Flutter MVVM ViewModel (<Component>ViewModel)
├── shared/                      # Optional cross-platform child components (each with its own dependence.json)
└── ui/
    ├── <Component>.svelte       # Svelte 5 implementation (<= 100 lines)
    ├── <Component>.tsx          # React TSX implementation (<= 100 lines)
    ├── <Component>.vue          # Vue 3 setup implementation (<= 100 lines)
    ├── <Component>.dart         # Flutter View implementation (<= 100 lines, UI only)
    └── flutter/                 # Optional Flutter-only helper widgets (registered in dependence.json)
```
