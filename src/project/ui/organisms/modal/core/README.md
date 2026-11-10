# Modal (`modal-core`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Modal.svelte`
- **React**: `ui/Modal.tsx`
- **Vue**: `ui/Modal.vue`
- **Flutter View**: `ui/Modal.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`ModalViewModel`)
- **Component Dependencies**: None (Standalone)

## Props (`ModalProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `ModalProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`ModalViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Modal from "./ui/Modal.svelte";
</script>

<Modal />
```

### React (TSX)
```tsx
import Modal from "./ui/Modal";

export function Example() {
  return <Modal />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Modal from "./ui/Modal.vue";
</script>

<template>
  <Modal />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Modal.dart';

// Inside build():
// Modal(props)
```
