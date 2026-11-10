# InputAge (`input-age`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputAge.svelte`
- **React**: `ui/InputAge.tsx`
- **Vue**: `ui/InputAge.vue`
- **Flutter View**: `ui/InputAge.dart`
- **Component Dependencies**: `input`, `input-label`

## Props (`InputAgeProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `inputProps` | `InputProps` | Yes |
| `labelProps` | `LabelProps` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputAgeViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputAge from "./ui/InputAge.svelte";
</script>

<InputAge />
```

### React (TSX)
```tsx
import InputAge from "./ui/InputAge";

export function Example() {
  return <InputAge />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputAge from "./ui/InputAge.vue";
</script>

<template>
  <InputAge />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputAge.dart';

// Inside build():
// InputAge(props)
```
