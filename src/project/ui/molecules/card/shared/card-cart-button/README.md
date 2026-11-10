# CardCartButton (`card-cart-button`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardCartButton.svelte`
- **React**: `ui/CardCartButton.tsx`
- **Vue**: `ui/CardCartButton.vue`
- **Flutter View**: `ui/CardCartButton.dart`
- **Component Dependencies**: `button-icon`

## Props (`CardCartButtonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardCartButtonProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardCartButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardCartButton from "./ui/CardCartButton.svelte";
</script>

<CardCartButton />
```

### React (TSX)
```tsx
import CardCartButton from "./ui/CardCartButton";

export function Example() {
  return <CardCartButton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardCartButton from "./ui/CardCartButton.vue";
</script>

<template>
  <CardCartButton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardCartButton.dart';

// Inside build():
// CardCartButton(props)
```
