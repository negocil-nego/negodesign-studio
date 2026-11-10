# DataTable (`data-table`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/DataTable.svelte`
- **React**: `ui/DataTable.tsx`
- **Vue**: `ui/DataTable.vue`
- **Flutter View**: `ui/DataTable.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`DataTableViewModel`)
- **Component Dependencies**: `data-table-filter`, `icon`, `text`

## Props (`DataTableProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `variant` | `"01" \| "02" \| "03" \| "04" \| "05"` | No |
| `title` | `string` | No |
| `columns` | `TextProps[]` | No |
| `rows` | `TextProps[]` | No |
| `className` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`DataTableViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import DataTable from "./ui/DataTable.svelte";
</script>

<DataTable />
```

### React (TSX)
```tsx
import DataTable from "./ui/DataTable";

export function Example() {
  return <DataTable />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import DataTable from "./ui/DataTable.vue";
</script>

<template>
  <DataTable />
</template>
```

### Flutter (Dart)
```dart
import 'ui/DataTable.dart';

// Inside build():
// DataTable(props)
```
