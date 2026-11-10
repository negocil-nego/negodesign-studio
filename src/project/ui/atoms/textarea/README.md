# Textarea (`textarea`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Textarea.svelte`
- **React**: `ui/Textarea.tsx`
- **Vue**: `ui/Textarea.vue`
- **Flutter View**: `ui/Textarea.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`TextareaViewModel`)
- **Component Dependencies**: None (Standalone)

## Props (`TextareaProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `id` | `string` | Yes |
| `value` | `string` | No |
| `placeholder` | `string` | No |
| `rows` | `number` | No |
| `disabled` | `boolean` | No |
| `required` | `boolean` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`TextareaViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Textarea from "./ui/Textarea.svelte";
</script>

<Textarea />
```

### React (TSX)
```tsx
import Textarea from "./ui/Textarea";

export function Example() {
  return <Textarea />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Textarea from "./ui/Textarea.vue";
</script>

<template>
  <Textarea />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Textarea.dart';

// Inside build():
// Textarea(props)
```
