<script lang="ts">
  import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.svelte";
  import type { ProductRatingProps } from "../data/types";
  import RatingSummary from "./sections/RatingSummary.svelte";
  import ReviewList from "./sections/ReviewList.svelte";

  let {
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
  }: ProductRatingProps = $props();
</script>

<section class={`space-y-6 ${className}`}>
  {#if titleProps}
    <CardTitle
      {...titleProps}
      className={titleProps.className ?? "text-base font-semibold text-zinc-100 sm:text-lg"}
    />
  {/if}

  <RatingSummary
    {summaryLayout}
    {averageScore}
    {ratingProps}
    {totalReviewsText}
    {recommendedPercent}
    {recommendedLabel}
    {writeReviewButtonProps}
    {breakdown}
  />

  <ReviewList {filterTitle} {resultCountText} {reviews} {loadMoreText} />
</section>
