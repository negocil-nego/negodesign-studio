export const IconNamed = {
  FolderAdd: "folder-add-stroke-rounded.svg",
  Layout04: "layout-04-stroke-rounded.svg",
  Settings01: "settings-01-stroke-rounded.svg",
  DocumentValidation: "document-validation-stroke-rounded.svg",
  HelpSquare: "help-square-stroke-rounded.svg",
  CheckmarkSquare01: "checkmark-square-01-stroke-rounded.svg",
  Languages: "languages-stroke-rounded.svg",
  Security: "security-stroke-rounded.svg",
  Home: "home-04-stroke-rounded.svg",

  FOLDER_ADD: "folder-add-stroke-rounded.svg",
  LAYOUT_04: "layout-04-stroke-rounded.svg",
  SETTINGS_01: "settings-01-stroke-rounded.svg",
  DOCUMENT_VALIDATION: "document-validation-stroke-rounded.svg",
  HELP_SQUARE: "help-square-stroke-rounded.svg",
  CHECKMARK_SQUARE_01: "checkmark-square-01-stroke-rounded.svg",
} as const;

export type IconNamed =
  | "folder-add-stroke-rounded.svg"
  | "layout-04-stroke-rounded.svg"
  | "settings-01-stroke-rounded.svg"
  | "document-validation-stroke-rounded.svg"
  | "help-square-stroke-rounded.svg"
  | "checkmark-square-01-stroke-rounded.svg"
  | "languages-stroke-rounded.svg"
  | "home-04-stroke-rounded.svg"
  | "security-stroke-rounded.svg";

/**
 * Helper para obter o caminho completo do ícone em /icons/
 */
export function getIconUrl(icon: IconNamed): string {
  return `/icons/${icon}`;
}
