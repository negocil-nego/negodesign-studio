# DataTableFilter (`data-table-filter`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/DataTableFilter.svelte`
- **React**: `ui/DataTableFilter.tsx`
- **Vue**: `ui/DataTableFilter.vue`
- **Flutter View**: `ui/DataTableFilter.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`DataTableFilterViewModel`)
- **Component Dependencies**: `icon`

## Props (`DataTableFilterProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `DataTableFilterProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`DataTableFilterViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import DataTableFilter from "./ui/DataTableFilter.svelte";
</script>

<DataTableFilter />
```

### React (TSX)
```tsx
import DataTableFilter from "./ui/DataTableFilter";

export function Example() {
  return <DataTableFilter />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import DataTableFilter from "./ui/DataTableFilter.vue";
</script>

<template>
  <DataTableFilter />
</template>
```

### Flutter (Dart)
```dart
import 'ui/DataTableFilter.dart';

// Inside build():
// DataTableFilter(props)
```
