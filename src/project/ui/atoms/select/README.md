# Select (`select`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Select.svelte`
- **React**: `ui/Select.tsx`
- **Vue**: `ui/Select.vue`
- **Flutter View**: `ui/Select.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`SelectViewModel`)
- **Component Dependencies**: `icon`

## Props (`SelectProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `id` | `string` | Yes |
| `options` | `SelectOption[]` | Yes |
| `value` | `string` | No |
| `placeholder` | `string` | No |
| `disabled` | `boolean` | No |
| `required` | `boolean` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SelectViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Select from "./ui/Select.svelte";
</script>

<Select />
```

### React (TSX)
```tsx
import Select from "./ui/Select";

export function Example() {
  return <Select />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Select from "./ui/Select.vue";
</script>

<template>
  <Select />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Select.dart';

// Inside build():
// Select(props)
```
