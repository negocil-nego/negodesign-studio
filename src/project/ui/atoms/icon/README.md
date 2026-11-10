# Icon (`icon`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Icon.svelte`
- **React**: `ui/Icon.tsx`
- **Vue**: `ui/Icon.vue`
- **Flutter View**: `ui/Icon.dart`
- **Component Dependencies**: None (Standalone)

## Props (`IconProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `name` | `IconName` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`IconViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Icon from "./ui/Icon.svelte";
</script>

<Icon />
```

### React (TSX)
```tsx
import Icon from "./ui/Icon";

export function Example() {
  return <Icon />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Icon from "./ui/Icon.vue";
</script>

<template>
  <Icon />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Icon.dart';

// Inside build():
// Icon(props)
```
