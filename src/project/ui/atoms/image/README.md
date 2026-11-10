# Image (`image`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Image.svelte`
- **React**: `ui/Image.tsx`
- **Vue**: `ui/Image.vue`
- **Flutter View**: `ui/Image.dart`
- **Component Dependencies**: `skeleton`

## Props (`ImageProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `src` | `string` | No |
| `alt` | `string` | No |
| `isLoading` | `boolean` | No |
| `onClick` | `() => void` | No |
| `className` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`ImageViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Image from "./ui/Image.svelte";
</script>

<Image />
```

### React (TSX)
```tsx
import Image from "./ui/Image";

export function Example() {
  return <Image />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Image from "./ui/Image.vue";
</script>

<template>
  <Image />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Image.dart';

// Inside build():
// Image(props)
```
