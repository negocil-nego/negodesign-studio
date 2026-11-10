# GridCardProfile (`grid-card-profile`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/GridCardProfile.svelte`
- **React**: `ui/GridCardProfile.tsx`
- **Vue**: `ui/GridCardProfile.vue`
- **Flutter View**: `ui/GridCardProfile.dart`
- **Component Dependencies**: `card-profile`, `section-grid`

## Props (`GridCardProfileProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `sectionGridProps` | `SectionGridProps` | No |
| `items` | `CardProfileProps[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`GridCardProfileViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import GridCardProfile from "./ui/GridCardProfile.svelte";
</script>

<GridCardProfile />
```

### React (TSX)
```tsx
import GridCardProfile from "./ui/GridCardProfile";

export function Example() {
  return <GridCardProfile />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import GridCardProfile from "./ui/GridCardProfile.vue";
</script>

<template>
  <GridCardProfile />
</template>
```

### Flutter (Dart)
```dart
import 'ui/GridCardProfile.dart';

// Inside build():
// GridCardProfile(props)
```
