import { useRef } from "react";
import type { ReactNode } from "react";
import Icon from "../../../../atoms/icon/ui/Icon";
import type { SectionGridCarouselProps } from "../data/types";
type Props = Omit<SectionGridCarouselProps, "children"> & { children: ReactNode[] };
export default function SectionGridCarousel({ titleProps, descriptionProps, className = "", rows = 1, columns = 4, children }: Props) {
  const viewport = useRef<HTMLDivElement>(null);
  const rowCount = Math.max(1, Math.floor(rows));
  const columnCount = Math.max(1, Math.floor(columns));
  const size = rowCount * columnCount;
  const groups = Array.from({ length: Math.ceil(children.length / size) }, (_, index) => children.slice(index * size, (index + 1) * size));
  const scroll = (direction: -1 | 1) => viewport.current?.scrollBy({ left: direction * viewport.current.clientWidth, behavior: "smooth" });
  return (
    <section className={`w-full min-w-0 py-6 ${className}`.trim()}>
      <header className="mb-4 flex items-end justify-between gap-4">
        <div className="min-w-0">
          {titleProps && <h2 className="text-2xl font-semibold leading-tight text-zinc-100 sm:text-3xl">{titleProps.text}</h2>}
          {descriptionProps && <p className="mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base">{descriptionProps.text}</p>}
        </div>
        <div className="flex shrink-0 gap-2">
          <button type="button" aria-label="Itens anteriores" onClick={() => scroll(-1)} className="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300">
            <Icon name="ArrowLeft" className="size-4" />
          </button>
          <button type="button" aria-label="Itens seguintes" onClick={() => scroll(1)} className="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300">
            <Icon name="ArrowRight" className="size-4" />
          </button>
        </div>
      </header>
      <div ref={viewport} className="flex snap-x snap-mandatory gap-2 overflow-x-auto scroll-smooth [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
        {groups.map((group, index) => (
          <div
            key={index}
            className="grid w-full shrink-0 snap-start gap-2 [&>*]:w-full [&>*]:min-w-0"
            style={{
              gridTemplateRows: `repeat(${rowCount}, minmax(0, 1fr))`,
              gridTemplateColumns: `repeat(${columnCount}, minmax(0, 1fr))`,
            }}
          >
            {group}
          </div>
        ))}
      </div>
    </section>
  );
}
