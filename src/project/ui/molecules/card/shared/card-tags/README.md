# CardTags (`card-tags`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardTags.svelte`
- **React**: `ui/CardTags.tsx`
- **Vue**: `ui/CardTags.vue`
- **Flutter View**: `ui/CardTags.dart`
- **Component Dependencies**: `icon`

## Props (`CardTagsProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `variant` | `any` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardTagsViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardTags from "./ui/CardTags.svelte";
</script>

<CardTags />
```

### React (TSX)
```tsx
import CardTags from "./ui/CardTags";

export function Example() {
  return <CardTags />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardTags from "./ui/CardTags.vue";
</script>

<template>
  <CardTags />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardTags.dart';

// Inside build():
// CardTags(props)
```
