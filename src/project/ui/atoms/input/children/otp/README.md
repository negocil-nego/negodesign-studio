# InputOtp (`input-otp`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputOtp.svelte`
- **React**: `ui/InputOtp.tsx`
- **Vue**: `ui/InputOtp.vue`
- **Flutter View**: `ui/InputOtp.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`InputOtpViewModel`)
- **Component Dependencies**: None (Standalone)

## Props (`InputOtpProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `id` | `string` | No |
| `name` | `string` | No |
| `value` | `string` | No |
| `length` | `number` | No |
| `separatorEvery` | `number` | No |
| `labelProps` | `InputOtpLabelProps` | No |
| `disabled` | `boolean` | No |
| `required` | `boolean` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputOtpViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputOtp from "./ui/InputOtp.svelte";
</script>

<InputOtp />
```

### React (TSX)
```tsx
import InputOtp from "./ui/InputOtp";

export function Example() {
  return <InputOtp />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputOtp from "./ui/InputOtp.vue";
</script>

<template>
  <InputOtp />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputOtp.dart';

// Inside build():
// InputOtp(props)
```
