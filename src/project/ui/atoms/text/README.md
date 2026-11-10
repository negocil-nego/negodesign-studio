# Text (`text`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Text.svelte`
- **React**: `ui/Text.tsx`
- **Vue**: `ui/Text.vue`
- **Flutter View**: `ui/Text.dart`
- **Component Dependencies**: None (Standalone)

## Props (`TextProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `text` | `string` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`TextViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Text from "./ui/Text.svelte";
</script>

<Text />
```

### React (TSX)
```tsx
import Text from "./ui/Text";

export function Example() {
  return <Text />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Text from "./ui/Text.vue";
</script>

<template>
  <Text />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Text.dart';

// Inside build():
// Text(props)
```
