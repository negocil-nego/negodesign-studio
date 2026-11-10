# Popover (`popover`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Popover.svelte`
- **React**: `ui/Popover.tsx`
- **Vue**: `ui/Popover.vue`
- **Flutter View**: `ui/Popover.dart`
- **Component Dependencies**: None (Standalone)

## Props (`PopoverProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `id` | `string` | Yes |
| `trigger` | `Snippet` | Yes |
| `panel` | `Snippet` | Yes |
| `role` | `string` | No |
| `ariaLabel` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`PopoverViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Popover from "./ui/Popover.svelte";
</script>

<Popover />
```

### React (TSX)
```tsx
import Popover from "./ui/Popover";

export function Example() {
  return <Popover />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Popover from "./ui/Popover.vue";
</script>

<template>
  <Popover />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Popover.dart';

// Inside build():
// Popover(props)
```
