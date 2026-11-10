# CarouselCardMedia (`carousel-card-media`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CarouselCardMedia.svelte`
- **React**: `ui/CarouselCardMedia.tsx`
- **Vue**: `ui/CarouselCardMedia.vue`
- **Flutter View**: `ui/CarouselCardMedia.dart`
- **Component Dependencies**: `card-media`, `section-carousel-buttons`

## Props (`CarouselCardMediaProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `sectionCarouselButtonsProps` | `SectionCarouselButtonsProps` | No |
| `items` | `CardMediaProps[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CarouselCardMediaViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CarouselCardMedia from "./ui/CarouselCardMedia.svelte";
</script>

<CarouselCardMedia />
```

### React (TSX)
```tsx
import CarouselCardMedia from "./ui/CarouselCardMedia";

export function Example() {
  return <CarouselCardMedia />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CarouselCardMedia from "./ui/CarouselCardMedia.vue";
</script>

<template>
  <CarouselCardMedia />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CarouselCardMedia.dart';

// Inside build():
// CarouselCardMedia(props)
```
