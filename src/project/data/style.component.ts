import { AtomsStyle } from "./theme/ts/atoms.style";
import { BootswatchComponentsStyle } from "./theme/ts/bootswatch_components.style";
import { CardsStyle } from "./theme/ts/cards.style";
import { InputsStyle } from "./theme/ts/inputs.style";
import { MenuStyle } from "./theme/ts/menu.style";
import { OrganismsStyle } from "./theme/ts/organisms.style";
import { PagesStyle } from "./theme/ts/pages.style";
import { createStyleTheme, type DeepPartial } from "./theme/ts/theme.builder";
import { DesignTokens } from "./theme/ts/tokens";

export {
  BOOTSWATCH_THEMES,
  BOOTSWATCH_THEME_LIST,
  getBootswatchTheme,
  applyBootswatchThemeToRoot,
  type BootswatchThemeId,
  type BootswatchThemePalette,
  type BootswatchThemeScope,
  type BootswatchThemeSelection,
} from "./theme/ts/bootswatch.themes";

const BaseStyleComponent = {
  tokens: DesignTokens,
  ...AtomsStyle,
  ...InputsStyle,
  ...CardsStyle,
  ...OrganismsStyle,
  ...MenuStyle,
  ...PagesStyle,
  ...BootswatchComponentsStyle,
} as const;

export type StyleComponentType = typeof BaseStyleComponent;

/**
 * Default external StyleComponent configuration for Web frameworks (Svelte, React, Vue).
 * Can be customized globally via `extendStyleComponent(overrides)`.
 */
export const StyleComponent = BaseStyleComponent;
export const LocalStyleComponent = BaseStyleComponent;

export function extendStyleComponent(overrides: DeepPartial<StyleComponentType>): StyleComponentType {
  return createStyleTheme(BaseStyleComponent, overrides);
}
