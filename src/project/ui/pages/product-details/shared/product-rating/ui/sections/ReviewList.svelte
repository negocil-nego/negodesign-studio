<script lang="ts">
  import Icon from "../../../../../../atoms/icon/ui/Icon.svelte";
  import CardDescription from "../../../../../../molecules/card/shared/card-description/ui/CardDescription.svelte";
  import CardImage from "../../../../../../molecules/card/shared/card-image/ui/CardImage.svelte";
  import CardRating from "../../../../../../molecules/card/shared/card-rating/ui/CardRating.svelte";
  import type { ProductRatingProps } from "../../data/types";

  let {
    filterTitle = "REFINED BY",
    resultCountText,
    reviews = [],
    loadMoreText,
  }: Pick<
    ProductRatingProps,
    "filterTitle" | "resultCountText" | "reviews" | "loadMoreText"
  > = $props();
</script>

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
                {#if review.verifiedText}<span class="rounded bg-emerald-500/15 px-1.5 py-0.5 text-[10px] font-medium text-emerald-300">{review.verifiedText}</span>{/if}
              </div>
              {#if review.ratingProps}
                <div class="mt-0.5 flex items-center gap-1.5"><CardRating {...review.ratingProps} /></div>
              {/if}
            </div>
          </div>
          {#if review.date}<span class="text-[11px] text-zinc-500">{review.date}</span>{/if}
        </div>

        {#if review.headline}<h5 class="text-xs font-semibold text-zinc-100">{review.headline}</h5>{/if}

        {#if review.descriptionProps}
          <CardDescription {...review.descriptionProps} className="text-xs leading-relaxed text-zinc-300" />
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
