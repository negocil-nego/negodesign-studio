# Toggle (`toggle`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Toggle.svelte`
- **React**: `ui/Toggle.tsx`
- **Vue**: `ui/Toggle.vue`
- **Flutter View**: `ui/Toggle.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`ToggleViewModel`)
- **Component Dependencies**: None (Standalone)

## Props (`ToggleProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `id` | `string` | Yes |
| `checked` | `boolean` | No |
| `disabled` | `boolean` | No |
| `label` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`ToggleViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Toggle from "./ui/Toggle.svelte";
</script>

<Toggle />
```

### React (TSX)
```tsx
import Toggle from "./ui/Toggle";

export function Example() {
  return <Toggle />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Toggle from "./ui/Toggle.vue";
</script>

<template>
  <Toggle />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Toggle.dart';

// Inside build():
// Toggle(props)
```
