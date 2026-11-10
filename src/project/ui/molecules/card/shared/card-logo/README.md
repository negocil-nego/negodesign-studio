# CardLogo (`card-logo`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardLogo.svelte`
- **React**: `ui/CardLogo.tsx`
- **Vue**: `ui/CardLogo.vue`
- **Flutter View**: `ui/CardLogo.dart`
- **Component Dependencies**: `avatar`

## Props (`CardLogoProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardLogoProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardLogoViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardLogo from "./ui/CardLogo.svelte";
</script>

<CardLogo />
```

### React (TSX)
```tsx
import CardLogo from "./ui/CardLogo";

export function Example() {
  return <CardLogo />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardLogo from "./ui/CardLogo.vue";
</script>

<template>
  <CardLogo />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardLogo.dart';

// Inside build():
// CardLogo(props)
```
