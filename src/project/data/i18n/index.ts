import { derived, writable } from "svelte/store";
import translations from "./translations";
import { translateText } from "./translate";

export const locale = writable("pt");
export const locales = Object.keys(translations);
export { translateText };

export const t = derived(locale, ($locale) => (key: string, vars: Record<string, string | number> = {}) =>
  translateText($locale, key, vars)
);
