<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import Accordion from "../../../../../molecules/accordion/ui/Accordion.svelte";
  import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton.svelte";
  import CardCartButton from "../../../../../molecules/card/shared/card-cart-button/ui/CardCartButton.svelte";
  import CardFavoriteButton from "../../../../../molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.svelte";
  import CardTags from "../../../../../molecules/card/shared/card-tags/ui/CardTags.svelte";
  import ProductTabs from "../../product-tabs/ui/ProductTabs.svelte";
  import type { ProductInfoActionsProps } from "../data/types";

  let {
    buttonProps, secondaryButtonProps, cartButtonProps, favoriteButtonProps,
    wishlistText, compareText, tagsProps, trustBadges = [], metaItems = [],
    accordionItems = [], jumpLinks = [], inlineTabsProps,
  }: ProductInfoActionsProps = $props();
</script>

<div class="flex flex-wrap items-center gap-2.5 pt-1">
  {#if buttonProps}
    <CardButton {...buttonProps} className={buttonProps.className ?? "flex-1 min-w-[160px] justify-center rounded-lg bg-blue-600 px-5 py-2.5 text-xs font-semibold text-white hover:bg-blue-500"} />
  {/if}
  {#if secondaryButtonProps}
    <CardButton variant="outline" {...secondaryButtonProps} className={secondaryButtonProps.className ?? "flex-1 min-w-[140px] justify-center rounded-lg border border-blender-border bg-blender-surface px-4 py-2.5 text-xs font-semibold text-zinc-200 hover:bg-blender-hover"} />
  {/if}
  {#if cartButtonProps}<CardCartButton {...cartButtonProps} />{/if}
  {#if favoriteButtonProps}<CardFavoriteButton {...favoriteButtonProps} />{/if}
</div>

{#if wishlistText || compareText}
  <div class="flex flex-wrap items-center gap-5 border-b border-blender-border/60 pb-3 text-xs text-zinc-400">
    {#if wishlistText}
      <button type="button" class="inline-flex items-center gap-1.5 hover:text-blue-400"><Icon name="Favorite" className="size-3.5" /><span>{wishlistText}</span></button>
    {/if}
    {#if compareText}
      <button type="button" class="inline-flex items-center gap-1.5 hover:text-blue-400"><Icon name="Refresh" className="size-3.5" /><span>{compareText}</span></button>
    {/if}
  </div>
{/if}

{#if trustBadges.length > 0}
  <div class="grid gap-2 pt-1 sm:grid-cols-3">
    {#each trustBadges as badgeItem (badgeItem.text)}
      <div class="flex items-center gap-2 rounded-lg border border-blender-border/70 bg-blender-surface/50 px-3 py-2 text-[11px] text-zinc-300">
        {#if badgeItem.icon}<Icon name={badgeItem.icon} className="size-4 shrink-0 text-blue-400" />{/if}
        <span>{badgeItem.text}</span>
      </div>
    {/each}
  </div>
{/if}

{#if tagsProps}<CardTags {...tagsProps} />{/if}

{#if metaItems.length > 0}
  <div class="space-y-1.5 border-t border-blender-border/60 pt-3 text-xs">
    {#each metaItems as meta (meta.label)}
      <p class="text-zinc-400"><span class="font-semibold text-zinc-200">{meta.label}:</span><span class="ml-1.5">{meta.value}</span></p>
    {/each}
  </div>
{/if}

{#if accordionItems.length > 0}
  <div class="space-y-2 pt-2">
    {#each accordionItems as acc (acc.title)}
      <Accordion title={acc.title} count={acc.count} open={acc.open}>
        <p class="text-xs leading-relaxed text-zinc-400">{acc.content}</p>
      </Accordion>
    {/each}
  </div>
{/if}

{#if jumpLinks.length > 0}
  <div class="space-y-2 pt-1">
    <span class="block text-[11px] font-medium text-zinc-400">Jump to:</span>
    <div class="flex flex-wrap gap-1.5">
      {#each jumpLinks as link (link)}
        <span class="rounded-full border border-blender-border bg-blender-surface px-2.5 py-1 text-[11px] text-zinc-300">↓ {link}</span>
      {/each}
    </div>
  </div>
{/if}

{#if inlineTabsProps}
  <div class="border-t border-blender-border pt-4">
    <ProductTabs {...inlineTabsProps} />
  </div>
{/if}
