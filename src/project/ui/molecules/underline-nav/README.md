# UnderlineNav (`underline-nav`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/UnderlineNav.svelte`
- **React**: `ui/UnderlineNav.tsx`
- **Vue**: `ui/UnderlineNav.vue`
- **Flutter View**: `ui/UnderlineNav.dart`
- **Component Dependencies**: None (Standalone)

## Props (`UnderlineNavProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `items` | `UnderlineNavItem[]` | Yes |
| `value` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`UnderlineNavViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import UnderlineNav from "./ui/UnderlineNav.svelte";
</script>

<UnderlineNav />
```

### React (TSX)
```tsx
import UnderlineNav from "./ui/UnderlineNav";

export function Example() {
  return <UnderlineNav />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import UnderlineNav from "./ui/UnderlineNav.vue";
</script>

<template>
  <UnderlineNav />
</template>
```

### Flutter (Dart)
```dart
import 'ui/UnderlineNav.dart';

// Inside build():
// UnderlineNav(props)
```
