# OutlineButton (`button-outline`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/OutlineButton.svelte`
- **React**: `ui/OutlineButton.tsx`
- **Vue**: `ui/OutlineButton.vue`
- **Flutter View**: `ui/OutlineButton.dart`
- **Component Dependencies**: `button-base`

## Props (`OutlineButtonProps`)

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
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`OutlineButtonViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import OutlineButton from "./ui/OutlineButton.svelte";
</script>

<OutlineButton />
```

### React (TSX)
```tsx
import OutlineButton from "./ui/OutlineButton";

export function Example() {
  return <OutlineButton />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import OutlineButton from "./ui/OutlineButton.vue";
</script>

<template>
  <OutlineButton />
</template>
```

### Flutter (Dart)
```dart
import 'ui/OutlineButton.dart';

// Inside build():
// OutlineButton(props)
```
