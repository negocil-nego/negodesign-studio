<script lang="ts">
  import type { ProductInfoOptionsProps } from "../data/types";

  let {
    colorsLabel = "Colors", colors = [], sizesLabel = "Size", sizes = [], purchaseOptions = [],
    showQuantity = false, quantity = $bindable(1), selectedColor = $bindable(""),
    selectedSize = $bindable(""), selectedPurchase = $bindable(""), stockCountText,
    subtotalText, deliveryText, urgencyText,
  }: ProductInfoOptionsProps = $props();

  const activeColor = $derived(selectedColor || colors[0]?.value || "");
  const activeSize = $derived(selectedSize || sizes[0]?.value || "");
  const activePurchase = $derived(selectedPurchase || purchaseOptions[0]?.id || "");
</script>

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
            active ? "border-blue-400 ring-2 ring-blue-500/40 scale-105" : "border-blender-border opacity-85 hover:opacity-100"
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
            active ? "border-blue-500 bg-blue-600 text-white shadow-xs" : "border-blender-border bg-blender-surface text-zinc-300 hover:border-blue-500/50 hover:text-zinc-100"
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
          active ? "bg-blue-500/15 text-zinc-100 font-semibold" : "text-zinc-400 hover:bg-blender-hover"
        }`}
      >
        <span class="flex items-center gap-2">
          <span class={`size-3.5 rounded-full border flex items-center justify-center ${active ? "border-blue-400" : "border-zinc-600"}`}>{#if active}<span class="size-2 rounded-full bg-blue-400"></span>{/if}</span>
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
          <button type="button" onclick={() => (quantity = Math.max(1, quantity - 1))} class="px-3 py-1.5 text-xs font-bold text-zinc-300 hover:text-white">−</button>
          <span class="min-w-8 text-center text-xs font-semibold tabular-nums text-zinc-100">{quantity}</span>
          <button type="button" onclick={() => (quantity += 1)} class="px-3 py-1.5 text-xs font-bold text-zinc-300 hover:text-white">+</button>
        </div>
        {#if stockCountText}<span class="text-xs text-zinc-400">{stockCountText}</span>{/if}
      </div>
    {/if}
    {#if subtotalText}<p class="text-xs text-zinc-300">Subtotal: <span class="font-bold text-zinc-100">{subtotalText}</span></p>{/if}
    {#if deliveryText || urgencyText}
      <div class="flex flex-wrap items-center gap-3 text-xs">
        {#if deliveryText}<span class="text-zinc-300">{deliveryText}</span>{/if}
        {#if urgencyText}<span class="font-semibold text-rose-400">{urgencyText}</span>{/if}
      </div>
    {/if}
  </div>
{/if}
