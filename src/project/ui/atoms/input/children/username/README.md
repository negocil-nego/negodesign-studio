# InputUsername (`input-username`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputUsername.svelte`
- **React**: `ui/InputUsername.tsx`
- **Vue**: `ui/InputUsername.vue`
- **Flutter View**: `ui/InputUsername.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`InputUsernameViewModel`)
- **Component Dependencies**: `input-label`

## Props (`InputUsernameProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `labelProps` | `LabelProps` | No |
| `inputProps` | `InputProps` | Yes |
| `validationMode` | `UsernameValidationMode` | No |
| `errorMessage` | `string` | No |
| `onValueChange` | `(value: string) => void` | No |
| `onValidityChange` | `(isValid: boolean) => void` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputUsernameViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputUsername from "./ui/InputUsername.svelte";
</script>

<InputUsername />
```

### React (TSX)
```tsx
import InputUsername from "./ui/InputUsername";

export function Example() {
  return <InputUsername />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputUsername from "./ui/InputUsername.vue";
</script>

<template>
  <InputUsername />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputUsername.dart';

// Inside build():
// InputUsername(props)
```
