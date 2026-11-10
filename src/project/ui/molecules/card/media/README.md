# CardMedia (`card-media`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardMedia.svelte`
- **React**: `ui/CardMedia.tsx`
- **Vue**: `ui/CardMedia.vue`
- **Flutter View**: `ui/CardMedia.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`CardMediaViewModel`)
- **Component Dependencies**: `card-button`, `card-cart-button`, `card-description`, `card-favorite-button`, `card-image`, `card-logo`, `card-price`, `card-rating`, `card-thumbnail-video`, `card-title`, `card-video`

## Props (`CardMediaProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardMediaProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardMediaViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardMedia from "./ui/CardMedia.svelte";
</script>

<CardMedia />
```

### React (TSX)
```tsx
import CardMedia from "./ui/CardMedia";

export function Example() {
  return <CardMedia />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardMedia from "./ui/CardMedia.vue";
</script>

<template>
  <CardMedia />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardMedia.dart';

// Inside build():
// CardMedia(props)
```
