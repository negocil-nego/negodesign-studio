# Sidebar (`sidebar`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Sidebar.svelte`
- **React**: `ui/Sidebar.tsx`
- **Vue**: `ui/Sidebar.vue`
- **Flutter View**: `ui/Sidebar.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`SidebarViewModel`)
- **Component Dependencies**: `icon`

## Props (`SidebarProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `variant` | `01 \| 02 \| 03 \| 04` | No |
| `theme` | `light \| dark` | No |
| `collapsed` | `boolean` | No |
| `sections` | `SidebarSectionProps[]` | No |
| `footerSections` | `SidebarSectionProps[]` | No |
| `user` | `SidebarProfileProps` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SidebarViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Sidebar from "./ui/Sidebar.svelte";
</script>

<Sidebar />
```

### React (TSX)
```tsx
import Sidebar from "./ui/Sidebar";

export function Example() {
  return <Sidebar />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Sidebar from "./ui/Sidebar.vue";
</script>

<template>
  <Sidebar />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Sidebar.dart';

// Inside build():
// Sidebar(props)
```
