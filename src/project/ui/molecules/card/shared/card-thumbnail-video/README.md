# CardThumbnailVideo (`card-thumbnail-video`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardThumbnailVideo.svelte`
- **React**: `ui/CardThumbnailVideo.tsx`
- **Vue**: `ui/CardThumbnailVideo.vue`
- **Flutter View**: `ui/CardThumbnailVideo.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`CardThumbnailVideoViewModel`)
- **Component Dependencies**: `card-image`, `card-video`, `icon`

## Props (`CardThumbnailVideoProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardThumbnailVideoProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardThumbnailVideoViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardThumbnailVideo from "./ui/CardThumbnailVideo.svelte";
</script>

<CardThumbnailVideo />
```

### React (TSX)
```tsx
import CardThumbnailVideo from "./ui/CardThumbnailVideo";

export function Example() {
  return <CardThumbnailVideo />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardThumbnailVideo from "./ui/CardThumbnailVideo.vue";
</script>

<template>
  <CardThumbnailVideo />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardThumbnailVideo.dart';

// Inside build():
// CardThumbnailVideo(props)
```
