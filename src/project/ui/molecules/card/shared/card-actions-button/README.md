# CardActionsButton (`card-actions-button`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardActionsButton.svelte`
- **React**: `ui/CardActionsButton.tsx`
- **Vue**: `ui/CardActionsButton.vue`
- **Flutter View**: `ui/CardActionsButton.dart`
- **Component Dependencies**: `button-outline`, `icon`

## Props (`CardActionsButtonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardActionsButtonProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardActionsButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardActionsButton from "./ui/CardActionsButton.svelte";
</script>

<CardActionsButton />
```

### React (TSX)
```tsx
import CardActionsButton from "./ui/CardActionsButton";

export function Example() {
  return <CardActionsButton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardActionsButton from "./ui/CardActionsButton.vue";
</script>

<template>
  <CardActionsButton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardActionsButton.dart';

// Inside build():
// CardActionsButton(props)
```
