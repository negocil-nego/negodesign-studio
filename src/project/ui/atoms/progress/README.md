# Progress (`progress`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Progress.svelte`
- **React**: `ui/Progress.tsx`
- **Vue**: `ui/Progress.vue`
- **Flutter View**: `ui/Progress.dart`
- **Component Dependencies**: None (Standalone)

## Props (`ProgressProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `value` | `number` | No |
| `variant` | `'primary' \| 'success' \| 'info' \| 'warning' \| 'danger'` | No |
| `striped` | `boolean` | No |
| `animated` | `boolean` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`ProgressViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Progress from "./ui/Progress.svelte";
</script>

<Progress />
```

### React (TSX)
```tsx
import Progress from "./ui/Progress";

export function Example() {
  return <Progress />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Progress from "./ui/Progress.vue";
</script>

<template>
  <Progress />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Progress.dart';

// Inside build():
// Progress(props)
```
