# CardPrice (`card-price`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardPrice.svelte`
- **React**: `ui/CardPrice.tsx`
- **Vue**: `ui/CardPrice.vue`
- **Flutter View**: `ui/CardPrice.dart`
- **Component Dependencies**: None (Standalone)

## Props (`CardPriceProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardPriceProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardPriceViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardPrice from "./ui/CardPrice.svelte";
</script>

<CardPrice />
```

### React (TSX)
```tsx
import CardPrice from "./ui/CardPrice";

export function Example() {
  return <CardPrice />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardPrice from "./ui/CardPrice.vue";
</script>

<template>
  <CardPrice />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardPrice.dart';

// Inside build():
// CardPrice(props)
```
