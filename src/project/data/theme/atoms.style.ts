import { DesignTokens } from "./tokens";

const { buttonDefault: ButtonDefault } = DesignTokens.primitives;

export const AtomsStyle = {
  avatar: {
    default: "w-12 h-12 rounded-lg",
  },
  image: {
    default: "w-12 h-12",
  },
  button: {
    solid: {
      default: `${ButtonDefault} bg-blue-900 hover:bg-primary/80 text-white`,
      danger: `${ButtonDefault} bg-destructive/10 text-destructive hover:bg-destructive/20 focus-visible:border-destructive/40 focus-visible:ring-destructive/20 dark:bg-destructive/20 dark:hover:bg-destructive/30 dark:focus-visible:ring-destructive/40`,
      success: `${ButtonDefault} bg-success/10 text-success hover:bg-success/20 focus-visible:border-success/40 focus-visible:ring-success/20 dark:bg-success/20 dark:hover:bg-success/30 dark:focus-visible:ring-success/40`,
      warning: `${ButtonDefault} bg-warning/10 text-warning hover:bg-warning/20 focus-visible:border-warning/40 focus-visible:ring-warning/20 dark:bg-warning/20 dark:hover:bg-warning/30 dark:focus-visible:ring-warning/40`,
    },
    outline: {
      default: `${ButtonDefault} border border-primary/40 text-primary-foreground`,
      danger: `${ButtonDefault} bg-destructive/10 text-destructive hover:bg-destructive/20 focus-visible:border-destructive/40 focus-visible:ring-destructive/20 dark:bg-destructive/20 dark:hover:bg-destructive/30 dark:focus-visible:ring-destructive/40`,
      success: `${ButtonDefault} bg-success/10 text-success hover:bg-success/20 focus-visible:border-success/40 focus-visible:ring-success/20 dark:bg-success/20 dark:hover:bg-success/30 dark:focus-visible:ring-success/40`,
      warning: `${ButtonDefault} bg-warning/10 text-warning hover:bg-warning/20 focus-visible:border-warning/40 focus-visible:ring-warning/20 dark:bg-warning/20 dark:hover:bg-warning/30 dark:focus-visible:ring-warning/40`,
    },
  },
  iconCenterButton: {
    default: "w-10 h-10 rounded-full border border-gray-100 dark:border-slate-600 flex items-center justify-center cursor-pointer bg-gray-100 dark:bg-slate-800",
  },
  checkbox: {
    root: "group flex items-center gap-3 px-4 py-2.5 bg-white dark:bg-zinc-900 border border-zinc-100 dark:border-zinc-800 rounded-full shadow-sm hover:shadow-md transition-all cursor-pointer w-fit",
    disabled: "opacity-50 cursor-not-allowed",
    input: "sr-only",
    box: "size-6 rounded-full border-2 border-zinc-200 dark:border-zinc-700 flex items-center justify-center transition-all group-has-[:checked]:bg-blue-600 group-has-[:checked]:border-blue-600",
    tick: "size-3.5 text-white",
    label: "text-base font-bold text-zinc-900 dark:text-zinc-100 select-none",
  },
  icon: {
    spinner: "animate-spin inline-block w-5 h-5",
    default: "text-base opacity-75",
  },
  modal: {
    root: "fixed left-1/2 top-1/2 m-0 w-[calc(100%-2rem)] max-w-md -translate-x-1/2 -translate-y-1/2 rounded-xl border border-blender-border bg-white p-5 text-zinc-900 shadow-2xl dark:bg-blender-card dark:text-zinc-200 [&::backdrop]:bg-black/60 [&::backdrop]:backdrop-blur-sm",
  },
  popover: {
    root: "popover w-full",
  },
  radio: {
    group: "flex flex-col gap-3",
    item: "group flex items-center gap-3 px-4 py-2.5 bg-white dark:bg-zinc-900 border border-zinc-100 dark:border-zinc-800 rounded-full shadow-sm hover:shadow-md transition-all cursor-pointer w-fit",
    disabled: "opacity-50 cursor-not-allowed",
    input: "sr-only",
    dot: "size-6 rounded-full border-2 border-zinc-200 dark:border-zinc-700 flex items-center justify-center transition-all group-has-[:checked]:border-blue-600 after:content-[''] after:size-3 after:rounded-full after:bg-blue-600 after:opacity-0 group-has-[:checked]:after:opacity-100",
    label: "text-base font-bold text-zinc-900 dark:text-zinc-100 select-none",
  },
  select: {
    root: "relative w-full",
    trigger: "w-full flex justify-center items-center text-left",
    triggerLabel: "text-[10px] uppercase font-semibold text-zinc-500 select-none w-full",
    triggerValue: "text-sm truncate w-full",
    panel: "mt-1.5 p-2 w-full min-w-60 shadow-lg z-50",
    search: "w-full px-3 py-1.5 mb-1.5 text-sm bg-blender-surface border border-blender-border rounded text-inherit outline-none focus:border-blue-500",
    list: "max-h-56 overflow-y-auto flex flex-col gap-0.5",
    option: "w-full flex items-center justify-between px-3 py-2 rounded text-sm text-zinc-300 hover:text-slate-900 hover:bg-blender-hover active:bg-blender-active transition-colors cursor-pointer text-left",
    optionSelected: "bg-blender-hover text-slate-900 font-medium",
    optionContent: "flex items-center gap-2.5 truncate",
    optionIcon: "text-base",
    optionEmpty: "px-3 py-2 text-xs text-zinc-500 text-center select-none",
    optionCheck: "text-blue-500 font-bold text-xs",
  },
  skeleton: {
    default: "relative overflow-hidden rounded-xl bg-gray-200 before:absolute before:inset-0 before:-translate-x-full before:animate-shimmer before:bg-linear-to-r before:from-transparent before:via-white/70 before:to-transparent",
  },
  textarea: {
    form: "w-full flex flex-col items-start justify-center gap-2",
    default: "w-full px-3 py-1.5 mb-1.5 text-sm bg-blender-surface border border-blender-border rounded text-inherit outline-none focus:border-blue-500",
  },
  toggle: {
    root: "group flex items-center gap-3 px-4 py-2.5 bg-white dark:bg-zinc-900 border border-zinc-100 dark:border-zinc-800 rounded-full shadow-sm hover:shadow-md transition-all cursor-pointer w-fit",
    track: "relative inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full transition-colors duration-200 focus:outline-none bg-zinc-200 dark:bg-zinc-700",
    on: "bg-blue-600",
    thumb: "pointer-events-none block h-5 w-5 rounded-full bg-white shadow-sm ring-0 transition-transform duration-200 translate-x-0.5",
    thumbOn: "translate-x-5.5",
    label: "text-base font-bold text-zinc-900 dark:text-zinc-100 select-none",
    text: "hidden",
    textOn: "",
    textOff: "",
  },
} as const;
