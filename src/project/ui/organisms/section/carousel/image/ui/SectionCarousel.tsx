import { useState } from "react";
import { getCarouselBackground } from "../data/background";
import type { SectionImageCarouselProps } from "../data/types";

export default function SectionCarousel({ items = [], className = "", backgroundColor = "" }: SectionImageCarouselProps) {
  const [activeIndex, setActiveIndex] = useState(0);
  const currentIndex = items.length ? Math.min(activeIndex, items.length - 1) : 0;
  const activeItem = items[currentIndex];
  if (!activeItem) return null;
  const background = getCarouselBackground(backgroundColor);

  return (
    <section
      className={`relative w-full overflow-hidden rounded-xl border border-blender-border bg-blender-card ${background.backgroundClass} ${className}`.trim()}
      style={background.isCssColor ? { backgroundColor } : undefined}
      aria-label="Carousel"
      aria-roledescription="carousel"
    >
      <div
        className="relative flex aspect-[4/5] min-h-[28rem] w-full items-end justify-center sm:aspect-[5/6]"
        aria-live="polite"
        aria-atomic="true"
        role="group"
        aria-roledescription="slide"
        aria-label={`Slide ${currentIndex + 1} de ${items.length}`}
      >
        {activeItem.backgroundImage && <div className="absolute inset-0 bg-cover bg-center bg-no-repeat" style={{ backgroundImage: `url("${activeItem.backgroundImage}")` }} aria-hidden="true" />}
        {(background.isCssGradient || background.isGradientClass) && <div className={`absolute inset-0 ${background.backgroundGradientClass}`} style={background.isCssGradient ? { backgroundImage: backgroundColor } : undefined} aria-hidden="true" />}
        <div className="pointer-events-none absolute inset-0 bg-gradient-to-b from-transparent via-black/15 to-black/70" aria-hidden="true" />
        <div className="relative z-10 mx-auto flex w-full max-w-xl flex-col items-center px-6 pb-14 text-center text-white sm:px-10 sm:pb-16">
          {activeItem.title?.text && <h2 className="text-2xl font-semibold leading-tight drop-shadow-md sm:text-3xl">{activeItem.title.text}</h2>}
          {activeItem.description?.text && <p className="mt-2 max-w-lg text-sm leading-relaxed text-white/80 drop-shadow sm:text-base">{activeItem.description.text}</p>}
        </div>
        {items.length > 1 && (
          <div className="absolute inset-x-0 bottom-5 z-20 flex items-center justify-center gap-2" aria-label="Slides">
            {items.map((_, index) => (
              <button
                key={index}
                type="button"
                className={`h-2 rounded-full border border-white/80 bg-black/25 transition-all hover:bg-white/70 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white ${currentIndex === index ? "w-5 bg-white" : "w-2"}`}
                aria-label={`Mostrar slide ${index + 1}`}
                aria-current={currentIndex === index ? "true" : undefined}
                onClick={() => setActiveIndex(index)}
              />
            ))}
          </div>
        )}
      </div>
    </section>
  );
}
