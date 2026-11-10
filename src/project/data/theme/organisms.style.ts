import { DataTableStyle } from "./datatable.style";

export const OrganismsStyle = {
  carousel: {
    root: "relative w-full overflow-hidden rounded-xl border border-blender-border bg-blender-card",
    slide: "relative flex aspect-[4/5] min-h-[28rem] w-full items-end justify-center sm:aspect-[5/6]",
    backgroundImage: "absolute inset-0 bg-cover bg-center bg-no-repeat",
    backgroundGradient: "absolute inset-0",
    overlay: "pointer-events-none absolute inset-0 bg-gradient-to-b from-transparent via-black/15 to-black/70",
    content: "relative z-10 mx-auto flex w-full max-w-xl flex-col items-center px-6 pb-14 text-center text-white sm:px-10 sm:pb-16",
    title: "text-2xl font-semibold leading-tight drop-shadow-md sm:text-3xl",
    description: "mt-2 max-w-lg text-sm leading-relaxed text-white/80 drop-shadow sm:text-base",
    indicators: "absolute inset-x-0 bottom-5 z-20 flex items-center justify-center gap-2",
    indicator: "h-2 w-2 rounded-full border border-white/80 bg-black/25 transition-all hover:bg-white/70 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white",
    indicatorActive: "w-5 bg-white",
  },
  sectionCarouselButtons: {
    root: "w-full min-w-0 py-6",
    header: "mb-6 flex items-end justify-between gap-5",
    heading: "min-w-0",
    title: "text-2xl font-semibold leading-tight text-zinc-100 sm:text-3xl",
    description: "mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base",
    controls: "flex shrink-0 items-center gap-2",
    control: "flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300 transition-colors hover:border-blue-500/60 hover:bg-blender-hover hover:text-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 disabled:cursor-not-allowed disabled:opacity-40",
    viewport: "min-w-0 overflow-hidden",
    track: "flex w-full snap-x snap-mandatory items-stretch gap-5 overflow-x-auto scroll-smooth [scrollbar-width:none] [&::-webkit-scrollbar]:hidden [&>*]:w-[calc((100%_-_3.75rem)_/_4)] [&>*]:shrink-0 [&>*]:snap-start",
  },
  accordion: {
    root: "overflow-hidden rounded-xl border border-blender-border bg-blender-card",
    summary: "flex cursor-pointer list-none items-center justify-between gap-3 px-4 py-3 text-sm font-semibold text-zinc-100 transition-colors hover:bg-blender-hover [&::-webkit-details-marker]:hidden",
    meta: "flex items-center gap-2 text-xs font-normal text-zinc-400",
    indicator: "transition-transform details-open:rotate-180",
    content: "border-t border-blender-border p-4",
  },
  dataTable: DataTableStyle,
} as const;
