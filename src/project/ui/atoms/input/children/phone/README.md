# InputPhone (`input-phone`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputPhone.svelte`
- **React**: `ui/InputPhone.tsx`
- **Vue**: `ui/InputPhone.vue`
- **Flutter View**: `ui/InputPhone.dart`
- **Component Dependencies**: `input`, `input-label`

## Props (`InputPhoneProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `inputProps` | `InputProps` | Yes |
| `labelProps` | `LabelProps` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputPhoneViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputPhone from "./ui/InputPhone.svelte";
</script>

<InputPhone />
```

### React (TSX)
```tsx
import InputPhone from "./ui/InputPhone";

export function Example() {
  return <InputPhone />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputPhone from "./ui/InputPhone.vue";
</script>

<template>
  <InputPhone />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputPhone.dart';

// Inside build():
// InputPhone(props)
```
