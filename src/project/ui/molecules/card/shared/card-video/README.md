# CardVideo (`card-video`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardVideo.svelte`
- **React**: `ui/CardVideo.tsx`
- **Vue**: `ui/CardVideo.vue`
- **Flutter View**: `ui/CardVideo.dart`
- **Component Dependencies**: None (Standalone)

## Props (`CardVideoProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardVideoProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardVideoViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardVideo from "./ui/CardVideo.svelte";
</script>

<CardVideo />
```

### React (TSX)
```tsx
import CardVideo from "./ui/CardVideo";

export function Example() {
  return <CardVideo />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardVideo from "./ui/CardVideo.vue";
</script>

<template>
  <CardVideo />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardVideo.dart';

// Inside build():
// CardVideo(props)
```
