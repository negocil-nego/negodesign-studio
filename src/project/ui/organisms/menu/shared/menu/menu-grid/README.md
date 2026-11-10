# MenuGridText (`menu-grid-text`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/MenuText.svelte`
- **React**: `ui/MenuText.tsx`
- **Vue**: `ui/MenuText.vue`
- **Flutter View**: `ui/MenuText.dart`
- **Component Dependencies**: `icon`, `menu-text`, `menu-title`, `text`

## Props (`MenuGridTextProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `MenuGridTextProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`MenuGridTextViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import MenuGridText from "./ui/MenuText.svelte";
</script>

<MenuGridText />
```

### React (TSX)
```tsx
import MenuGridText from "./ui/MenuText";

export function Example() {
  return <MenuGridText />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import MenuGridText from "./ui/MenuText.vue";
</script>

<template>
  <MenuGridText />
</template>
```

### Flutter (Dart)
```dart
import 'ui/MenuText.dart';

// Inside build():
// MenuGridText(props)
```
