# InputDate (`input-date`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputDate.svelte`
- **React**: `ui/InputDate.tsx`
- **Vue**: `ui/InputDate.vue`
- **Flutter View**: `ui/InputDate.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`InputDateViewModel`)
- **Component Dependencies**: `icon`, `input`, `input-label`, `popover`

## Props (`InputDateProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `inputProps` | `InputProps` | Yes |
| `labelProps` | `LabelProps` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputDateViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputDate from "./ui/InputDate.svelte";
</script>

<InputDate />
```

### React (TSX)
```tsx
import InputDate from "./ui/InputDate";

export function Example() {
  return <InputDate />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputDate from "./ui/InputDate.vue";
</script>

<template>
  <InputDate />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputDate.dart';

// Inside build():
// InputDate(props)
```
