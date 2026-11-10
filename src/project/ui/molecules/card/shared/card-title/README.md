# CardTitle (`card-title`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardTitle.svelte`
- **React**: `ui/CardTitle.tsx`
- **Vue**: `ui/CardTitle.vue`
- **Flutter View**: `ui/CardTitle.dart`
- **Component Dependencies**: None (Standalone)

## Props (`CardTitleProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardTitleProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardTitleViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardTitle from "./ui/CardTitle.svelte";
</script>

<CardTitle />
```

### React (TSX)
```tsx
import CardTitle from "./ui/CardTitle";

export function Example() {
  return <CardTitle />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardTitle from "./ui/CardTitle.vue";
</script>

<template>
  <CardTitle />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardTitle.dart';

// Inside build():
// CardTitle(props)
```
