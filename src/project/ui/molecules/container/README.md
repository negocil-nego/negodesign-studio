# Containers (`containers`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Containers.svelte`
- **React**: `ui/Containers.tsx`
- **Vue**: `ui/Containers.vue`
- **Flutter View**: `ui/Containers.dart`
- **Component Dependencies**: None (Standalone)

## Props (`ContainersProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `title` | `string` | No |
| `lead` | `string` | No |
| `body` | `string` | No |
| `actionLabel` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`ContainersViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Containers from "./ui/Containers.svelte";
</script>

<Containers />
```

### React (TSX)
```tsx
import Containers from "./ui/Containers";

export function Example() {
  return <Containers />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Containers from "./ui/Containers.vue";
</script>

<template>
  <Containers />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Containers.dart';

// Inside build():
// Containers(props)
```
