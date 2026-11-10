# InputNumber (`input-number`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputNumber.svelte`
- **React**: `ui/InputNumber.tsx`
- **Vue**: `ui/InputNumber.vue`
- **Flutter View**: `ui/InputNumber.dart`
- **Component Dependencies**: `input`, `input-label`

## Props (`InputNumberProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `inputProps` | `InputProps` | Yes |
| `labelProps` | `LabelProps` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputNumberViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputNumber from "./ui/InputNumber.svelte";
</script>

<InputNumber />
```

### React (TSX)
```tsx
import InputNumber from "./ui/InputNumber";

export function Example() {
  return <InputNumber />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputNumber from "./ui/InputNumber.vue";
</script>

<template>
  <InputNumber />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputNumber.dart';

// Inside build():
// InputNumber(props)
```
