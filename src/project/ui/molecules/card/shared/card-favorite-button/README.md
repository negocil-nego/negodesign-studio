# CardFavoriteButton (`card-favorite-button`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardFavoriteButton.svelte`
- **React**: `ui/CardFavoriteButton.tsx`
- **Vue**: `ui/CardFavoriteButton.vue`
- **Flutter View**: `ui/CardFavoriteButton.dart`
- **Component Dependencies**: `button-icon`

## Props (`CardFavoriteButtonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardFavoriteButtonProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardFavoriteButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardFavoriteButton from "./ui/CardFavoriteButton.svelte";
</script>

<CardFavoriteButton />
```

### React (TSX)
```tsx
import CardFavoriteButton from "./ui/CardFavoriteButton";

export function Example() {
  return <CardFavoriteButton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardFavoriteButton from "./ui/CardFavoriteButton.vue";
</script>

<template>
  <CardFavoriteButton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardFavoriteButton.dart';

// Inside build():
// CardFavoriteButton(props)
```
