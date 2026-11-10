# ButtonBase (`button-base`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/ButtonBase.svelte`
- **React**: `ui/ButtonBase.tsx`
- **Vue**: `ui/ButtonBase.vue`
- **Flutter View**: `ui/ButtonBase.dart`
- **Component Dependencies**: `icon`

## Props (`ButtonBaseProps`)

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
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`ButtonBaseViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import ButtonBase from "./ui/ButtonBase.svelte";
</script>

<ButtonBase />
```

### React (TSX)
```tsx
import ButtonBase from "./ui/ButtonBase";

export function Example() {
  return <ButtonBase />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import ButtonBase from "./ui/ButtonBase.vue";
</script>

<template>
  <ButtonBase />
</template>
```

### Flutter (Dart)
```dart
import 'ui/ButtonBase.dart';

// Inside build():
// ButtonBase(props)
```
