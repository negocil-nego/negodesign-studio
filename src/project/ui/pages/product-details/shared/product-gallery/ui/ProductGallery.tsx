// @ts-nocheck
import { useState } from "react";
import Icon from "../../../../../atoms/icon/ui/Icon";
import CardImage from "../../../../../molecules/card/shared/card-image/ui/CardImage";
import CardMinFile from "../../../../../molecules/card/shared/card-min-file/ui/CardMinFile";
import CardThumbnailVideo from "../../../../../molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo";
import CardVideo from "../../../../../molecules/card/shared/card-video/ui/CardVideo";
import type { ProductGalleryProps } from "../data/types";

export default function ProductGallery({
  layout = "bottom-thumbs",
  badge,
  showArrows = false,
  imageProps,
  videoProps,
  thumbnailVideoProps,
  minFileProps = [],
  galleryImages = [],
  className = "",
}: ProductGalleryProps) {
  const [activeIdx, setActiveIdx] = useState(0);

  const allThumbs =
    minFileProps.length > 0
      ? minFileProps
      : galleryImages.map((img, i) => ({
          imageProps: img,
          active: i === activeIdx,
        }));

  const currentImage = allThumbs[activeIdx]?.imageProps ??
    imageProps ?? {
      src: "/img/ux-01.jpg",
      alt: "Product preview",
    };

  const prevSlide = () => {
    if (!allThumbs.length) return;
    setActiveIdx((activeIdx - 1 + allThumbs.length) % allThumbs.length);
  };

  const nextSlide = () => {
    if (!allThumbs.length) return;
    setActiveIdx((activeIdx + 1) % allThumbs.length);
  };

  if (layout === "grid-gallery") {
    return (
      <div className={`space-y-4 ${className}`}>
        <div className="relative aspect-[4/3] w-full overflow-hidden rounded-2xl border border-blender-border bg-blender-surface">
          {badge && (
            <span className="absolute left-3 top-3 z-10 rounded-full border border-blue-500/40 bg-blue-500/15 px-3 py-1 text-xs font-semibold text-blue-300">
              {badge}
            </span>
          )}
          <CardImage {...currentImage} className="h-full w-full object-cover" />
        </div>
        {allThumbs.length > 0 && (
          <div className="grid grid-cols-2 gap-3">
            {allThumbs.map((thumb, idx) =>
              thumb.imageProps ? (
                <button
                  key={idx}
                  type="button"
                  onClick={() => setActiveIdx(idx)}
                  className={`group relative aspect-square overflow-hidden rounded-xl border bg-blender-surface transition ${
                    activeIdx === idx
                      ? "border-blue-500 ring-2 ring-blue-500/30"
                      : "border-blender-border hover:border-blue-500/50"
                  }`}
                >
                  <CardImage {...thumb.imageProps} className="h-full w-full object-cover" />
                </button>
              ) : null,
            )}
          </div>
        )}
      </div>
    );
  }

  if (layout === "left-thumbs") {
    return (
      <div className={`grid grid-cols-1 gap-4 sm:grid-cols-[5.5rem_minmax(0,1fr)] ${className}`}>
        {allThumbs.length > 0 && (
          <div className="order-2 flex gap-2.5 overflow-x-auto sm:order-1 sm:flex-col">
            {allThumbs.map((thumb, idx) => (
              <div key={idx} className="size-20 shrink-0">
                <CardMinFile
                  imageProps={thumb.imageProps}
                  active={activeIdx === idx}
                  onClick={() => setActiveIdx(idx)}
                />
              </div>
            ))}
          </div>
        )}
        <div className="order-1 relative aspect-square w-full overflow-hidden rounded-2xl border border-blender-border bg-blender-surface sm:order-2">
          {badge && (
            <span className="absolute left-3.5 top-3.5 z-10 rounded-full border border-blue-500/40 bg-blue-500/15 px-3 py-1 text-xs font-semibold text-blue-300">
              {badge}
            </span>
          )}
          {thumbnailVideoProps ? (
            <CardThumbnailVideo {...thumbnailVideoProps} />
          ) : videoProps ? (
            <CardVideo {...videoProps} controls={false} />
          ) : (
            <CardImage {...currentImage} className="h-full w-full object-cover" />
          )}
        </div>
      </div>
    );
  }

  return (
    <div className={`space-y-3.5 ${className}`}>
      <div className="relative aspect-[4/3.6] w-full overflow-hidden rounded-2xl border border-blender-border bg-blender-surface">
        {badge && (
          <span className="absolute left-3.5 top-3.5 z-10 rounded-full border border-blue-500/40 bg-blue-500/15 px-3 py-1 text-xs font-semibold text-blue-300">
            {badge}
          </span>
        )}
        <CardImage {...currentImage} className="h-full w-full object-cover" />
        {showArrows && allThumbs.length > 1 && (
          <>
            <button
              type="button"
              onClick={prevSlide}
              className="absolute left-3 top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-lg border border-blender-border bg-blender-card/90 text-zinc-200"
            >
              <Icon name="ChevronLeft" className="size-4" />
            </button>
            <button
              type="button"
              onClick={nextSlide}
              className="absolute right-3 top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-lg border border-blender-border bg-blender-card/90 text-zinc-200"
            >
              <Icon name="ChevronRight" className="size-4" />
            </button>
          </>
        )}
      </div>
      {allThumbs.length > 0 && (
        <div className="grid grid-cols-4 gap-2.5 sm:grid-cols-5">
          {allThumbs.map((thumb, idx) => (
            <button
              key={idx}
              type="button"
              onClick={() => setActiveIdx(idx)}
              className={`relative aspect-square overflow-hidden rounded-xl border bg-blender-surface transition ${
                activeIdx === idx
                  ? "border-blue-500 ring-2 ring-blue-500/30"
                  : "border-blender-border hover:border-blue-500/50"
              }`}
            >
              {thumb.imageProps && <CardImage {...thumb.imageProps} className="h-full w-full object-cover" />}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
