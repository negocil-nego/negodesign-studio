# Label (`label`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Label.svelte`
- **React**: `ui/Label.tsx`
- **Vue**: `ui/Label.vue`
- **Flutter View**: `ui/Label.dart`
- **Component Dependencies**: None (Standalone)

## Props (`LabelProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `label` | `string` | Yes |
| `for` | `string` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`LabelViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Label from "./ui/Label.svelte";
</script>

<Label />
```

### React (TSX)
```tsx
import Label from "./ui/Label";

export function Example() {
  return <Label />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Label from "./ui/Label.vue";
</script>

<template>
  <Label />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Label.dart';

// Inside build():
// Label(props)
```
