# InputEmail (`input-email`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputEmail.svelte`
- **React**: `ui/InputEmail.tsx`
- **Vue**: `ui/InputEmail.vue`
- **Flutter View**: `ui/InputEmail.dart`
- **Component Dependencies**: `input`, `input-label`

## Props (`InputEmailProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `inputProps` | `InputProps` | Yes |
| `labelProps` | `LabelProps` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputEmailViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputEmail from "./ui/InputEmail.svelte";
</script>

<InputEmail />
```

### React (TSX)
```tsx
import InputEmail from "./ui/InputEmail";

export function Example() {
  return <InputEmail />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputEmail from "./ui/InputEmail.vue";
</script>

<template>
  <InputEmail />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputEmail.dart';

// Inside build():
// InputEmail(props)
```
