import {
  BOOTSWATCH_THEME_LIST,
  getBootswatchTheme,
  type BootswatchThemeScope,
  type BootswatchThemeSelection,
} from "../../../project/data/style.component";

const STORAGE_THEME_KEY = "negodesign-bootswatch-theme";
const STORAGE_SCOPE_KEY = "negodesign-bootswatch-scope";
const STORAGE_MODE_KEY = "negodesign-theme";

class BootswatchThemeController {
  theme = $state<BootswatchThemeSelection>("default");
  scope = $state<BootswatchThemeScope>("app");
  colorMode = $state<"light" | "dark">("dark");
  initialized = false;

  readonly themes = BOOTSWATCH_THEME_LIST;

  readonly activePalette = $derived(getBootswatchTheme(this.theme));

  readonly projectDataAttr = $derived(
    this.scope === "project" && this.theme !== "default" ? this.theme : undefined,
  );

  init() {
    if (typeof window === "undefined" || this.initialized) return;
    this.initialized = true;
    const savedTheme = localStorage.getItem(STORAGE_THEME_KEY) as BootswatchThemeSelection | null;
    const savedScope = localStorage.getItem(STORAGE_SCOPE_KEY) as BootswatchThemeScope | null;
    const savedMode = localStorage.getItem(STORAGE_MODE_KEY);

    if (savedTheme) this.theme = savedTheme;
    if (savedScope === "app" || savedScope === "project") this.scope = savedScope;
    this.colorMode = savedMode === "light" ? "light" : "dark";
    this.syncDom();
  }

  setTheme(nextTheme: BootswatchThemeSelection) {
    this.theme = nextTheme;
    if (typeof window !== "undefined") {
      localStorage.setItem(STORAGE_THEME_KEY, nextTheme);
    }
    const palette = getBootswatchTheme(nextTheme);
    if (palette && this.scope === "app") {
      this.colorMode = palette.isDark ? "dark" : "light";
      if (typeof window !== "undefined") {
        localStorage.setItem(STORAGE_MODE_KEY, this.colorMode);
      }
    }
    this.syncDom();
  }

  setScope(nextScope: BootswatchThemeScope) {
    this.scope = nextScope;
    if (typeof window !== "undefined") {
      localStorage.setItem(STORAGE_SCOPE_KEY, nextScope);
    }
    this.syncDom();
  }

  toggleColorMode() {
    this.colorMode = this.colorMode === "light" ? "dark" : "light";
    if (typeof window !== "undefined") {
      localStorage.setItem(STORAGE_MODE_KEY, this.colorMode);
    }
    this.syncDom();
  }

  syncDom() {
    if (typeof document === "undefined") return;
    const root = document.documentElement;
    if (this.scope === "app" && this.theme !== "default") {
      const palette = getBootswatchTheme(this.theme);
      root.dataset.bootswatch = this.theme;
      root.dataset.theme = palette?.isDark ? "dark" : "light";
    } else {
      delete root.dataset.bootswatch;
      root.dataset.theme = this.colorMode;
    }
  }
}

export const bootswatchUi = new BootswatchThemeController();

