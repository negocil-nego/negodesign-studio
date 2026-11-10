# InputText (`input-text`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputText.svelte`
- **React**: `ui/InputText.tsx`
- **Vue**: `ui/InputText.vue`
- **Flutter View**: `ui/InputText.dart`
- **Component Dependencies**: `input`, `input-label`

## Props (`InputTextProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `inputProps` | `InputProps` | Yes |
| `labelProps` | `LabelProps` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputTextViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputText from "./ui/InputText.svelte";
</script>

<InputText />
```

### React (TSX)
```tsx
import InputText from "./ui/InputText";

export function Example() {
  return <InputText />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputText from "./ui/InputText.vue";
</script>

<template>
  <InputText />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputText.dart';

// Inside build():
// InputText(props)
```
