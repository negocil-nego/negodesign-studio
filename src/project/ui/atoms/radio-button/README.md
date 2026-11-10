# RadioButton (`radio-button`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/RadioButton.svelte`
- **React**: `ui/RadioButton.tsx`
- **Vue**: `ui/RadioButton.vue`
- **Flutter View**: `ui/RadioButton.dart`
- **Component Dependencies**: None (Standalone)

## Props (`RadioButtonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `options` | `RadioButtonOption[]` | Yes |
| `value` | `string` | No |
| `multiple` | `boolean` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`RadioButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import RadioButton from "./ui/RadioButton.svelte";
</script>

<RadioButton />
```

### React (TSX)
```tsx
import RadioButton from "./ui/RadioButton";

export function Example() {
  return <RadioButton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import RadioButton from "./ui/RadioButton.vue";
</script>

<template>
  <RadioButton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/RadioButton.dart';

// Inside build():
// RadioButton(props)
```
