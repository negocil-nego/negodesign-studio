# BlockButton (`block-button`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/BlockButton.svelte`
- **React**: `ui/BlockButton.tsx`
- **Vue**: `ui/BlockButton.vue`
- **Flutter View**: `ui/BlockButton.dart`
- **Component Dependencies**: `button-base`

## Props (`BlockButtonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `label` | `string` | Yes |
| `variant` | `'primary' \| 'secondary' \| 'outline'` | No |
| `disabled` | `boolean` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`BlockButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import BlockButton from "./ui/BlockButton.svelte";
</script>

<BlockButton />
```

### React (TSX)
```tsx
import BlockButton from "./ui/BlockButton";

export function Example() {
  return <BlockButton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import BlockButton from "./ui/BlockButton.vue";
</script>

<template>
  <BlockButton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/BlockButton.dart';

// Inside build():
// BlockButton(props)
```
