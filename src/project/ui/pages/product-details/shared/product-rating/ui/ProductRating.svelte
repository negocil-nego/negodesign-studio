<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import Progress from "../../../../../atoms/progress/ui/Progress.svelte";
  import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton.svelte";
  import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.svelte";
  import CardImage from "../../../../../molecules/card/shared/card-image/ui/CardImage.svelte";
  import CardRating from "../../../../../molecules/card/shared/card-rating/ui/CardRating.svelte";
  import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.svelte";
  import type { ProductRatingProps } from "../data/types";

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

  <div class="flex flex-wrap items-center justify-between gap-3 border-b border-blender-border pb-3">
    <div>
      <h4 class="text-xs font-semibold uppercase tracking-wider text-zinc-200">
        {filterTitle}
      </h4>
      {#if resultCountText}
        <p class="mt-0.5 text-[11px] text-zinc-400">{resultCountText}</p>
      {/if}
    </div>

    <div class="flex flex-wrap items-center gap-2">
      <select class="rounded-lg border border-blender-border bg-blender-surface px-3 py-1.5 text-xs text-zinc-200">
        <option>All Star</option>
        <option>5 Stars</option>
        <option>4 Stars</option>
        <option>3 Stars</option>
      </select>
      <select class="rounded-lg border border-blender-border bg-blender-surface px-3 py-1.5 text-xs text-zinc-200">
        <option>Sort by: Newest</option>
        <option>Sort by: Relevance</option>
        <option>Sort by: Highest Rating</option>
      </select>
    </div>
  </div>

  {#if reviews.length > 0}
    <div class="divide-y divide-blender-border/60">
      {#each reviews as review, idx (review.id ?? idx)}
        <article class="space-y-3 py-5 first:pt-1 last:pb-1">
          <div class="flex items-start justify-between gap-3">
            <div class="flex items-center gap-3">
              <img
                src={review.avatarSrc ?? "/favicon.png"}
                alt={review.author}
                class="size-9 rounded-full border border-blender-border bg-blender-surface object-cover"
              />
              <div>
                <div class="flex items-center gap-2">
                  <span class="text-xs font-semibold text-zinc-100">{review.author}</span>
                  {#if review.verifiedText}
                    <span class="rounded bg-emerald-500/15 px-1.5 py-0.5 text-[10px] font-medium text-emerald-300">
                      {review.verifiedText}
                    </span>
                  {/if}
                </div>
                {#if review.ratingProps}
                  <div class="mt-0.5 flex items-center gap-1.5">
                    <CardRating {...review.ratingProps} />
                  </div>
                {/if}
              </div>
            </div>
            {#if review.date}
              <span class="text-[11px] text-zinc-500">{review.date}</span>
            {/if}
          </div>

          {#if review.headline}
            <h5 class="text-xs font-semibold text-zinc-100">{review.headline}</h5>
          {/if}

          {#if review.descriptionProps}
            <CardDescription
              {...review.descriptionProps}
              className="text-xs leading-relaxed text-zinc-300"
            />
          {/if}

          {#if review.mediaImages && review.mediaImages.length > 0}
            <div class="flex flex-wrap gap-2.5 pt-1">
              {#each review.mediaImages as img, imgIdx (imgIdx)}
                <div class="size-20 overflow-hidden rounded-xl border border-blender-border bg-blender-surface">
                  <CardImage {...img} className="h-full w-full object-cover" />
                </div>
              {/each}
            </div>
          {/if}
        </article>
      {/each}
    </div>
  {/if}

  {#if loadMoreText}
    <div class="pt-1">
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-lg border border-blender-border bg-blender-surface px-4 py-2 text-xs font-semibold text-zinc-300 transition hover:border-blue-500/50 hover:text-zinc-100"
      >
        <Icon name="Refresh" className="size-3.5" />
        <span>{loadMoreText}</span>
      </button>
    </div>
  {/if}
</section>
