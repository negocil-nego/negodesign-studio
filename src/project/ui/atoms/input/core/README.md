# Input (`input`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Input.svelte`
- **React**: `ui/Input.tsx`
- **Vue**: `ui/Input.vue`
- **Flutter View**: `ui/Input.dart`
- **Component Dependencies**: None (Standalone)

## Props (`InputProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `id` | `string` | No |
| `icon` | `IconNamed \| string` | No |
| `type` | `InputType` | Yes |
| `required` | `boolean` | No |
| `disabled` | `boolean` | No |
| `placeholder` | `string` | No |
| `value` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Input from "./ui/Input.svelte";
</script>

<Input />
```

### React (TSX)
```tsx
import Input from "./ui/Input";

export function Example() {
  return <Input />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Input from "./ui/Input.vue";
</script>

<template>
  <Input />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Input.dart';

// Inside build():
// Input(props)
```
