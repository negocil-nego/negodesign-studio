# PageRegister (`page-register`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/PageRegister.svelte`
- **React**: `ui/PageRegister.tsx`
- **Vue**: `ui/PageRegister.vue`
- **Flutter View**: `ui/PageRegister.dart`
- **Component Dependencies**: `button`, `form`, `section-form`, `simple-section`, `text`

## Props (`PageRegisterProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `PageRegisterProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`PageRegisterViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import PageRegister from "./ui/PageRegister.svelte";
</script>

<PageRegister />
```

### React (TSX)
```tsx
import PageRegister from "./ui/PageRegister";

export function Example() {
  return <PageRegister />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import PageRegister from "./ui/PageRegister.vue";
</script>

<template>
  <PageRegister />
</template>
```

### Flutter (Dart)
```dart
import 'ui/PageRegister.dart';

// Inside build():
// PageRegister(props)
```
