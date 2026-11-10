# GridCarouselCardMedia (`grid-carousel-card-media`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/GridCarouselCardMedia.svelte`
- **React**: `ui/GridCarouselCardMedia.tsx`
- **Vue**: `ui/GridCarouselCardMedia.vue`
- **Flutter View**: `ui/GridCarouselCardMedia.dart`
- **Component Dependencies**: `card-media`, `section-grid-carousel`

## Props (`GridCarouselCardMediaProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `sectionGridCarouselProps` | `SectionGridCarouselProps` | No |
| `items` | `CardMediaProps[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`GridCarouselCardMediaViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import GridCarouselCardMedia from "./ui/GridCarouselCardMedia.svelte";
</script>

<GridCarouselCardMedia />
```

### React (TSX)
```tsx
import GridCarouselCardMedia from "./ui/GridCarouselCardMedia";

export function Example() {
  return <GridCarouselCardMedia />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import GridCarouselCardMedia from "./ui/GridCarouselCardMedia.vue";
</script>

<template>
  <GridCarouselCardMedia />
</template>
```

### Flutter (Dart)
```dart
import 'ui/GridCarouselCardMedia.dart';

// Inside build():
// GridCarouselCardMedia(props)
```
