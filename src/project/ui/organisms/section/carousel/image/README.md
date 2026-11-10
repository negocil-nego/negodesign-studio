# SectionCarousel (`section-carousel`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SectionCarousel.svelte`
- **React**: `ui/SectionCarousel.tsx`
- **Vue**: `ui/SectionCarousel.vue`
- **Flutter View**: `ui/SectionCarousel.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`SectionCarouselViewModel`)
- **Component Dependencies**: `text`

## Props (`SectionCarouselProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `items` | `SectionImageCarouselItem[]` | No |
| `className` | `string` | No |
| `backgroundColor` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SectionCarouselViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SectionCarousel from "./ui/SectionCarousel.svelte";
</script>

<SectionCarousel />
```

### React (TSX)
```tsx
import SectionCarousel from "./ui/SectionCarousel";

export function Example() {
  return <SectionCarousel />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SectionCarousel from "./ui/SectionCarousel.vue";
</script>

<template>
  <SectionCarousel />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SectionCarousel.dart';

// Inside build():
// SectionCarousel(props)
```
