# Checkbox (`checkbox`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Checkbox.svelte`
- **React**: `ui/Checkbox.tsx`
- **Vue**: `ui/Checkbox.vue`
- **Flutter View**: `ui/Checkbox.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`CheckboxViewModel`)
- **Component Dependencies**: None (Standalone)

## Props (`CheckboxProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `id` | `string` | Yes |
| `checked` | `boolean` | No |
| `label` | `string` | No |
| `disabled` | `boolean` | No |
| `required` | `boolean` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CheckboxViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Checkbox from "./ui/Checkbox.svelte";
</script>

<Checkbox />
```

### React (TSX)
```tsx
import Checkbox from "./ui/Checkbox";

export function Example() {
  return <Checkbox />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Checkbox from "./ui/Checkbox.vue";
</script>

<template>
  <Checkbox />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Checkbox.dart';

// Inside build():
// Checkbox(props)
```
