# SectionOtp (`section-otp`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SectionOtp.svelte`
- **React**: `ui/SectionOtp.tsx`
- **Vue**: `ui/SectionOtp.vue`
- **Flutter View**: `ui/SectionOtp.dart`
- **Component Dependencies**: `button`, `input-otp`, `text`

## Props (`SectionOtpProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `SectionOtpProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SectionOtpViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SectionOtp from "./ui/SectionOtp.svelte";
</script>

<SectionOtp />
```

### React (TSX)
```tsx
import SectionOtp from "./ui/SectionOtp";

export function Example() {
  return <SectionOtp />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SectionOtp from "./ui/SectionOtp.vue";
</script>

<template>
  <SectionOtp />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SectionOtp.dart';

// Inside build():
// SectionOtp(props)
```
