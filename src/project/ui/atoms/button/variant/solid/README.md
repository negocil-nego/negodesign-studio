# SolidButton (`button-solid`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SolidButton.svelte`
- **React**: `ui/SolidButton.tsx`
- **Vue**: `ui/SolidButton.vue`
- **Flutter View**: `ui/SolidButton.dart`
- **Component Dependencies**: `button-base`

## Props (`SolidButtonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `icon` | `IconProps` | No |
| `text` | `string` | No |
| `className` | `string` | No |
| `isLoading` | `boolean` | No |
| `isSkeleton` | `boolean` | No |
| `isDisabled` | `boolean` | No |
| `onClick` | `() => void` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SolidButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SolidButton from "./ui/SolidButton.svelte";
</script>

<SolidButton />
```

### React (TSX)
```tsx
import SolidButton from "./ui/SolidButton";

export function Example() {
  return <SolidButton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SolidButton from "./ui/SolidButton.vue";
</script>

<template>
  <SolidButton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SolidButton.dart';

// Inside build():
// SolidButton(props)
```
