# GridCardProduct (`grid-card-product`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/GridCardProduct.svelte`
- **React**: `ui/GridCardProduct.tsx`
- **Vue**: `ui/GridCardProduct.vue`
- **Flutter View**: `ui/GridCardProduct.dart`
- **Component Dependencies**: `card-product`, `section-grid`

## Props (`GridCardProductProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `sectionGridProps` | `SectionGridProps` | No |
| `items` | `CardProductProps[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`GridCardProductViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import GridCardProduct from "./ui/GridCardProduct.svelte";
</script>

<GridCardProduct />
```

### React (TSX)
```tsx
import GridCardProduct from "./ui/GridCardProduct";

export function Example() {
  return <GridCardProduct />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import GridCardProduct from "./ui/GridCardProduct.vue";
</script>

<template>
  <GridCardProduct />
</template>
```

### Flutter (Dart)
```dart
import 'ui/GridCardProduct.dart';

// Inside build():
// GridCardProduct(props)
```
