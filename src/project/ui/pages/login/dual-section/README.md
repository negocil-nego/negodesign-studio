# PageLoginDual (`page-login-dual`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/PageLoginDual.svelte`
- **React**: `ui/PageLoginDual.tsx`
- **Vue**: `ui/PageLoginDual.vue`
- **Flutter View**: `ui/PageLoginDual.dart`
- **Component Dependencies**: `button`, `dual-section`, `form`, `section-form`, `text`

## Props (`PageLoginDualProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `PageLoginDualProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`PageLoginDualViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import PageLoginDual from "./ui/PageLoginDual.svelte";
</script>

<PageLoginDual />
```

### React (TSX)
```tsx
import PageLoginDual from "./ui/PageLoginDual";

export function Example() {
  return <PageLoginDual />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import PageLoginDual from "./ui/PageLoginDual.vue";
</script>

<template>
  <PageLoginDual />
</template>
```

### Flutter (Dart)
```dart
import 'ui/PageLoginDual.dart';

// Inside build():
// PageLoginDual(props)
```
