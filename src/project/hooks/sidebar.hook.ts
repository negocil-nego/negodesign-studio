export const SIDEBAR_RAIL_ICONS = ["Grid", "Search", "Star", "Home", "Mail", "User", "Settings"] as const;
export const SIDEBAR_RAIL_FOOTER_ICONS = ["Bell", "Settings"] as const;

export function computeSidebarTheme(variant = "01", theme = "light", collapsed = false, className = "") {
  const dark = theme === "dark";
  const compact = collapsed;
  const rail = variant === "02";
  const surface = dark
    ? "border-zinc-800 bg-zinc-950 text-zinc-100"
    : "border-zinc-200 bg-white text-zinc-900";
  const widthCls = rail
    ? "w-[min(100%,420px)] flex-row"
    : compact
      ? "w-[76px]"
      : "w-[min(100%,280px)]";
  return {
    dark,
    compact,
    rail,
    showSearch: variant === "01" || variant === "02",
    accent: (variant === "03" ? "violet" : "blue") as "violet" | "blue",
    rootClass: `relative flex h-[min(760px,90vh)] min-h-[560px] shrink-0 flex-col overflow-hidden rounded-2xl border shadow-sm transition-all ${surface} ${widthCls} ${className}`,
    railClass: `flex w-[68px] shrink-0 flex-col items-center gap-3 border-r p-3 ${dark ? "border-zinc-800 bg-zinc-900" : "border-zinc-100 bg-zinc-50"}`,
    logoBoxClass: `grid h-8 w-8 shrink-0 place-items-center rounded-lg text-lg font-bold ${dark ? "bg-zinc-800" : "bg-zinc-100"}`,
    searchClass: `mb-4 flex h-9 items-center gap-2 rounded-lg border px-2.5 ${dark ? "border-zinc-700 bg-zinc-900" : "border-zinc-200 bg-white"}`,
    storageClass: `my-3 rounded-xl border p-3 ${dark ? "border-zinc-800 bg-zinc-900" : "border-zinc-200 bg-zinc-50"}`,
  };
}
