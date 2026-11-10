# MenuTitle (`menu-title`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/MenuTitle.svelte`
- **React**: `ui/MenuTitle.tsx`
- **Vue**: `ui/MenuTitle.vue`
- **Flutter View**: `ui/MenuTitle.dart`
- **Component Dependencies**: `text`

## Props (`MenuTitleProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `MenuTitleProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`MenuTitleViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import MenuTitle from "./ui/MenuTitle.svelte";
</script>

<MenuTitle />
```

### React (TSX)
```tsx
import MenuTitle from "./ui/MenuTitle";

export function Example() {
  return <MenuTitle />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import MenuTitle from "./ui/MenuTitle.vue";
</script>

<template>
  <MenuTitle />
</template>
```

### Flutter (Dart)
```dart
import 'ui/MenuTitle.dart';

// Inside build():
// MenuTitle(props)
```
