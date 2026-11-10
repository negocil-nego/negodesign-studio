# CardProfile (`card-profile`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardProfile.svelte`
- **React**: `ui/CardProfile.tsx`
- **Vue**: `ui/CardProfile.vue`
- **Flutter View**: `ui/CardProfile.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`CardProfileViewModel`)
- **Component Dependencies**: `card-actions-button`, `card-button`, `card-cart-button`, `card-description`, `card-favorite-button`, `card-image`, `card-logo`, `card-rating`, `card-tags`, `card-thumbnail-video`, `card-title`, `card-video`, `icon`

## Props (`CardProfileProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardProfileProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardProfileViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardProfile from "./ui/CardProfile.svelte";
</script>

<CardProfile />
```

### React (TSX)
```tsx
import CardProfile from "./ui/CardProfile";

export function Example() {
  return <CardProfile />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardProfile from "./ui/CardProfile.vue";
</script>

<template>
  <CardProfile />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardProfile.dart';

// Inside build():
// CardProfile(props)
```
