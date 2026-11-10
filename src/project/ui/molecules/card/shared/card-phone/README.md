# CardPhone (`card-phone`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CardPhone.svelte`
- **React**: `ui/CardPhone.tsx`
- **Vue**: `ui/CardPhone.vue`
- **Flutter View**: `ui/CardPhone.dart`
- **Component Dependencies**: None (Standalone)

## Props (`CardPhoneProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `CardPhoneProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CardPhoneViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CardPhone from "./ui/CardPhone.svelte";
</script>

<CardPhone />
```

### React (TSX)
```tsx
import CardPhone from "./ui/CardPhone";

export function Example() {
  return <CardPhone />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CardPhone from "./ui/CardPhone.vue";
</script>

<template>
  <CardPhone />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CardPhone.dart';

// Inside build():
// CardPhone(props)
```
