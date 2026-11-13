// @ts-nocheck
import Icon from "../../../../../../atoms/icon/ui/Icon";
import CardImage from "../../../../../../molecules/card/shared/card-image/ui/CardImage";

export default function ProductGalleryBottom({
  currentImage,
  allThumbs = [],
  activeIdx = 0,
  badge,
  showArrows = false,
  className = "",
  onSelect,
  onPrev,
  onNext,
}: {
  currentImage: any;
  allThumbs?: any[];
  activeIdx?: number;
  badge?: string;
  showArrows?: boolean;
  className?: string;
  onSelect: (idx: number) => void;
  onPrev: () => void;
  onNext: () => void;
}) {
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
              onClick={onPrev}
              className="absolute left-3 top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-lg border border-blender-border bg-blender-card/90 text-zinc-200"
            >
              <Icon name="ChevronLeft" className="size-4" />
            </button>
            <button
              type="button"
              onClick={onNext}
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
              onClick={() => onSelect(idx)}
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
