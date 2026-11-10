# Form (`form`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/form.svelte`
- **React**: `ui/form.tsx`
- **Vue**: `ui/form.vue`
- **Flutter View**: `ui/Form.dart`
- **Component Dependencies**: `checkbox`, `country-popover`, `input-age`, `input-date`, `input-email`, `input-label`, `input-number`, `input-otp`, `input-password`, `input-phone`, `input-phone-flag`, `input-text`, `input-time`, `input-username`, `input-username-or-email`, `input-username-or-phone`, `radio`, `select`, `textarea`, `toggle`

## Props (`FormProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `FormProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`FormViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Form from "./ui/form.svelte";
</script>

<Form />
```

### React (TSX)
```tsx
import Form from "./ui/form";

export function Example() {
  return <Form />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Form from "./ui/form.vue";
</script>

<template>
  <Form />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Form.dart';

// Inside build():
// Form(props)
```
