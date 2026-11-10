# PaginationNav (`pagination-nav`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/PaginationNav.svelte`
- **React**: `ui/PaginationNav.tsx`
- **Vue**: `ui/PaginationNav.vue`
- **Flutter View**: `ui/PaginationNav.dart`
- **Component Dependencies**: None (Standalone)

## Props (`PaginationNavProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `totalPages` | `number` | No |
| `currentPage` | `number` | No |
| `size` | `'sm' \| 'md' \| 'lg'` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`PaginationNavViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import PaginationNav from "./ui/PaginationNav.svelte";
</script>

<PaginationNav />
```

### React (TSX)
```tsx
import PaginationNav from "./ui/PaginationNav";

export function Example() {
  return <PaginationNav />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import PaginationNav from "./ui/PaginationNav.vue";
</script>

<template>
  <PaginationNav />
</template>
```

### Flutter (Dart)
```dart
import 'ui/PaginationNav.dart';

// Inside build():
// PaginationNav(props)
```
