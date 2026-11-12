<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import Accordion from "../../../../../molecules/accordion/ui/Accordion.svelte";
  import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton.svelte";
  import CardCartButton from "../../../../../molecules/card/shared/card-cart-button/ui/CardCartButton.svelte";
  import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.svelte";
  import CardFavoriteButton from "../../../../../molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.svelte";
  import CardPrice from "../../../../../molecules/card/shared/card-price/ui/CardPrice.svelte";
  import CardRating from "../../../../../molecules/card/shared/card-rating/ui/CardRating.svelte";
  import CardTags from "../../../../../molecules/card/shared/card-tags/ui/CardTags.svelte";
  import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.svelte";
  import ProductTabs from "../../product-tabs/ui/ProductTabs.svelte";
  import type { ProductInfoProps } from "../data/types";

  let {
    category,
    badge,
    stockBadge,
    titleProps,
    subtitle,
    ratingProps,
    reviewCountText,
    priceProps,
    priceUnit,
    oldPrice,
    discountText,
    socialProofText,
    descriptionProps,
    featuresTitle = "Features:",
    features = [],
    colorsLabel = "Colors",
    colors = [],
    sizesLabel = "Size",
    sizes = [],
    purchaseOptions = [],
    showQuantity = false,
    initialQuantity = 1,
    stockCountText,
    subtotalText,
    deliveryText,
    urgencyText,
    calloutText,
    buttonProps,
    secondaryButtonProps,
    cartButtonProps,
    favoriteButtonProps,
    wishlistText,
    compareText,
    tagsProps,
    trustBadges = [],
    metaItems = [],
    accordionItems = [],
    jumpLinks = [],
    inlineTabsProps,
    className = "",
  }: ProductInfoProps = $props();

  // svelte-ignore state_referenced_locally
  let quantity = $state(initialQuantity);
  let selectedColor = $state("");
  let selectedSize = $state("");
  let selectedPurchase = $state("");

  const activeColor = $derived(selectedColor || colors[0]?.value || "");
  const activeSize = $derived(selectedSize || sizes[0]?.value || "");
  const activePurchase = $derived(selectedPurchase || purchaseOptions[0]?.id || "");
</script>

<div class={`space-y-5 ${className}`}>
  <div class="space-y-2">
    {#if category || badge || stockBadge}
      <div class="flex flex-wrap items-center justify-between gap-2">
        <div class="flex flex-wrap items-center gap-2">
          {#if category}
            <span class="text-xs font-medium uppercase tracking-wider text-blue-400">
              {category}
            </span>
          {/if}
          {#if badge}
            <span class="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-emerald-300">
              {badge}
            </span>
          {/if}
        </div>
        {#if stockBadge}
          <span class="rounded-md border border-blue-500/40 bg-blue-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-blue-300">
            {stockBadge}
          </span>
        {/if}
      </div>
    {/if}

    {#if titleProps}
      <CardTitle
        {...titleProps}
        className={titleProps.className ?? "text-2xl font-bold tracking-tight text-zinc-100 sm:text-3xl"}
      />
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
          {#if reviewCountText}
            <span class="text-zinc-400">({reviewCountText})</span>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  {#if priceProps || oldPrice || discountText}
    <div class="space-y-1">
      <div class="flex flex-wrap items-baseline gap-2.5">
        {#if priceProps}
          <CardPrice {...priceProps} />
        {/if}
        {#if priceUnit}
          <span class="text-xs text-zinc-400">{priceUnit}</span>
        {/if}
        {#if oldPrice}
          <span class="text-sm text-zinc-500 line-through">{oldPrice}</span>
        {/if}
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
    <CardDescription
      {...descriptionProps}
      className={descriptionProps.className ?? "text-xs leading-relaxed text-zinc-400"}
    />
  {/if}

  {#if features.length > 0}
    <div class="space-y-2">
      {#if featuresTitle}
        <h4 class="text-xs font-semibold text-zinc-200">{featuresTitle}</h4>
      {/if}
      <ul class="space-y-1.5 text-xs text-zinc-300">
        {#each features as feat (feat)}
          <li class="flex items-center gap-2">
            <span class="flex size-4 shrink-0 items-center justify-center rounded-full bg-blue-500/20 text-blue-400 text-[10px]">
              ✓
            </span>
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

  {#if colors.length > 0}
    <div class="space-y-2">
      <span class="block text-xs font-semibold text-zinc-300">{colorsLabel}</span>
      <div class="flex flex-wrap items-center gap-2.5">
        {#each colors as color (color.value)}
          {@const active = activeColor === color.value}
          <button
            type="button"
            title={color.label}
            onclick={() => (selectedColor = color.value)}
            style:background-color={color.hex}
            class={`flex size-7 items-center justify-center rounded-md border text-[10px] font-bold text-white transition ${
              active
                ? "border-blue-400 ring-2 ring-blue-500/40 scale-105"
                : "border-blender-border opacity-85 hover:opacity-100"
            }`}
          >
            {#if active}✓{/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  {#if sizes.length > 0}
    <div class="space-y-2">
      <span class="block text-xs font-semibold text-zinc-300">{sizesLabel}</span>
      <div class="flex flex-wrap items-center gap-2">
        {#each sizes as size (size.value)}
          {@const active = activeSize === size.value}
          <button
            type="button"
            onclick={() => (selectedSize = size.value)}
            class={`rounded-lg border px-3 py-1.5 text-xs font-medium transition ${
              active
                ? "border-blue-500 bg-blue-600 text-white shadow-xs"
                : "border-blender-border bg-blender-surface text-zinc-300 hover:border-blue-500/50 hover:text-zinc-100"
            }`}
          >
            {size.priceLabel ? `${size.label} = ${size.priceLabel}` : size.label}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  {#if purchaseOptions.length > 0}
    <div class="space-y-2 rounded-xl border border-blender-border bg-blender-surface/50 p-3">
      {#each purchaseOptions as opt (opt.id)}
        {@const active = activePurchase === opt.id}
        <button
          type="button"
          onclick={() => (selectedPurchase = opt.id)}
          class={`flex w-full items-center justify-between rounded-lg px-3 py-2 text-xs transition ${
            active
              ? "bg-blue-500/15 text-zinc-100 font-semibold"
              : "text-zinc-400 hover:bg-blender-hover"
          }`}
        >
          <span class="flex items-center gap-2">
            <span
              class={`size-3.5 rounded-full border flex items-center justify-center ${
                active ? "border-blue-400" : "border-zinc-600"
              }`}
            >
              {#if active}
                <span class="size-2 rounded-full bg-blue-400"></span>
              {/if}
            </span>
            <span>{opt.label}</span>
          </span>
          <span class="font-bold text-zinc-100">{opt.price}</span>
        </button>
      {/each}
    </div>
  {/if}

  {#if showQuantity || stockCountText || subtotalText || deliveryText}
    <div class="space-y-2.5">
      {#if showQuantity}
        <div class="flex flex-wrap items-center gap-4">
          <div class="inline-flex items-center rounded-lg border border-blender-border bg-blender-surface">
            <button
              type="button"
              onclick={() => (quantity = Math.max(1, quantity - 1))}
              class="px-3 py-1.5 text-xs font-bold text-zinc-300 hover:text-white"
            >
              −
            </button>
            <span class="min-w-8 text-center text-xs font-semibold tabular-nums text-zinc-100">
              {quantity}
            </span>
            <button
              type="button"
              onclick={() => (quantity += 1)}
              class="px-3 py-1.5 text-xs font-bold text-zinc-300 hover:text-white"
            >
              +
            </button>
          </div>
          {#if stockCountText}
            <span class="text-xs text-zinc-400">{stockCountText}</span>
          {/if}
        </div>
      {/if}

      {#if subtotalText}
        <p class="text-xs text-zinc-300">
          Subtotal: <span class="font-bold text-zinc-100">{subtotalText}</span>
        </p>
      {/if}

      {#if deliveryText || urgencyText}
        <div class="flex flex-wrap items-center gap-3 text-xs">
          {#if deliveryText}
            <span class="text-zinc-300">{deliveryText}</span>
          {/if}
          {#if urgencyText}
            <span class="font-semibold text-rose-400">{urgencyText}</span>
          {/if}
        </div>
      {/if}
    </div>
  {/if}

  <div class="flex flex-wrap items-center gap-2.5 pt-1">
    {#if buttonProps}
      <CardButton
        {...buttonProps}
        className={buttonProps.className ?? "flex-1 min-w-[160px] justify-center rounded-lg bg-blue-600 px-5 py-2.5 text-xs font-semibold text-white hover:bg-blue-500"}
      />
    {/if}
    {#if secondaryButtonProps}
      <CardButton
        variant="outline"
        {...secondaryButtonProps}
        className={secondaryButtonProps.className ?? "flex-1 min-w-[140px] justify-center rounded-lg border border-blender-border bg-blender-surface px-4 py-2.5 text-xs font-semibold text-zinc-200 hover:bg-blender-hover"}
      />
    {/if}
    {#if cartButtonProps}
      <CardCartButton {...cartButtonProps} />
    {/if}
    {#if favoriteButtonProps}
      <CardFavoriteButton {...favoriteButtonProps} />
    {/if}
  </div>

  {#if wishlistText || compareText}
    <div class="flex flex-wrap items-center gap-5 border-b border-blender-border/60 pb-3 text-xs text-zinc-400">
      {#if wishlistText}
        <button type="button" class="inline-flex items-center gap-1.5 hover:text-blue-400">
          <Icon name="Favorite" className="size-3.5" />
          <span>{wishlistText}</span>
        </button>
      {/if}
      {#if compareText}
        <button type="button" class="inline-flex items-center gap-1.5 hover:text-blue-400">
          <Icon name="Refresh" className="size-3.5" />
          <span>{compareText}</span>
        </button>
      {/if}
    </div>
  {/if}

  {#if trustBadges.length > 0}
    <div class="grid gap-2 pt-1 sm:grid-cols-3">
      {#each trustBadges as badgeItem (badgeItem.text)}
        <div class="flex items-center gap-2 rounded-lg border border-blender-border/70 bg-blender-surface/50 px-3 py-2 text-[11px] text-zinc-300">
          {#if badgeItem.icon}
            <Icon name={badgeItem.icon} className="size-4 shrink-0 text-blue-400" />
          {/if}
          <span>{badgeItem.text}</span>
        </div>
      {/each}
    </div>
  {/if}

  {#if tagsProps}
    <CardTags {...tagsProps} />
  {/if}

  {#if metaItems.length > 0}
    <div class="space-y-1.5 border-t border-blender-border/60 pt-3 text-xs">
      {#each metaItems as meta (meta.label)}
        <p class="text-zinc-400">
          <span class="font-semibold text-zinc-200">{meta.label}:</span>
          <span class="ml-1.5">{meta.value}</span>
        </p>
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
          <span class="rounded-full border border-blender-border bg-blender-surface px-2.5 py-1 text-[11px] text-zinc-300">
            ↓ {link}
          </span>
        {/each}
      </div>
    </div>
  {/if}

  {#if inlineTabsProps}
    <div class="border-t border-blender-border pt-4">
      <ProductTabs {...inlineTabsProps} />
    </div>
  {/if}
</div>
