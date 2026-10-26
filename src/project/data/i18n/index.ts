import { derived, writable } from "svelte/store";
import translations from "./translations";
import { defaultLocale, translateText } from "./translate";

export const locale = writable(defaultLocale);
export const locales = Object.keys(translations);
export { translateText };

export const t = derived(locale, ($locale) => (key: string, vars: Record<string, string | number> = {}) =>
  translateText($locale, key, vars)
);
