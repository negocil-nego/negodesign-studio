# GridCarouselCardProfile (`grid-carousel-card-profile`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/GridCarouselCardProfile.svelte`
- **React**: `ui/GridCarouselCardProfile.tsx`
- **Vue**: `ui/GridCarouselCardProfile.vue`
- **Flutter View**: `ui/GridCarouselCardProfile.dart`
- **Component Dependencies**: `card-profile`, `section-grid-carousel`

## Props (`GridCarouselCardProfileProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `sectionGridCarouselProps` | `SectionGridCarouselProps` | No |
| `items` | `CardProfileProps[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`GridCarouselCardProfileViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import GridCarouselCardProfile from "./ui/GridCarouselCardProfile.svelte";
</script>

<GridCarouselCardProfile />
```

### React (TSX)
```tsx
import GridCarouselCardProfile from "./ui/GridCarouselCardProfile";

export function Example() {
  return <GridCarouselCardProfile />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import GridCarouselCardProfile from "./ui/GridCarouselCardProfile.vue";
</script>

<template>
  <GridCarouselCardProfile />
</template>
```

### Flutter (Dart)
```dart
import 'ui/GridCarouselCardProfile.dart';

// Inside build():
// GridCarouselCardProfile(props)
```
