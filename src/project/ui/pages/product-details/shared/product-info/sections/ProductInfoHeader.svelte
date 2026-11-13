<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.svelte";
  import CardPrice from "../../../../../molecules/card/shared/card-price/ui/CardPrice.svelte";
  import CardRating from "../../../../../molecules/card/shared/card-rating/ui/CardRating.svelte";
  import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.svelte";
  import type { ProductInfoHeaderProps } from "../data/types";

  let {
    category, badge, stockBadge, titleProps, subtitle, ratingProps, reviewCountText,
    priceProps, priceUnit, oldPrice, discountText, socialProofText, descriptionProps,
    featuresTitle = "Features:", features = [], calloutText,
  }: ProductInfoHeaderProps = $props();
</script>

<div class="space-y-2">
  {#if category || badge || stockBadge}
    <div class="flex flex-wrap items-center justify-between gap-2">
      <div class="flex flex-wrap items-center gap-2">
        {#if category}<span class="text-xs font-medium uppercase tracking-wider text-blue-400">{category}</span>{/if}
        {#if badge}<span class="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-emerald-300">{badge}</span>{/if}
      </div>
      {#if stockBadge}<span class="rounded-md border border-blue-500/40 bg-blue-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-blue-300">{stockBadge}</span>{/if}
    </div>
  {/if}
  {#if titleProps}
    <CardTitle {...titleProps} className={titleProps.className ?? "text-2xl font-bold tracking-tight text-zinc-100 sm:text-3xl"} />
  {/if}
  <div class="flex flex-wrap items-center gap-3 text-xs text-zinc-400">
    {#if subtitle}
      <span>{subtitle}</span>
      <span class="text-zinc-600">|</span>
    {/if}
    {#if ratingProps}
      <div class="flex items-center gap-2">
        <CardRating {...ratingProps} />
        <span class="font-semibold text-zinc-200">{ratingProps.rating}</span>
        {#if reviewCountText}<span class="text-zinc-400">({reviewCountText})</span>{/if}
      </div>
    {/if}
  </div>
</div>
{#if priceProps || oldPrice || discountText}
  <div class="space-y-1">
    <div class="flex flex-wrap items-baseline gap-2.5">
      {#if priceProps}<CardPrice {...priceProps} />{/if}
      {#if priceUnit}<span class="text-xs text-zinc-400">{priceUnit}</span>{/if}
      {#if oldPrice}<span class="text-sm text-zinc-500 line-through">{oldPrice}</span>{/if}
    </div>
    {#if discountText}
      <p class="inline-flex items-center gap-1.5 text-xs font-medium text-emerald-400">
        <Icon name="Tag" className="size-3.5" />
        <span>{discountText}</span>
      </p>
    {/if}
  </div>
{/if}
{#if socialProofText}
  <div class="inline-flex items-center gap-2 rounded-lg border border-blender-border bg-blender-surface px-3 py-1.5 text-xs text-zinc-300">
    <span class="size-2 rounded-full bg-emerald-400"></span>
    <span>{socialProofText}</span>
  </div>
{/if}
{#if descriptionProps}
  <CardDescription {...descriptionProps} className={descriptionProps.className ?? "text-xs leading-relaxed text-zinc-400"} />
{/if}
{#if features.length > 0}
  <div class="space-y-2">
    {#if featuresTitle}<h4 class="text-xs font-semibold text-zinc-200">{featuresTitle}</h4>{/if}
    <ul class="space-y-1.5 text-xs text-zinc-300">
      {#each features as feat (feat)}
        <li class="flex items-center gap-2">
          <span class="flex size-4 shrink-0 items-center justify-center rounded-full bg-blue-500/20 text-blue-400 text-[10px]">✓</span>
          <span>{feat}</span>
        </li>
      {/each}
    </ul>
  </div>
{/if}
{#if calloutText}
  <div class="flex items-start gap-2.5 rounded-xl border border-blender-border bg-blender-surface/70 p-3 text-xs text-zinc-300">
    <Icon name="HelpCircle" className="size-4 shrink-0 text-blue-400 mt-0.5" />
    <span>{calloutText}</span>
  </div>
{/if}
