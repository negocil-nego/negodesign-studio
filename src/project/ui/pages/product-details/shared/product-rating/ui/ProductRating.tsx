// @ts-nocheck
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle";
import type { ProductRatingProps } from "../data/types";
import RatingSummary from "./sections/RatingSummary";
import ReviewList from "./sections/ReviewList";

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

      <RatingSummary
        summaryLayout={summaryLayout}
        averageScore={averageScore}
        ratingProps={ratingProps}
        totalReviewsText={totalReviewsText}
        recommendedPercent={recommendedPercent}
        recommendedLabel={recommendedLabel}
        writeReviewButtonProps={writeReviewButtonProps}
        breakdown={breakdown}
      />

      <ReviewList
        filterTitle={filterTitle}
        resultCountText={resultCountText}
        reviews={reviews}
        loadMoreText={loadMoreText}
      />
    </section>
  );
}
