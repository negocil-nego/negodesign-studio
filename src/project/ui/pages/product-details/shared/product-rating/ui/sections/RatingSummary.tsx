// @ts-nocheck
import Progress from "../../../../../../atoms/progress/ui/Progress";
import CardButton from "../../../../../../molecules/card/shared/card-button/ui/CardButton";
import CardRating from "../../../../../../molecules/card/shared/card-rating/ui/CardRating";

export default function RatingSummary({
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
}: {
  summaryLayout?: "bars-left" | "score-left";
  averageScore?: string;
  ratingProps?: any;
  totalReviewsText?: string;
  recommendedPercent?: string;
  recommendedLabel?: string;
  writeReviewButtonProps?: any;
  breakdown?: any[];
}) {
  return (
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
  );
}
