<script lang="ts">
  import Progress from "../../../../../../atoms/progress/ui/Progress.svelte";
  import CardButton from "../../../../../../molecules/card/shared/card-button/ui/CardButton.svelte";
  import CardRating from "../../../../../../molecules/card/shared/card-rating/ui/CardRating.svelte";
  import type { ProductRatingProps } from "../../data/types";

  let {
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
  }: Pick<
    ProductRatingProps,
    | "summaryLayout"
    | "averageScore"
    | "ratingProps"
    | "totalReviewsText"
    | "recommendedPercent"
    | "recommendedLabel"
    | "writeReviewButtonProps"
    | "breakdown"
  > = $props();
</script>

<div class="rounded-2xl border border-blender-border bg-blender-surface/50 p-5">
  {#if summaryLayout === "score-left"}
    <div class="grid gap-6 md:grid-cols-[13rem_minmax(0,1fr)] md:items-center">
      <div class="flex flex-col items-center justify-center border-b border-blender-border pb-4 text-center md:border-b-0 md:border-r md:pb-0 md:pr-6">
        <div class="text-3xl font-bold text-zinc-100">
          {averageScore}
          <span class="text-sm font-normal text-zinc-400">out of 5</span>
        </div>
        <div class="mt-2">
          <CardRating {...ratingProps} />
        </div>
        {#if totalReviewsText}
          <span class="mt-1.5 text-xs text-zinc-400">{totalReviewsText}</span>
        {/if}
      </div>

      <div class="space-y-2.5">
        {#each breakdown as row (row.stars)}
          <div class="grid grid-cols-[3.5rem_minmax(0,1fr)_2.5rem] items-center gap-3 text-xs">
            <span class="font-medium text-zinc-300">{row.label ?? `${row.stars} Star`}</span>
            <Progress value={row.percentage} variant="warning" />
            <span class="text-right tabular-nums text-zinc-400">{row.percentage}%</span>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="space-y-2">
      <span class="text-xs font-semibold uppercase tracking-wider text-zinc-400">Summary</span>
      <div class="grid gap-6 md:grid-cols-12 md:items-center">
        <div class="space-y-2 md:col-span-5">
          {#each breakdown as row (row.stars)}
            <div class="grid grid-cols-[1.5rem_minmax(0,1fr)] items-center gap-2.5 text-xs">
              <span class="font-medium text-zinc-400">{row.stars}</span>
              <Progress value={row.percentage} variant="warning" />
            </div>
          {/each}
        </div>

        <div class="flex items-center gap-6 md:col-span-4">
          <div>
            <div class="flex items-center gap-1.5 text-2xl font-bold text-zinc-100">
              <span>{averageScore}</span>
              <span class="text-amber-400 text-lg">★</span>
            </div>
            <p class="text-[11px] text-zinc-400">{totalReviewsText}</p>
          </div>

          {#if recommendedPercent}
            <div class="border-l border-blender-border pl-6">
              <div class="text-xl font-bold text-zinc-100">{recommendedPercent}</div>
              <p class="text-[11px] text-zinc-400">{recommendedLabel}</p>
            </div>
          {/if}
        </div>

        {#if writeReviewButtonProps}
          <div class="flex flex-col items-start gap-2 md:col-span-3 md:items-end">
            <span class="text-[11px] text-blue-400">Ratings & Reviews Guideline</span>
            <CardButton {...writeReviewButtonProps} />
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
