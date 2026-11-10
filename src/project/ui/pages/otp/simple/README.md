# PageOtp (`page-otp`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/PageOtp.svelte`
- **React**: `ui/PageOtp.tsx`
- **Vue**: `ui/PageOtp.vue`
- **Flutter View**: `ui/PageOtp.dart`
- **Component Dependencies**: `button`, `input-otp`, `section-otp`, `text`

## Props (`PageOtpProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `PageOtpProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`PageOtpViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import PageOtp from "./ui/PageOtp.svelte";
</script>

<PageOtp />
```

### React (TSX)
```tsx
import PageOtp from "./ui/PageOtp";

export function Example() {
  return <PageOtp />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import PageOtp from "./ui/PageOtp.vue";
</script>

<template>
  <PageOtp />
</template>
```

### Flutter (Dart)
```dart
import 'ui/PageOtp.dart';

// Inside build():
// PageOtp(props)
```
