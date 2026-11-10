# SectionCarouselButtons (`section-carousel-buttons`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/SectionCarouselButtons.svelte`
- **React**: `ui/SectionCarouselButtons.tsx`
- **Vue**: `ui/SectionCarouselButtons.vue`
- **Flutter View**: `ui/SectionCarouselButtons.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`SectionCarouselButtonsViewModel`)
- **Component Dependencies**: `icon`

## Props (`SectionCarouselButtonsProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `titleProps` | `TextProps` | No |
| `descriptionProps` | `TextProps` | No |
| `children` | `Snippet \| ReactNode \| slot \| List<Widget>` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`SectionCarouselButtonsViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import SectionCarouselButtons from "./ui/SectionCarouselButtons.svelte";
</script>

<SectionCarouselButtons />
```

### React (TSX)
```tsx
import SectionCarouselButtons from "./ui/SectionCarouselButtons";

export function Example() {
  return <SectionCarouselButtons />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import SectionCarouselButtons from "./ui/SectionCarouselButtons.vue";
</script>

<template>
  <SectionCarouselButtons />
</template>
```

### Flutter (Dart)
```dart
import 'ui/SectionCarouselButtons.dart';

// Inside build():
// SectionCarouselButtons(props)
```
