# CardProduct (`card-product`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardProduct.svelte`
- **React**: `ui/CardProduct.tsx`
- **Vue**: `ui/CardProduct.vue`
- **Flutter View**: `ui/CardProduct.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`CardProductViewModel`)
- **Component Dependencies**: `card-button`, `card-cart-button`, `card-description`, `card-favorite-button`, `card-image`, `card-logo`, `card-min-file`, `card-price`, `card-rating`, `card-thumbnail-video`, `card-title`, `card-video`

## Props (`CardProductProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardProductProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardProductViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardProduct from "./ui/CardProduct.svelte";
</script>

<CardProduct />
```

### React (TSX)
```tsx
import CardProduct from "./ui/CardProduct";

export function Example() {
  return <CardProduct />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardProduct from "./ui/CardProduct.vue";
</script>

<template>
  <CardProduct />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardProduct.dart';

// Inside build():
// CardProduct(props)
```
