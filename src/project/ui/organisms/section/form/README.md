# SectionForm (`section-form`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SectionForm.svelte`
- **React**: `ui/SectionForm.tsx`
- **Vue**: `ui/SectionForm.vue`
- **Flutter View**: `ui/SectionForm.dart`
- **Component Dependencies**: `button`, `form`, `text`

## Props (`SectionFormProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `SectionFormProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SectionFormViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SectionForm from "./ui/SectionForm.svelte";
</script>

<SectionForm />
```

### React (TSX)
```tsx
import SectionForm from "./ui/SectionForm";

export function Example() {
  return <SectionForm />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SectionForm from "./ui/SectionForm.vue";
</script>

<template>
  <SectionForm />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SectionForm.dart';

// Inside build():
// SectionForm(props)
```
