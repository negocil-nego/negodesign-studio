# SectionGrid (`section-grid`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SectionGrid.svelte`
- **React**: `ui/SectionGrid.tsx`
- **Vue**: `ui/SectionGrid.vue`
- **Flutter View**: `ui/SectionGrid.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`SectionGridViewModel`)
- **Component Dependencies**: None (Standalone)

## Props (`SectionGridProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `titleProps` | `TextProps` | No |
| `descriptionProps` | `TextProps` | No |
| `children` | `ReactNode \| Snippet \| slot \| List<Widget>` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SectionGridViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SectionGrid from "./ui/SectionGrid.svelte";
</script>

<SectionGrid />
```

### React (TSX)
```tsx
import SectionGrid from "./ui/SectionGrid";

export function Example() {
  return <SectionGrid />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SectionGrid from "./ui/SectionGrid.vue";
</script>

<template>
  <SectionGrid />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SectionGrid.dart';

// Inside build():
// SectionGrid(props)
```
