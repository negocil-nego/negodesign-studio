import translations from "./translations";

export function translateText(
  language: string,
  key: string,
  vars: Record<string, string | number> = {},
): string {
  if (!key) throw new Error("no key provided to translateText()");
  if (!language) throw new Error(`no translation for key "${key}"`);

  const languageTranslations = (translations as Record<string, Record<string, string>>)[language];
  if (!languageTranslations) {
    throw new Error(`no translations found for locale "${language}"`);
  }

  let text = languageTranslations[key];
  if (!text) throw new Error(`no translation found for ${language}.${key}`);

  for (const [name, value] of Object.entries(vars)) {
    text = text.replaceAll(`{{${name}}}`, String(value));
  }

  return text;
}
