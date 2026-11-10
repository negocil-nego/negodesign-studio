# InputPhoneFlag (`input-phone-flag`)

Multi-platform declarative UI component for **Svelte 5**, **React**, **Vue 3**, and **Flutter**.
Follows NegoDesign's Object Calisthenics pattern (`*Props` composition), MVVM for Flutter (`ViewModel` in `data/`), shared TS hooks for Web, and the strict `<= 100` lines per file rule.

## Entry Files & Architecture

- **Svelte**: `ui/InputPhoneFlag.svelte`
- **React**: `ui/InputPhoneFlag.tsx`
- **Vue**: `ui/InputPhoneFlag.vue`
- **Flutter View**: `ui/InputPhoneFlag.dart`
- **Flutter MVVM ViewModel**: `data/viewmodel.dart` (`InputPhoneFlagViewModel`)
- **Component Dependencies**: `input-phone`, `input-label`, `country-popover`

## Props (`InputPhoneFlagProps`)

| Prop | Type | Required |
| :--- | :--- | :--- |
| `inputProps` | `InputProps` | Yes |
| `labelProps` | `LabelProps` | No |
| `flag` | `FlagNamed[]` | Yes |
| `locale` | `string` | No |
| `onCountrySelect` | `(country: CountryPhoneOption) => void` | No |

## Theme & MVVM / Hooks

- **Web (Svelte / React / Vue)**: Styled via `StyleComponent` (`src/project/data/style.component.ts`) and powered by shared TypeScript hooks (`src/project/hooks/`).
- **Flutter (MVVM)**: UI widgets only render the view tree; all state, `initState`/`didUpdateWidget` lifecycle logic, and controllers live in `data/viewmodel.dart` (`InputPhoneFlagViewModel`) and theme tokens come from `NegoThemeData`.

## Usage

### Svelte 5
```svelte
<script lang="ts">
  import InputPhoneFlag from "./ui/InputPhoneFlag.svelte";
</script>

<InputPhoneFlag />
```

### React (TSX)
```tsx
import InputPhoneFlag from "./ui/InputPhoneFlag";

export function Example() {
  return <InputPhoneFlag />;
}
```

### Vue 3
```vue
<script setup lang="ts">
import InputPhoneFlag from "./ui/InputPhoneFlag.vue";
</script>

<template>
  <InputPhoneFlag />
</template>
```

### Flutter (Dart)
```dart
import 'ui/InputPhoneFlag.dart';

// Inside build():
// InputPhoneFlag(props)
```
