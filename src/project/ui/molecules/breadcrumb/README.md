# Breadcrumbs (`breadcrumbs`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Breadcrumbs.svelte`
- **React**: `ui/Breadcrumbs.tsx`
- **Vue**: `ui/Breadcrumbs.vue`
- **Flutter View**: `ui/Breadcrumbs.dart`
- **Component Dependencies**: None (Standalone)

## Props (`BreadcrumbsProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `items` | `BreadcrumbItem[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`BreadcrumbsViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Breadcrumbs from "./ui/Breadcrumbs.svelte";
</script>

<Breadcrumbs />
```

### React (TSX)
```tsx
import Breadcrumbs from "./ui/Breadcrumbs";

export function Example() {
  return <Breadcrumbs />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Breadcrumbs from "./ui/Breadcrumbs.vue";
</script>

<template>
  <Breadcrumbs />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Breadcrumbs.dart';

// Inside build():
// Breadcrumbs(props)
```
