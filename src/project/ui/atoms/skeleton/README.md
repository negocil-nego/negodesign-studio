# Skeleton (`skeleton`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Skeleton.svelte`
- **React**: `ui/Skeleton.tsx`
- **Vue**: `ui/Skeleton.vue`
- **Flutter View**: `ui/Skeleton.dart`
- **Component Dependencies**: None (Standalone)

## Props (`SkeletonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `className` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SkeletonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Skeleton from "./ui/Skeleton.svelte";
</script>

<Skeleton />
```

### React (TSX)
```tsx
import Skeleton from "./ui/Skeleton";

export function Example() {
  return <Skeleton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Skeleton from "./ui/Skeleton.vue";
</script>

<template>
  <Skeleton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Skeleton.dart';

// Inside build():
// Skeleton(props)
```
