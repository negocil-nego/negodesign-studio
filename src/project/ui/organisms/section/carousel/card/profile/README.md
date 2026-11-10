# CarouselCardProfile (`carousel-card-profile`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CarouselCardProfile.svelte`
- **React**: `ui/CarouselCardProfile.tsx`
- **Vue**: `ui/CarouselCardProfile.vue`
- **Flutter View**: `ui/CarouselCardProfile.dart`
- **Component Dependencies**: `card-profile`, `section-carousel-buttons`

## Props (`CarouselCardProfileProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `sectionCarouselButtonsProps` | `SectionCarouselButtonsProps` | No |
| `items` | `CardProfileProps[]` | Yes |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CarouselCardProfileViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CarouselCardProfile from "./ui/CarouselCardProfile.svelte";
</script>

<CarouselCardProfile />
```

### React (TSX)
```tsx
import CarouselCardProfile from "./ui/CarouselCardProfile";

export function Example() {
  return <CarouselCardProfile />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CarouselCardProfile from "./ui/CarouselCardProfile.vue";
</script>

<template>
  <CarouselCardProfile />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CarouselCardProfile.dart';

// Inside build():
// CarouselCardProfile(props)
```
