# Radio (`radio`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Radio.svelte`
- **React**: `ui/Radio.tsx`
- **Vue**: `ui/Radio.vue`
- **Flutter View**: `ui/Radio.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`RadioViewModel`)
- **Component Dependencies**: None (Standalone)

## Props (`RadioProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `name` | `string` | Yes |
| `options` | `RadioOption[]` | Yes |
| `value` | `string` | No |
| `disabled` | `boolean` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`RadioViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Radio from "./ui/Radio.svelte";
</script>

<Radio />
```

### React (TSX)
```tsx
import Radio from "./ui/Radio";

export function Example() {
  return <Radio />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Radio from "./ui/Radio.vue";
</script>

<template>
  <Radio />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Radio.dart';

// Inside build():
// Radio(props)
```
