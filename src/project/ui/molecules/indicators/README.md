# Indicators (`indicators`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/Indicators.svelte`
- **React**: `ui/Indicators.tsx`
- **Vue**: `ui/Indicators.vue`
- **Flutter View**: `ui/Indicators.dart`
- **Component Dependencies**: None (Standalone)

## Props (`IndicatorsProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `alerts` | `AlertItem[]` | No |
| `badges` | `BadgeItem[]` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`IndicatorsViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import Indicators from "./ui/Indicators.svelte";
</script>

<Indicators />
```

### React (TSX)
```tsx
import Indicators from "./ui/Indicators";

export function Example() {
  return <Indicators />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import Indicators from "./ui/Indicators.vue";
</script>

<template>
  <Indicators />
</template>
```

### Flutter (Dart)
```dart
import 'ui/Indicators.dart';

// Inside build():
// Indicators(props)
```
