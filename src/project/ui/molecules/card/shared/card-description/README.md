# CardDescription (`card-description`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardDescription.svelte`
- **React**: `ui/CardDescription.tsx`
- **Vue**: `ui/CardDescription.vue`
- **Flutter View**: `ui/CardDescription.dart`
- **Component Dependencies**: None (Standalone)

## Props (`CardDescriptionProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardDescriptionProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardDescriptionViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardDescription from "./ui/CardDescription.svelte";
</script>

<CardDescription />
```

### React (TSX)
```tsx
import CardDescription from "./ui/CardDescription";

export function Example() {
  return <CardDescription />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardDescription from "./ui/CardDescription.vue";
</script>

<template>
  <CardDescription />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardDescription.dart';

// Inside build():
// CardDescription(props)
```
