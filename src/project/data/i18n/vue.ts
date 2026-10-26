import { inject, provide, ref, type InjectionKey, type Ref } from "vue";
import { translateText } from "./translate";

export type VueI18nContext = {
  locale: Ref<string>;
  setLocale: (locale: string) => void;
  t: (key: string, vars?: Record<string, string | number>) => string;
};

const i18nKey: InjectionKey<VueI18nContext> = Symbol("negodesign-i18n");
const defaultLocale = ref("pt");
const defaultContext: VueI18nContext = {
  locale: defaultLocale,
  setLocale: (language) => {
    defaultLocale.value = language;
  },
  t: (key, vars) => translateText(defaultLocale.value, key, vars),
};

export function provideI18n(initialLocale = "pt"): VueI18nContext {
  const locale = ref(initialLocale);
  const context: VueI18nContext = {
    locale,
    setLocale: (language) => {
      locale.value = language;
    },
    t: (key, vars) => translateText(locale.value, key, vars),
  };
  provide(i18nKey, context);
  return context;
}

export function useI18n(): VueI18nContext {
  return inject(i18nKey, defaultContext);
}
