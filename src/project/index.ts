/**
 * NegoDesign Package Entrypoint (Web: Svelte, React, Vue)
 * When copied to `/src/negodesign` in target projects (SvelteKit, Nuxt, Next.js, Vite),
 * import theme, icons, and shared hooks directly from `negodesign`.
 */

export {
  StyleComponent,
  LocalStyleComponent,
  extendStyleComponent,
  BOOTSWATCH_THEMES,
  BOOTSWATCH_THEME_LIST,
  getBootswatchTheme,
  applyBootswatchThemeToRoot,
} from "./data/style.component";
export type {
  StyleComponentType,
  BootswatchThemeId,
  BootswatchThemePalette,
  BootswatchThemeScope,
  BootswatchThemeSelection,
} from "./data/style.component";
export { DesignTokens } from "./data/theme/ts/tokens";
export type { DesignTokensType } from "./data/theme/ts/tokens";
export { createStyleTheme } from "./data/theme/ts/theme.builder";
export type { DeepPartial } from "./data/theme/ts/theme.builder";
export { NegoIconsConfig, resolveIconClass } from "./data/icons/icons.config";
export type { IconDefinition, IconRegistry } from "./data/icons/icons.config";

export * from "./hooks/data-table.hook";
export * from "./hooks/date-picker.hook";
export * from "./hooks/time-picker.hook";
export * from "./hooks/otp-input.hook";
export * from "./hooks/menu.hook";
export * from "./hooks/sidebar.hook";
