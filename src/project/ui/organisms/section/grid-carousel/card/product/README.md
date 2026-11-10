# GridCarouselCardProduct (`grid-carousel-card-product`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/GridCarouselCardProduct.svelte`
- **React**: `ui/GridCarouselCardProduct.tsx`
- **Vue**: `ui/GridCarouselCardProduct.vue`
- **Flutter View**: `ui/GridCarouselCardProduct.dart`
- **Component Dependencies**: `card-product`, `section-grid-carousel`

## Props (`GridCarouselCardProductProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `sectionGridCarouselProps` | `SectionGridCarouselProps` | No |
| `items` | `CardProductProps[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`GridCarouselCardProductViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import GridCarouselCardProduct from "./ui/GridCarouselCardProduct.svelte";
</script>

<GridCarouselCardProduct />
```

### React (TSX)
```tsx
import GridCarouselCardProduct from "./ui/GridCarouselCardProduct";

export function Example() {
  return <GridCarouselCardProduct />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import GridCarouselCardProduct from "./ui/GridCarouselCardProduct.vue";
</script>

<template>
  <GridCarouselCardProduct />
</template>
```

### Flutter (Dart)
```dart
import 'ui/GridCarouselCardProduct.dart';

// Inside build():
// GridCarouselCardProduct(props)
```
