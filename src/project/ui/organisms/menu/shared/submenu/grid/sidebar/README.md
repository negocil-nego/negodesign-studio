# SubmenuSidebarGrid (`submenu-sidebar-grid`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SubmenuSidebarGrid.svelte`
- **React**: `ui/SubmenuSidebarGrid.tsx`
- **Vue**: `ui/SubmenuSidebarGrid.vue`
- **Flutter View**: `ui/SubmenuSidebarGrid.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`SubmenuSidebarGridViewModel`)
- **Component Dependencies**: `menu-grid-text`, `menu-title`, `submenu-title`

## Props (`SubmenuSidebarGridProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `SubmenuSidebarGridProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SubmenuSidebarGridViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SubmenuSidebarGrid from "./ui/SubmenuSidebarGrid.svelte";
</script>

<SubmenuSidebarGrid />
```

### React (TSX)
```tsx
import SubmenuSidebarGrid from "./ui/SubmenuSidebarGrid";

export function Example() {
  return <SubmenuSidebarGrid />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SubmenuSidebarGrid from "./ui/SubmenuSidebarGrid.vue";
</script>

<template>
  <SubmenuSidebarGrid />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SubmenuSidebarGrid.dart';

// Inside build():
// SubmenuSidebarGrid(props)
```
