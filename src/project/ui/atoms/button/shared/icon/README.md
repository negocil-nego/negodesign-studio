# IconButton (`button-icon`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/IconButton.svelte`
- **React**: `ui/IconButton.tsx`
- **Vue**: `ui/IconButton.vue`
- **Flutter View**: `ui/IconButton.dart`
- **Component Dependencies**: `button`, `icon`

## Props (`IconButtonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `className` | `string` | No |
| `icon` | `IconProps` | Yes |
| `isLoading` | `boolean` | No |
| `onClick` | `() => void` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`IconButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import IconButton from "./ui/IconButton.svelte";
</script>

<IconButton />
```

### React (TSX)
```tsx
import IconButton from "./ui/IconButton";

export function Example() {
  return <IconButton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import IconButton from "./ui/IconButton.vue";
</script>

<template>
  <IconButton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/IconButton.dart';

// Inside build():
// IconButton(props)
```
