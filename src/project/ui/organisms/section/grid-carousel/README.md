# SectionGridCarousel (`section-grid-carousel`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SectionGridCarousel.svelte`
- **React**: `ui/SectionGridCarousel.tsx`
- **Vue**: `ui/SectionGridCarousel.vue`
- **Flutter View**: `ui/SectionGridCarousel.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`SectionGridCarouselViewModel`)
- **Component Dependencies**: `icon`

## Props (`SectionGridCarouselProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `titleProps` | `TextProps` | No |
| `descriptionProps` | `TextProps` | No |
| `rows` | `number` | No |
| `columns` | `number` | No |
| `children` | `ReactNode[] \| Snippet \| slot \| List<Widget>` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SectionGridCarouselViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SectionGridCarousel from "./ui/SectionGridCarousel.svelte";
</script>

<SectionGridCarousel />
```

### React (TSX)
```tsx
import SectionGridCarousel from "./ui/SectionGridCarousel";

export function Example() {
  return <SectionGridCarousel />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SectionGridCarousel from "./ui/SectionGridCarousel.vue";
</script>

<template>
  <SectionGridCarousel />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SectionGridCarousel.dart';

// Inside build():
// SectionGridCarousel(props)
```
