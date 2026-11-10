# CountryPhonePopover (`country-popover`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/CountryPhonePopover.svelte`
- **React**: `ui/CountryPhonePopover.tsx`
- **Vue**: `ui/CountryPhonePopover.vue`
- **Flutter View**: `ui/CountryPhonePopover.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`CountryPhonePopoverViewModel`)
- **Component Dependencies**: `popover`

## Props (`CountryPhonePopoverProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `flag` | `FlagNamed \| string` | Yes |
| `locale` | `string` | No |
| `onselect` | `(country: CountryPhoneOption) => void` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`CountryPhonePopoverViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import CountryPhonePopover from "./ui/CountryPhonePopover.svelte";
</script>

<CountryPhonePopover />
```

### React (TSX)
```tsx
import CountryPhonePopover from "./ui/CountryPhonePopover";

export function Example() {
  return <CountryPhonePopover />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import CountryPhonePopover from "./ui/CountryPhonePopover.vue";
</script>

<template>
  <CountryPhonePopover />
</template>
```

### Flutter (Dart)
```dart
import 'ui/CountryPhonePopover.dart';

// Inside build():
// CountryPhonePopover(props)
```
