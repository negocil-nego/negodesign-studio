// @ts-nocheck
import CardImage from "../../../../../../molecules/card/shared/card-image/ui/CardImage";
import CardMinFile from "../../../../../../molecules/card/shared/card-min-file/ui/CardMinFile";
import CardThumbnailVideo from "../../../../../../molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo";
import CardVideo from "../../../../../../molecules/card/shared/card-video/ui/CardVideo";

export default function ProductGalleryLeft({
  currentImage,
  allThumbs = [],
  activeIdx = 0,
  badge,
  videoProps,
  thumbnailVideoProps,
  className = "",
  onSelect,
}: {
  currentImage: any;
  allThumbs?: any[];
  activeIdx?: number;
  badge?: string;
  videoProps?: any;
  thumbnailVideoProps?: any;
  className?: string;
  onSelect: (idx: number) => void;
}) {
  return (
    <div className={`grid grid-cols-1 gap-4 sm:grid-cols-[5.5rem_minmax(0,1fr)] ${className}`}>
      {allThumbs.length > 0 && (
        <div className="order-2 flex gap-2.5 overflow-x-auto sm:order-1 sm:flex-col">
          {allThumbs.map((thumb, idx) => (
            <div key={idx} className="size-20 shrink-0">
              <CardMinFile
                imageProps={thumb.imageProps}
                active={activeIdx === idx}
                onClick={() => onSelect(idx)}
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
