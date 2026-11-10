# InputRange (`input-range`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputRange.svelte`
- **React**: `ui/InputRange.tsx`
- **Vue**: `ui/InputRange.vue`
- **Flutter View**: `ui/InputRange.dart`
- **Component Dependencies**: None (Standalone)

## Props (`InputRangeProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `label` | `string` | No |
| `value` | `number` | No |
| `min` | `number` | No |
| `max` | `number` | No |
| `step` | `number` | No |
| `disabled` | `boolean` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputRangeViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputRange from "./ui/InputRange.svelte";
</script>

<InputRange />
```

### React (TSX)
```tsx
import InputRange from "./ui/InputRange";

export function Example() {
  return <InputRange />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputRange from "./ui/InputRange.vue";
</script>

<template>
  <InputRange />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputRange.dart';

// Inside build():
// InputRange(props)
```
