# ListGroups (`list-groups`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/ListGroups.svelte`
- **React**: `ui/ListGroups.tsx`
- **Vue**: `ui/ListGroups.vue`
- **Flutter View**: `ui/ListGroups.dart`
- **Component Dependencies**: None (Standalone)

## Props (`ListGroupsProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `items` | `ListGroupEntry[]` | Yes |
| `activeId` | `string` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`ListGroupsViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import ListGroups from "./ui/ListGroups.svelte";
</script>

<ListGroups />
```

### React (TSX)
```tsx
import ListGroups from "./ui/ListGroups";

export function Example() {
  return <ListGroups />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import ListGroups from "./ui/ListGroups.vue";
</script>

<template>
  <ListGroups />
</template>
```

### Flutter (Dart)
```dart
import 'ui/ListGroups.dart';

// Inside build():
// ListGroups(props)
```
