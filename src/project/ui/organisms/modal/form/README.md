# ModalForm (`modal-form`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/ModalForm.svelte`
- **React**: `ui/ModalForm.tsx`
- **Vue**: `ui/ModalForm.vue`
- **Flutter View**: `ui/ModalForm.dart`
- **Component Dependencies**: `form`, `modal-core`

## Props (`ModalFormProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `ModalFormProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`ModalFormViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import ModalForm from "./ui/ModalForm.svelte";
</script>

<ModalForm />
```

### React (TSX)
```tsx
import ModalForm from "./ui/ModalForm";

export function Example() {
  return <ModalForm />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import ModalForm from "./ui/ModalForm.vue";
</script>

<template>
  <ModalForm />
</template>
```

### Flutter (Dart)
```dart
import 'ui/ModalForm.dart';

// Inside build():
// ModalForm(props)
```
