# CardRating (`card-rating`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardRating.svelte`
- **React**: `ui/CardRating.tsx`
- **Vue**: `ui/CardRating.vue`
- **Flutter View**: `ui/CardRating.dart`
- **Component Dependencies**: `icon`

## Props (`CardRatingProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardRatingProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardRatingViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardRating from "./ui/CardRating.svelte";
</script>

<CardRating />
```

### React (TSX)
```tsx
import CardRating from "./ui/CardRating";

export function Example() {
  return <CardRating />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardRating from "./ui/CardRating.vue";
</script>

<template>
  <CardRating />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardRating.dart';

// Inside build():
// CardRating(props)
```
