# PageRegisterDual (`page-register-dual`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/PageRegisterDual.svelte`
- **React**: `ui/PageRegisterDual.tsx`
- **Vue**: `ui/PageRegisterDual.vue`
- **Flutter View**: `ui/PageRegisterDual.dart`
- **Component Dependencies**: `button`, `dual-section`, `form`, `section-form`, `text`

## Props (`PageRegisterDualProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `PageRegisterDualProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`PageRegisterDualViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import PageRegisterDual from "./ui/PageRegisterDual.svelte";
</script>

<PageRegisterDual />
```

### React (TSX)
```tsx
import PageRegisterDual from "./ui/PageRegisterDual";

export function Example() {
  return <PageRegisterDual />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import PageRegisterDual from "./ui/PageRegisterDual.vue";
</script>

<template>
  <PageRegisterDual />
</template>
```

### Flutter (Dart)
```dart
import 'ui/PageRegisterDual.dart';

// Inside build():
// PageRegisterDual(props)
```
