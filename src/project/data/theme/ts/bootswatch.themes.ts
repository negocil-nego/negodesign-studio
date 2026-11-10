import { BOOTSWATCH_PART_1 } from "./bootswatch_part1";
import { BOOTSWATCH_PART_2 } from "./bootswatch_part2";
import { BOOTSWATCH_PART_3 } from "./bootswatch_part3";
import { BOOTSWATCH_PART_4 } from "./bootswatch_part4";
import { BOOTSWATCH_PART_5 } from "./bootswatch_part5";
import { BOOTSWATCH_PART_6 } from "./bootswatch_part6";
import { BOOTSWATCH_PART_7 } from "./bootswatch_part7";

export interface BootswatchThemePalette {
  id: string;
  name: string;
  description: string;
  isDark: boolean;
  primary: string;
  secondary: string;
  success: string;
  info: string;
  warning: string;
  danger: string;
  bodyBg: string;
  bodyColor: string;
  surfaceBg: string;
  cardBg: string;
  borderColor: string;
}

export const BOOTSWATCH_THEMES = {
  ...BOOTSWATCH_PART_1,
  ...BOOTSWATCH_PART_2,
  ...BOOTSWATCH_PART_3,
  ...BOOTSWATCH_PART_4,
  ...BOOTSWATCH_PART_5,
  ...BOOTSWATCH_PART_6,
  ...BOOTSWATCH_PART_7,
} as const;

export type BootswatchThemeId = keyof typeof BOOTSWATCH_THEMES;
export type BootswatchThemeSelection = "default" | BootswatchThemeId;
export type BootswatchThemeScope = "app" | "project";

export const BOOTSWATCH_THEME_LIST: BootswatchThemePalette[] = Object.values(BOOTSWATCH_THEMES);

export function getBootswatchTheme(id: BootswatchThemeSelection): BootswatchThemePalette | null {
  if (id === "default") return null;
  return BOOTSWATCH_THEMES[id] ?? null;
}

export function applyBootswatchThemeToRoot(
  theme: BootswatchThemeSelection,
  scope: BootswatchThemeScope = "app",
): void {
  if (typeof document === "undefined") return;
  const root = document.documentElement;
  if (scope === "app" && theme !== "default") {
    const palette = getBootswatchTheme(theme);
    root.dataset.bootswatch = theme;
    if (palette) root.dataset.theme = palette.isDark ? "dark" : "light";
  } else {
    delete root.dataset.bootswatch;
  }
}
