import { useRef } from "react";
import type { ReactNode } from "react";
import Icon from "../../../../../atoms/icon/ui/Icon";
import type { SectionCarouselButtonsProps } from "../data/types";

type Props = Omit<SectionCarouselButtonsProps, "children"> & { children: ReactNode };

export default function SectionCarouselButtons({ titleProps, descriptionProps, className = "", children }: Props) {
  const viewport = useRef<HTMLDivElement>(null);
  const scroll = (direction: -1 | 1) => viewport.current?.scrollBy({ left: direction * viewport.current.clientWidth * 0.8, behavior: "smooth" });

  return (
    <section className={`w-full min-w-0 py-6 ${className}`.trim()}>
      <header className="mb-6 flex items-end justify-between gap-5">
        <div className="min-w-0">
          {titleProps && <h2 className="text-2xl font-semibold leading-tight text-zinc-100 sm:text-3xl">{titleProps.text}</h2>}
          {descriptionProps && <p className="mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base">{descriptionProps.text}</p>}
        </div>
        <div className="flex shrink-0 items-center gap-2">
          <button type="button" className="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300 transition-colors hover:border-blue-500/60 hover:bg-blender-hover hover:text-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500" aria-label="Itens anteriores" onClick={() => scroll(-1)}><Icon name="ArrowLeft" className="size-4" /></button>
          <button type="button" className="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300 transition-colors hover:border-blue-500/60 hover:bg-blender-hover hover:text-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500" aria-label="Itens seguintes" onClick={() => scroll(1)}><Icon name="ArrowRight" className="size-4" /></button>
        </div>
      </header>
      <div ref={viewport} className="flex w-full snap-x snap-mandatory items-stretch gap-5 overflow-x-auto scroll-smooth [scrollbar-width:none] [&::-webkit-scrollbar]:hidden [&>*]:w-[min(17rem,78vw)] [&>*]:shrink-0 [&>*]:snap-start">
        {children}
      </div>
    </section>
  );
}
