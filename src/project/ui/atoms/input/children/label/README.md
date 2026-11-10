# InputLabel (`input-label`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputLabel.svelte`
- **React**: `ui/InputLabel.tsx`
- **Vue**: `ui/InputLabel.vue`
- **Flutter View**: `ui/InputLabel.dart`
- **Component Dependencies**: `input`, `label`

## Props (`InputLabelProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `labelProps` | `LabelProps` | Yes |
| `inputProps` | `InputProps` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputLabelViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputLabel from "./ui/InputLabel.svelte";
</script>

<InputLabel />
```

### React (TSX)
```tsx
import InputLabel from "./ui/InputLabel";

export function Example() {
  return <InputLabel />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputLabel from "./ui/InputLabel.vue";
</script>

<template>
  <InputLabel />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputLabel.dart';

// Inside build():
// InputLabel(props)
```
