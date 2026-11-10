/**
 * Intermediate Design Tokens for Negodesign.
 * Shared source of truth converted into Web StyleComponent classes and Flutter ThemeData.
 */
export const DesignTokens = {
  colors: {
    blenderBase: "#12121A",
    blenderSurface: "#16161E",
    blenderCard: "#1E1E2E",
    blenderBorder: "#2E2E3E",
    blenderHover: "#262636",
    blenderActive: "#2E2E42",
    primary: "#3B82F6",
    primaryHover: "#2563EB",
    textPrimary: "#F4F4F5",
    textSecondary: "#D4D4D8",
    textMuted: "#71717A",
    danger: "#DC2626",
    success: "#059669",
    warning: "#D97706",
  },
  radius: {
    sm: 4,
    md: 6,
    lg: 8,
    xl: 12,
    xxl: 16,
    full: 9999,
  },
  spacing: {
    xs: 4,
    sm: 8,
    md: 12,
    lg: 16,
    xl: 24,
  },
  primitives: {
    buttonDefault: "btn w-full py-2 px-4 rounded-md text-center text-nowrap",
    inputForm:
      "form-control flex w-full min-w-0 items-center gap-2 rounded-md border border-blender-border bg-blender-surface px-3 py-2 text-sm text-zinc-200 focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 [&>input]:w-full [&>input]:min-w-0 [&>input]:border-0 [&>input]:bg-transparent [&>input]:p-0 [&>input]:text-inherit [&>input]:outline-none [&>input::placeholder]:text-zinc-500 [&>input:disabled]:cursor-not-allowed [&>input:disabled]:opacity-50",
  },
} as const;

export type DesignTokensType = typeof DesignTokens;
