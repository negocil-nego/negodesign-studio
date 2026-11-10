# PageLogin (`page-login`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/PageLogin.svelte`
- **React**: `ui/PageLogin.tsx`
- **Vue**: `ui/PageLogin.vue`
- **Flutter View**: `ui/PageLogin.dart`
- **Component Dependencies**: `button`, `form`, `section-form`, `simple-section`, `text`

## Props (`PageLoginProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `PageLoginProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`PageLoginViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import PageLogin from "./ui/PageLogin.svelte";
</script>

<PageLogin />
```

### React (TSX)
```tsx
import PageLogin from "./ui/PageLogin";

export function Example() {
  return <PageLogin />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import PageLogin from "./ui/PageLogin.vue";
</script>

<template>
  <PageLogin />
</template>
```

### Flutter (Dart)
```dart
import 'ui/PageLogin.dart';

// Inside build():
// PageLogin(props)
```
