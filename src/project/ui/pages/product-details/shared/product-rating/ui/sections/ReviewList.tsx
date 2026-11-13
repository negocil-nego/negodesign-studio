// @ts-nocheck
import CardDescription from "../../../../../../molecules/card/shared/card-description/ui/CardDescription";
import CardImage from "../../../../../../molecules/card/shared/card-image/ui/CardImage";
import CardRating from "../../../../../../molecules/card/shared/card-rating/ui/CardRating";

export default function ReviewList({
  filterTitle = "REFINED BY",
  resultCountText,
  reviews = [],
  loadMoreText,
}: {
  filterTitle?: string;
  resultCountText?: string;
  reviews?: any[];
  loadMoreText?: string;
}) {
  return (
    <>
      <div className="flex flex-wrap items-center justify-between gap-3 border-b border-blender-border pb-3">
        <div>
          <h4 className="text-xs font-semibold uppercase tracking-wider text-zinc-200">{filterTitle}</h4>
          {resultCountText && <p className="mt-0.5 text-[11px] text-zinc-400">{resultCountText}</p>}
        </div>
      </div>

      {reviews.length > 0 && (
        <div className="divide-y divide-blender-border/60">
          {reviews.map((review, idx) => (
            <article key={review.id ?? idx} className="space-y-3 py-5">
              <div className="flex items-start justify-between gap-3">
                <div className="flex items-center gap-3">
                  <img
                    src={review.avatarSrc ?? "/favicon.png"}
                    alt={review.author}
                    className="size-9 rounded-full border border-blender-border bg-blender-surface object-cover"
                  />
                  <div>
                    <span className="text-xs font-semibold text-zinc-100">{review.author}</span>
                    {review.ratingProps && <CardRating {...review.ratingProps} />}
                  </div>
                </div>
                {review.date && <span className="text-[11px] text-zinc-500">{review.date}</span>}
              </div>
              {review.headline && <h5 className="text-xs font-semibold text-zinc-100">{review.headline}</h5>}
              {review.descriptionProps && <CardDescription {...review.descriptionProps} />}
              {review.mediaImages && review.mediaImages.length > 0 && (
                <div className="flex flex-wrap gap-2.5 pt-1">
                  {review.mediaImages.map((img, i) => (
                    <div key={i} className="size-20 overflow-hidden rounded-xl border border-blender-border bg-blender-surface">
                      <CardImage {...img} className="h-full w-full object-cover" />
                    </div>
                  ))}
                </div>
              )}
            </article>
          ))}
        </div>
      )}

      {loadMoreText && (
        <button
          type="button"
          className="rounded-lg border border-blender-border bg-blender-surface px-4 py-2 text-xs font-semibold text-zinc-300"
        >
          {loadMoreText}
        </button>
      )}
    </>
  );
}
