# InputTime (`input-time`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputTime.svelte`
- **React**: `ui/InputTime.tsx`
- **Vue**: `ui/InputTime.vue`
- **Flutter View**: `ui/InputTime.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`InputTimeViewModel`)
- **Component Dependencies**: `icon`, `input`, `input-label`, `popover`

## Props (`InputTimeProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `inputProps` | `InputProps` | Yes |
| `labelProps` | `LabelProps` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputTimeViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputTime from "./ui/InputTime.svelte";
</script>

<InputTime />
```

### React (TSX)
```tsx
import InputTime from "./ui/InputTime";

export function Example() {
  return <InputTime />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputTime from "./ui/InputTime.vue";
</script>

<template>
  <InputTime />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputTime.dart';

// Inside build():
// InputTime(props)
```
