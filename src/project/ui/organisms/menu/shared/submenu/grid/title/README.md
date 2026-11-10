# SubmenuGridTitle (`submenu-grid-title`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SubmenuGridTitle.svelte`
- **React**: `ui/SubmenuGridTitle.tsx`
- **Vue**: `ui/SubmenuGridTitle.vue`
- **Flutter View**: `ui/SubmenuGridTitle.dart`
- **Component Dependencies**: `menu-title`, `submenu-title`

## Props (`SubmenuGridTitleProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `SubmenuGridTitleProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SubmenuGridTitleViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SubmenuGridTitle from "./ui/SubmenuGridTitle.svelte";
</script>

<SubmenuGridTitle />
```

### React (TSX)
```tsx
import SubmenuGridTitle from "./ui/SubmenuGridTitle";

export function Example() {
  return <SubmenuGridTitle />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SubmenuGridTitle from "./ui/SubmenuGridTitle.vue";
</script>

<template>
  <SubmenuGridTitle />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SubmenuGridTitle.dart';

// Inside build():
// SubmenuGridTitle(props)
```
