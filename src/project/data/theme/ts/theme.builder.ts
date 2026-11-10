export type DeepPartial<T> = {
  [K in keyof T]?: T[K] extends Record<string, unknown> ? DeepPartial<T[K]> : T[K];
};

function mergeDeep<T extends Record<string, unknown>>(base: T, patch: DeepPartial<T>): T {
  const output: Record<string, unknown> = { ...base };
  for (const key of Object.keys(patch)) {
    const patchVal = (patch as Record<string, unknown>)[key];
    const baseVal = base[key];
    if (
      patchVal &&
      typeof patchVal === "object" &&
      !Array.isArray(patchVal) &&
      baseVal &&
      typeof baseVal === "object"
    ) {
      output[key] = mergeDeep(
        baseVal as Record<string, unknown>,
        patchVal as DeepPartial<Record<string, unknown>>,
      );
    } else if (patchVal !== undefined) {
      output[key] = patchVal;
    }
  }
  return output as T;
}

/**
 * Creates a customized StyleComponent configuration (similar to Flutter's ThemeData.copyWith
 * and React Native's StyleSheet.create), allowing external overrides without touching components.
 */
export function createStyleTheme<T extends Record<string, unknown>>(
  baseTheme: T,
  overrides: DeepPartial<T> = {},
): T {
  return mergeDeep(baseTheme, overrides);
}
