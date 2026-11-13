// @ts-nocheck
import CardImage from "../../../../../../molecules/card/shared/card-image/ui/CardImage";

export default function ProductGalleryGrid({
  currentImage,
  allThumbs = [],
  activeIdx = 0,
  badge,
  className = "",
  onSelect,
}: {
  currentImage: any;
  allThumbs?: any[];
  activeIdx?: number;
  badge?: string;
  className?: string;
  onSelect: (idx: number) => void;
}) {
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
                onClick={() => onSelect(idx)}
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
