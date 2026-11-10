# SimpleSection (`simple-section`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SimpleSection.svelte`
- **React**: `ui/SimpleSection.tsx`
- **Vue**: `ui/SimpleSection.vue`
- **Flutter View**: `ui/SimpleSection.dart`
- **Component Dependencies**: `section-form`

## Props (`SimpleSectionProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `props` | `SimpleSectionProps` | Declarative composite props object |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SimpleSectionViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SimpleSection from "./ui/SimpleSection.svelte";
</script>

<SimpleSection />
```

### React (TSX)
```tsx
import SimpleSection from "./ui/SimpleSection";

export function Example() {
  return <SimpleSection />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SimpleSection from "./ui/SimpleSection.vue";
</script>

<template>
  <SimpleSection />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SimpleSection.dart';

// Inside build():
// SimpleSection(props)
```
