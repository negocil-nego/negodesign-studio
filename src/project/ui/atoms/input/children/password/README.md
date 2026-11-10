# InputPassword (`input-password`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputPassword.svelte`
- **React**: `ui/InputPassword.tsx`
- **Vue**: `ui/InputPassword.vue`
- **Flutter View**: `ui/InputPassword.dart`
- **Component Dependencies**: `input`, `input-label`

## Props (`InputPasswordProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `inputProps` | `InputProps` | Yes |
| `labelProps` | `LabelProps` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputPasswordViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputPassword from "./ui/InputPassword.svelte";
</script>

<InputPassword />
```

### React (TSX)
```tsx
import InputPassword from "./ui/InputPassword";

export function Example() {
  return <InputPassword />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputPassword from "./ui/InputPassword.vue";
</script>

<template>
  <InputPassword />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputPassword.dart';

// Inside build():
// InputPassword(props)
```
