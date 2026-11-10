# Card (`card`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Card.svelte`
- **React**: `ui/Card.tsx`
- **Vue**: `ui/Card.vue`
- **Flutter View**: `ui/Card.dart`
- **Component Dependencies**: None (Standalone)

## Props (`CardProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Card from "./ui/Card.svelte";
</script>

<Card />
```

### React (TSX)
```tsx
import Card from "./ui/Card";

export function Example() {
  return <Card />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Card from "./ui/Card.vue";
</script>

<template>
  <Card />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Card.dart';

// Inside build():
// Card(props)
```
