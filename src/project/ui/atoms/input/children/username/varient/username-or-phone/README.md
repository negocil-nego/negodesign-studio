# InputUsernameOrPhone (`input-username-or-phone`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputUsernameOrPhone.svelte`
- **React**: `ui/InputUsernameOrPhone.tsx`
- **Vue**: `ui/InputUsernameOrPhone.vue`
- **Flutter View**: `ui/InputUsernameOrPhone.dart`
- **Component Dependencies**: `input-username`

## Props (`InputUsernameOrPhoneProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `labelProps` | `LabelProps` | No |
| `inputProps` | `InputProps` | Yes |
| `errorMessage` | `string` | No |
| `onValueChange` | `(value: string) => void` | No |
| `onValidityChange` | `(isValid: boolean) => void` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputUsernameOrPhoneViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputUsernameOrPhone from "./ui/InputUsernameOrPhone.svelte";
</script>

<InputUsernameOrPhone />
```

### React (TSX)
```tsx
import InputUsernameOrPhone from "./ui/InputUsernameOrPhone";

export function Example() {
  return <InputUsernameOrPhone />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputUsernameOrPhone from "./ui/InputUsernameOrPhone.vue";
</script>

<template>
  <InputUsernameOrPhone />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputUsernameOrPhone.dart';

// Inside build():
// InputUsernameOrPhone(props)
```
