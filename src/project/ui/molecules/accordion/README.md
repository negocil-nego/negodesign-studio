# Accordion (`accordion`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Accordion.svelte`
- **React**: `ui/Accordion.tsx`
- **Vue**: `ui/Accordion.vue`
- **Flutter View**: `ui/Accordion.dart`
- **Component Dependencies**: None (Standalone)

## Props (`AccordionProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `title` | `string` | Yes |
| `count` | `string` | No |
| `open` | `boolean` | No |
| `children` | `Snippet` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`AccordionViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Accordion from "./ui/Accordion.svelte";
</script>

<Accordion />
```

### React (TSX)
```tsx
import Accordion from "./ui/Accordion";

export function Example() {
  return <Accordion />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Accordion from "./ui/Accordion.vue";
</script>

<template>
  <Accordion />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Accordion.dart';

// Inside build():
// Accordion(props)
```
