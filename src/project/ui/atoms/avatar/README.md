# Avatar (`avatar`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Avatar.svelte`
- **React**: `ui/Avatar.tsx`
- **Vue**: `ui/Avatar.vue`
- **Flutter View**: `ui/Avatar.dart`
- **Component Dependencies**: `skeleton`

## Props (`AvatarProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `src` | `string` | No |
| `alt` | `string` | No |
| `isLoading` | `boolean` | No |
| `onClick` | `() => void` | No |
| `className` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`AvatarViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Avatar from "./ui/Avatar.svelte";
</script>

<Avatar />
```

### React (TSX)
```tsx
import Avatar from "./ui/Avatar";

export function Example() {
  return <Avatar />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Avatar from "./ui/Avatar.vue";
</script>

<template>
  <Avatar />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Avatar.dart';

// Inside build():
// Avatar(props)
```
