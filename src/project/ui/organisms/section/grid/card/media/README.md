# GridCardMedia (`grid-card-media`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/GridCardMedia.svelte`
- **React**: `ui/GridCardMedia.tsx`
- **Vue**: `ui/GridCardMedia.vue`
- **Flutter View**: `ui/GridCardMedia.dart`
- **Component Dependencies**: `card-media`, `section-grid`

## Props (`GridCardMediaProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `sectionGridProps` | `SectionGridProps` | No |
| `items` | `CardMediaProps[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`GridCardMediaViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import GridCardMedia from "./ui/GridCardMedia.svelte";
</script>

<GridCardMedia />
```

### React (TSX)
```tsx
import GridCardMedia from "./ui/GridCardMedia";

export function Example() {
  return <GridCardMedia />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import GridCardMedia from "./ui/GridCardMedia.vue";
</script>

<template>
  <GridCardMedia />
</template>
```

### Flutter (Dart)
```dart
import 'ui/GridCardMedia.dart';

// Inside build():
// GridCardMedia(props)
```
