# SubmenuTitle (`submenu-title`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SubmenuTitle.svelte`
- **React**: `ui/SubmenuTitle.tsx`
- **Vue**: `ui/SubmenuTitle.vue`
- **Flutter View**: `ui/SubmenuTitle.dart`
- **Component Dependencies**: `menu-grid-text`, `menu-text`, `menu-title`

## Props (`SubmenuTitleProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `SubmenuTitleProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SubmenuTitleViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SubmenuTitle from "./ui/SubmenuTitle.svelte";
</script>

<SubmenuTitle />
```

### React (TSX)
```tsx
import SubmenuTitle from "./ui/SubmenuTitle";

export function Example() {
  return <SubmenuTitle />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SubmenuTitle from "./ui/SubmenuTitle.vue";
</script>

<template>
  <SubmenuTitle />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SubmenuTitle.dart';

// Inside build():
// SubmenuTitle(props)
```
