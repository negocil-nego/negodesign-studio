// @ts-nocheck
import Progress from "../../../../../atoms/progress/ui/Progress";
import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton";
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription";
import CardImage from "../../../../../molecules/card/shared/card-image/ui/CardImage";
import CardRating from "../../../../../molecules/card/shared/card-rating/ui/CardRating";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle";
import type { ProductRatingProps } from "../data/types";

export default function ProductRating({
  titleProps,
  summaryLayout = "bars-left",
  averageScore = "4.5/5",
  ratingProps = { rating: 5 },
  totalReviewsText = "150 Reviews",
  recommendedPercent,
  recommendedLabel = "Recommended",
  writeReviewButtonProps,
  breakdown = [
    { stars: 5, label: "5 Star", percentage: 78 },
    { stars: 4, label: "4 Star", percentage: 52 },
    { stars: 3, label: "3 Star", percentage: 28 },
    { stars: 2, label: "2 Star", percentage: 14 },
    { stars: 1, label: "1 Star", percentage: 6 },
  ],
  filterTitle = "REFINED BY",
  resultCountText,
  reviews = [],
  loadMoreText,
  className = "",
}: ProductRatingProps) {
  return (
    <section className={`space-y-6 ${className}`}>
      {titleProps && <CardTitle {...titleProps} />}

      <div className="rounded-2xl border border-blender-border bg-blender-surface/50 p-5">
        {summaryLayout === "score-left" ? (
          <div className="grid gap-6 md:grid-cols-[13rem_minmax(0,1fr)] md:items-center">
            <div className="flex flex-col items-center justify-center text-center md:border-r md:border-blender-border md:pr-6">
              <div className="text-3xl font-bold text-zinc-100">
                {averageScore} <span className="text-sm font-normal text-zinc-400">out of 5</span>
              </div>
              <div className="mt-2">
                <CardRating {...ratingProps} />
              </div>
              {totalReviewsText && <span className="mt-1.5 text-xs text-zinc-400">{totalReviewsText}</span>}
            </div>
            <div className="space-y-2.5">
              {breakdown.map((row) => (
                <div key={row.stars} className="grid grid-cols-[3.5rem_minmax(0,1fr)_2.5rem] items-center gap-3 text-xs">
                  <span className="font-medium text-zinc-300">{row.label ?? `${row.stars} Star`}</span>
                  <Progress value={row.percentage} variant="warning" />
                  <span className="text-right tabular-nums text-zinc-400">{row.percentage}%</span>
                </div>
              ))}
            </div>
          </div>
        ) : (
          <div className="grid gap-6 md:grid-cols-12 md:items-center">
            <div className="space-y-2 md:col-span-5">
              {breakdown.map((row) => (
                <div key={row.stars} className="grid grid-cols-[1.5rem_minmax(0,1fr)] items-center gap-2.5 text-xs">
                  <span className="font-medium text-zinc-400">{row.stars}</span>
                  <Progress value={row.percentage} variant="warning" />
                </div>
              ))}
            </div>
            <div className="flex items-center gap-6 md:col-span-4">
              <div>
                <div className="text-2xl font-bold text-zinc-100">{averageScore} ★</div>
                <p className="text-[11px] text-zinc-400">{totalReviewsText}</p>
              </div>
              {recommendedPercent && (
                <div className="border-l border-blender-border pl-6">
                  <div className="text-xl font-bold text-zinc-100">{recommendedPercent}</div>
                  <p className="text-[11px] text-zinc-400">{recommendedLabel}</p>
                </div>
              )}
            </div>
            {writeReviewButtonProps && (
              <div className="md:col-span-3 md:text-right">
                <CardButton {...writeReviewButtonProps} />
              </div>
            )}
          </div>
        )}
      </div>

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
    </section>
  );
}
