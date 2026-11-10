# MenuSimple (`menu-simple`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Menu.svelte`
- **React**: `ui/Menu.tsx`
- **Vue**: `ui/Menu.vue`
- **Flutter View**: `ui/Menu.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`MenuSimpleViewModel`)
- **Component Dependencies**: `button`, `icon`, `image`, `menu-text`, `submenu-grid-title`, `submenu-sidebar-grid`

## Props (`MenuSimpleProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `logo` | `ImageProps` | No |
| `menus` | `(MenuTextProps \| SubmenuGridTitleProps \| SubmenuSidebarGridProps)[]` | No |
| `buttonsProps` | `ButtonProps[]` | No |
| `defaultOpenSubmenu` | `number \| null` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`MenuSimpleViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import MenuSimple from "./ui/Menu.svelte";
</script>

<MenuSimple />
```

### React (TSX)
```tsx
import MenuSimple from "./ui/Menu";

export function Example() {
  return <MenuSimple />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import MenuSimple from "./ui/Menu.vue";
</script>

<template>
  <MenuSimple />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Menu.dart';

// Inside build():
// MenuSimple(props)
```
