/**
 * NegoDesign Package Entrypoint (Web: Svelte, React, Vue)
 * When copied to `/src/negodesign` in target projects (SvelteKit, Nuxt, Next.js, Vite),
 * import theme, icons, and shared hooks directly from `negodesign`.
 */

export { StyleComponent, extendStyleComponent } from "./data/style.component";
export type { StyleComponentSchema } from "./data/style.component";
export { DesignTokens } from "./data/theme/tokens";
export type { DesignTokenType } from "./data/theme/tokens";
export { createStyleTheme } from "./data/theme/theme.builder";
export type { DeepPartial } from "./data/theme/theme.builder";
export { NegoIconsConfig, resolveIconClass } from "./data/icons/icons.config";
export type { IconDefinition, IconRegistry } from "./data/icons/icons.config";

export * from "./hooks/data-table.hook";
export * from "./hooks/date-picker.hook";
export * from "./hooks/time-picker.hook";
export * from "./hooks/otp-input.hook";
export * from "./hooks/menu.hook";
export * from "./hooks/sidebar.hook";

