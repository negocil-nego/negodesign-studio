# CardImage (`card-image`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardImage.svelte`
- **React**: `ui/CardImage.tsx`
- **Vue**: `ui/CardImage.vue`
- **Flutter View**: `ui/CardImage.dart`
- **Component Dependencies**: None (Standalone)

## Props (`CardImageProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardImageProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardImageViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardImage from "./ui/CardImage.svelte";
</script>

<CardImage />
```

### React (TSX)
```tsx
import CardImage from "./ui/CardImage";

export function Example() {
  return <CardImage />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardImage from "./ui/CardImage.vue";
</script>

<template>
  <CardImage />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardImage.dart';

// Inside build():
// CardImage(props)
```
