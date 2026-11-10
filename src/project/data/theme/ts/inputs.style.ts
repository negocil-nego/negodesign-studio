import { DesignTokens } from "./tokens";

export const InputsStyle = {
  input: {
    form: DesignTokens.primitives.inputForm,
    text: {
      label: "form-label mb-1",
      input: DesignTokens.primitives.inputForm,
    },
    username: {
      field: "w-full min-w-0",
      control: "relative flex w-full min-w-0 items-center",
      controlWithCountry: "pl-16",
      countryOverlay: "absolute left-1.5 top-1/2 z-10 -translate-y-1/2",
      hint: "form-text mt-1 text-xs text-rose-400",
    },
    otp: {
      field: "w-full min-w-0",
      label: "form-label mb-1",
      group: "grid w-full grid-cols-6 gap-2",
      slot: `${DesignTokens.primitives.inputForm} h-11 justify-center px-0 text-center text-base font-semibold tracking-widest tabular-nums`,
    },
    range: {
      field: "w-full min-w-0 space-y-1.5",
      header: "flex items-center justify-between gap-2",
      label: "form-label mb-0 text-xs font-medium text-zinc-300",
      valueBadge: "badge text-bg-secondary px-2 py-0.5 text-[11px] tabular-nums",
      input: "form-range w-full",
      hint: "form-text text-xs text-zinc-400",
    },
  },
  form: {
    default: "w-full space-y-4",
    fields: "grid grid-cols-1 gap-4",
    item: "min-w-0 space-y-1.5",
    actions: "flex items-center justify-end gap-2 pt-2",
  },
  countryPopover: {
    root: "relative inline-block",
    trigger:
      "inline-flex items-center gap-1.5 rounded border border-transparent px-1.5 py-1 text-xs font-medium text-zinc-300 hover:bg-blender-hover focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:cursor-not-allowed disabled:opacity-50",
    flag: "inline-flex h-4 w-5 shrink-0 items-center justify-center overflow-hidden rounded-[2px]",
    dialCode: "tabular-nums text-zinc-300",
    chevron: "size-3 text-zinc-400",
    panel:
      "dropdown-menu absolute left-0 top-full z-50 mt-1.5 w-64 rounded-lg border border-blender-border bg-blender-card p-2 shadow-xl",
    search: `${DesignTokens.primitives.inputForm} mb-2 py-1.5 text-xs`,
    list: "max-h-56 overflow-y-auto divide-y divide-blender-border/60",
    option:
      "dropdown-item flex w-full items-center gap-2.5 rounded px-2 py-1.5 text-left text-xs text-zinc-300 hover:bg-blender-hover",
    optionActive: "bg-blue-600/20 font-medium text-blue-400",
    optionName: "min-w-0 flex-1 truncate",
    optionDialCode: "shrink-0 tabular-nums text-zinc-400",
    empty: "py-4 text-center text-xs text-zinc-500",
  },
} as const;
