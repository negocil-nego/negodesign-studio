import { AtomsStyle } from "./theme/atoms.style";
import { CardsStyle } from "./theme/cards.style";
import { InputsStyle } from "./theme/inputs.style";
import { OrganismsStyle } from "./theme/organisms.style";
import { createStyleTheme, type DeepPartial } from "./theme/theme.builder";
import { DesignTokens } from "./theme/tokens";

const BaseStyleComponent = {
  tokens: DesignTokens,
  ...AtomsStyle,
  ...InputsStyle,
  ...CardsStyle,
  ...OrganismsStyle,
} as const;

export type StyleComponentType = typeof BaseStyleComponent;

/**
 * Default external StyleComponent configuration for Web frameworks (Svelte, React, Vue).
 * Can be customized globally via `extendStyleComponent(overrides)`.
 */
export const StyleComponent = BaseStyleComponent;

export function extendStyleComponent(overrides: DeepPartial<StyleComponentType>): StyleComponentType {
  return createStyleTheme(BaseStyleComponent, overrides);
}
