# DualSection (`dual-section`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/DualSection.svelte`
- **React**: `ui/DualSection.tsx`
- **Vue**: `ui/DualSection.vue`
- **Flutter View**: `ui/DualSection.dart`
- **Component Dependencies**: `section-form`

## Props (`DualSectionProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `DualSectionProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`DualSectionViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import DualSection from "./ui/DualSection.svelte";
</script>

<DualSection />
```

### React (TSX)
```tsx
import DualSection from "./ui/DualSection";

export function Example() {
  return <DualSection />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import DualSection from "./ui/DualSection.vue";
</script>

<template>
  <DualSection />
</template>
```

### Flutter (Dart)
```dart
import 'ui/DualSection.dart';

// Inside build():
// DualSection(props)
```
