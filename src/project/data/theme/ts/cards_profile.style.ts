export const CardsProfileStyle = {
  v01: {
    root: "card flex w-full flex-col items-center rounded-xl border border-blender-border bg-blender-card p-5 text-center shadow-sm",
    avatarWrapper: "mb-3",
    name: "card-title text-base font-semibold text-zinc-100",
    role: "card-subtitle text-xs text-zinc-400",
    meta: "mt-3 flex flex-wrap items-center justify-center gap-3 text-xs text-zinc-300",
    actions: "mt-4 flex w-full items-center justify-center gap-2",
  },
  v02: {
    root: "card flex w-full items-center gap-4 rounded-xl border border-blender-border bg-blender-card p-4 shadow-sm",
    body: "card-body min-w-0 flex-1 space-y-1 p-0",
    actions: "flex shrink-0 items-center gap-1.5",
  },
  v03: {
    root: "card flex w-full flex-col overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-sm",
    cover: "h-20 w-full bg-gradient-to-r from-blue-600/30 via-indigo-600/20 to-blender-surface",
    body: "card-body -mt-8 flex flex-col items-start gap-2 px-4 pb-4",
    header: "flex w-full items-end justify-between gap-2",
  },
} as const;
