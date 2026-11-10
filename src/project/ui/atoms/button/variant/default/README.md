# Button (`button`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Button.svelte`
- **React**: `ui/Button.tsx`
- **Vue**: `ui/Button.vue`
- **Flutter View**: `ui/Button.dart`
- **Component Dependencies**: `button-base`, `button-outline`, `button-solid`

## Props (`ButtonProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `variant` | `"solid" \| "outline"` | No |
| `icon` | `IconProps` | No |
| `text` | `string` | No |
| `className` | `string` | No |
| `isLoading` | `boolean` | No |
| `isSkeleton` | `boolean` | No |
| `isDisabled` | `boolean` | No |
| `onClick` | `() => void` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`ButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Button from "./ui/Button.svelte";
</script>

<Button />
```

### React (TSX)
```tsx
import Button from "./ui/Button";

export function Example() {
  return <Button />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Button from "./ui/Button.vue";
</script>

<template>
  <Button />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Button.dart';

// Inside build():
// Button(props)
```
