# CarouselCardProduct (`carousel-card-product`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CarouselCardProduct.svelte`
- **React**: `ui/CarouselCardProduct.tsx`
- **Vue**: `ui/CarouselCardProduct.vue`
- **Flutter View**: `ui/CarouselCardProduct.dart`
- **Component Dependencies**: `card-product`, `section-carousel-buttons`

## Props (`CarouselCardProductProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `sectionCarouselButtonsProps` | `SectionCarouselButtonsProps` | No |
| `items` | `CardProductProps[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CarouselCardProductViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CarouselCardProduct from "./ui/CarouselCardProduct.svelte";
</script>

<CarouselCardProduct />
```

### React (TSX)
```tsx
import CarouselCardProduct from "./ui/CarouselCardProduct";

export function Example() {
  return <CarouselCardProduct />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CarouselCardProduct from "./ui/CarouselCardProduct.vue";
</script>

<template>
  <CarouselCardProduct />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CarouselCardProduct.dart';

// Inside build():
// CarouselCardProduct(props)
```
