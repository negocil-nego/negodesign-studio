import { DesignTokens } from "./tokens";

export const AtomsStyle = {
  button: {
    default: DesignTokens.primitives.buttonDefault,
    outline: {
      default: `${DesignTokens.primitives.buttonDefault} btn-outline-secondary border border-blender-border bg-blender-surface text-zinc-200 hover:bg-blender-hover`,
    },
    solid: {
      default: `${DesignTokens.primitives.buttonDefault} btn-primary text-white shadow-xs hover:opacity-90`,
    },
    block: {
      default: `${DesignTokens.primitives.buttonDefault} btn-primary btn-lg btn-block w-full py-2.5 px-5 text-sm font-semibold text-white shadow-xs hover:opacity-90`,
    },
  },
  avatar: {
    root: "relative inline-flex shrink-0 items-center justify-center overflow-hidden rounded-full bg-blender-surface text-zinc-300 select-none border border-blender-border",
    size: {
      sm: "size-8 text-xs",
      md: "size-10 text-sm",
      lg: "size-14 text-base",
      xl: "size-20 text-lg",
    },
  },
  skeleton: {
    root: "animate-pulse rounded-md bg-blender-hover/80",
  },
  checkbox: {
    root: "form-check inline-flex items-center gap-2.5 cursor-pointer select-none",
    disabled: "cursor-not-allowed opacity-50",
    input: "peer sr-only",
    box: "form-check-input flex h-4 w-4 shrink-0 items-center justify-center rounded-[4px] border border-blender-border bg-blender-surface text-white transition-colors peer-focus-visible:ring-2 peer-focus-visible:ring-blue-500/40 peer-checked:border-[color:var(--bs-primary,#2563eb)] peer-checked:bg-[color:var(--bs-primary,#2563eb)]",
    tick: "h-2.5 w-2.5",
    label: "form-check-label text-sm text-zinc-200",
  },
  label: {
    default: "form-label block text-xs font-medium text-zinc-300 select-none",
    required: "ml-0.5 text-rose-400",
  },
  popover: {
    root: "relative inline-block",
    panel: "dropdown-menu absolute left-0 top-full z-50 mt-1.5 min-w-52 rounded-lg border border-blender-border bg-blender-card p-2 shadow-xl",
  },
  radio: {
    group: "flex flex-col gap-2",
    item: "form-check inline-flex items-center gap-2.5 cursor-pointer select-none",
    disabled: "cursor-not-allowed opacity-50",
    input: "peer sr-only",
    dot: "form-check-input flex h-4 w-4 shrink-0 items-center justify-center rounded-full border border-blender-border bg-blender-surface transition-colors peer-focus-visible:ring-2 peer-focus-visible:ring-blue-500/40 peer-checked:border-[color:var(--bs-primary,#2563eb)] peer-checked:bg-[color:var(--bs-primary,#2563eb)] after:h-1.5 after:w-1.5 after:scale-0 after:rounded-full after:bg-white after:transition-transform peer-checked:after:scale-100",
    label: "form-check-label text-sm text-zinc-200",
  },
  select: {
    root: "relative w-full min-w-0",
    trigger: `${DesignTokens.primitives.inputForm} form-select justify-between text-left disabled:cursor-not-allowed disabled:opacity-50`,
    value: "min-w-0 truncate text-zinc-200",
    placeholder: "min-w-0 truncate text-zinc-500",
    chevron: "size-4 shrink-0 text-zinc-400 transition-transform",
    panel: "dropdown-menu absolute left-0 right-0 top-full z-50 mt-1.5 max-h-60 overflow-y-auto rounded-lg border border-blender-border bg-blender-card p-1 shadow-xl",
    option: "dropdown-item flex w-full items-center justify-between gap-2 rounded-md px-2.5 py-2 text-left text-sm text-zinc-200 hover:bg-blender-hover",
    optionActive: "bg-blue-600/20 font-medium text-blue-400",
    optionDisabled: "cursor-not-allowed opacity-40 hover:bg-transparent",
  },
  textarea: {
    default: `${DesignTokens.primitives.inputForm} min-h-24 resize-y`,
  },
  toggle: {
    root: "form-check form-switch inline-flex items-center gap-2.5 cursor-pointer select-none",
    disabled: "cursor-not-allowed opacity-50",
    input: "peer sr-only",
    track: "relative h-5 w-9 shrink-0 rounded-full border border-blender-border bg-blender-surface transition-colors peer-focus-visible:ring-2 peer-focus-visible:ring-blue-500/40 peer-checked:border-[color:var(--bs-primary,#2563eb)] peer-checked:bg-[color:var(--bs-primary,#2563eb)] after:absolute after:left-0.5 after:top-0.5 after:h-3.5 after:w-3.5 after:rounded-full after:bg-zinc-400 after:transition-transform peer-checked:after:translate-x-4 peer-checked:after:bg-white",
    label: "form-check-label text-sm text-zinc-200",
  },
  modal: {
    backdrop: "fixed inset-0 z-50 flex items-center justify-center bg-black/65 p-4 backdrop-blur-xs",
    root: "modal-content relative flex max-h-[calc(100vh-2rem)] w-full flex-col overflow-hidden rounded-2xl border border-blender-border bg-blender-card shadow-2xl",
    size: { sm: "max-w-sm", md: "max-w-lg", lg: "max-w-2xl" },
    header: "modal-header flex items-start justify-between gap-4 border-b border-blender-border px-5 py-4",
    heading: "min-w-0 space-y-1",
    title: "modal-title text-base font-semibold leading-6 text-zinc-100",
    description: "text-xs leading-5 text-zinc-400",
    closeButton: "btn-close -mr-1 -mt-1 rounded-md p-1.5 text-zinc-400 transition-colors hover:bg-blender-hover hover:text-zinc-200",
    body: "modal-body min-h-0 flex-1 overflow-y-auto px-5 py-4 text-sm text-zinc-300",
    footer: "modal-footer flex flex-wrap items-center justify-end gap-2 border-t border-blender-border bg-blender-surface px-5 py-3",
    form: "space-y-4",
  },
} as const;
