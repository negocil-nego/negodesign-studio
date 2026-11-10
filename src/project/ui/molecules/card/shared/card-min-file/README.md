# CardMinFile (`card-min-file`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardMinFile.svelte`
- **React**: `ui/CardMinFile.tsx`
- **Vue**: `ui/CardMinFile.vue`
- **Flutter View**: `ui/CardMinFile.dart`
- **Component Dependencies**: `card-image`

## Props (`CardMinFileProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardMinFileProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardMinFileViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardMinFile from "./ui/CardMinFile.svelte";
</script>

<CardMinFile />
```

### React (TSX)
```tsx
import CardMinFile from "./ui/CardMinFile";

export function Example() {
  return <CardMinFile />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardMinFile from "./ui/CardMinFile.vue";
</script>

<template>
  <CardMinFile />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardMinFile.dart';

// Inside build():
// CardMinFile(props)
```
