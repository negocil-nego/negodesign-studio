# InputUsernameOrEmail (`input-username-or-email`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputUsernameOrEmail.svelte`
- **React**: `ui/InputUsernameOrEmail.tsx`
- **Vue**: `ui/InputUsernameOrEmail.vue`
- **Flutter View**: `ui/InputUsernameOrEmail.dart`
- **Component Dependencies**: `input-username`

## Props (`InputUsernameOrEmailProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `labelProps` | `LabelProps` | No |
| `inputProps` | `InputProps` | Yes |
| `errorMessage` | `string` | No |
| `onValueChange` | `(value: string) => void` | No |
| `onValidityChange` | `(isValid: boolean) => void` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputUsernameOrEmailViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputUsernameOrEmail from "./ui/InputUsernameOrEmail.svelte";
</script>

<InputUsernameOrEmail />
```

### React (TSX)
```tsx
import InputUsernameOrEmail from "./ui/InputUsernameOrEmail";

export function Example() {
  return <InputUsernameOrEmail />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputUsernameOrEmail from "./ui/InputUsernameOrEmail.vue";
</script>

<template>
  <InputUsernameOrEmail />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputUsernameOrEmail.dart';

// Inside build():
// InputUsernameOrEmail(props)
```
