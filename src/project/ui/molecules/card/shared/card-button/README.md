# CardButton (`card-button`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardButton.svelte`
- **React**: `ui/CardButton.tsx`
- **Vue**: `ui/CardButton.vue`
- **Flutter View**: `ui/CardButton.dart`
- **Component Dependencies**: `button`

## Props (`CardButtonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardButtonProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardButton from "./ui/CardButton.svelte";
</script>

<CardButton />
```

### React (TSX)
```tsx
import CardButton from "./ui/CardButton";

export function Example() {
  return <CardButton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardButton from "./ui/CardButton.vue";
</script>

<template>
  <CardButton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardButton.dart';

// Inside build():
// CardButton(props)
```
