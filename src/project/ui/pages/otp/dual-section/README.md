# PageOtpDual (`page-otp-dual`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/PageOtpDual.svelte`
- **React**: `ui/PageOtpDual.tsx`
- **Vue**: `ui/PageOtpDual.vue`
- **Flutter View**: `ui/PageOtpDual.dart`
- **Component Dependencies**: `button`, `input-otp`, `section-otp`, `text`

## Props (`PageOtpDualProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `PageOtpDualProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`PageOtpDualViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import PageOtpDual from "./ui/PageOtpDual.svelte";
</script>

<PageOtpDual />
```

### React (TSX)
```tsx
import PageOtpDual from "./ui/PageOtpDual";

export function Example() {
  return <PageOtpDual />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import PageOtpDual from "./ui/PageOtpDual.vue";
</script>

<template>
  <PageOtpDual />
</template>
```

### Flutter (Dart)
```dart
import 'ui/PageOtpDual.dart';

// Inside build():
// PageOtpDual(props)
```
