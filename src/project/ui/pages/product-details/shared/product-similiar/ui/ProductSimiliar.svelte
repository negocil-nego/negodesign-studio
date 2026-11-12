<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import CardProduct from "../../../../../molecules/card/product/ui/CardProduct.svelte";
  import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton.svelte";
  import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.svelte";
  import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.svelte";
  import type { ProductSimiliarProps } from "../data/types";

  let {
    variant = "grid",
    headerAlign = "left",
    titleProps = { title: "Similar Products" },
    descriptionProps,
    columns = 4,
    items = [],
    moreButtonProps,
    className = "",
  }: ProductSimiliarProps = $props();

  let trackEl = $state<HTMLDivElement | null>(null);

  function scrollCarousel(dir: -1 | 1) {
    if (!trackEl) return;
    trackEl.scrollBy({ left: dir * 320, behavior: "smooth" });
  }

  const gridColsClass = $derived(
    columns === 5
      ? "grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5"
      : columns === 3
        ? "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3"
        : "grid-cols-1 sm:grid-cols-2 lg:grid-cols-4",
  );
</script>

{#if variant === "sidebar"}
  <aside class={`space-y-4 rounded-2xl border border-blender-border bg-blender-card p-5 shadow-sm ${className}`}>
    {#if titleProps}
      <div class="border-b border-blender-border pb-3 text-center">
        <CardTitle
          {...titleProps}
          className={titleProps.className ?? "text-sm font-semibold text-zinc-100"}
        />
      </div>
    {/if}

    <div class="space-y-3">
      {#each items as item, idx (idx)}
        <CardProduct {...item} variant={item.variant ?? "02"} />
      {/each}
    </div>

    {#if moreButtonProps}
      <div class="pt-2 text-center">
        <CardButton variant="outline" {...moreButtonProps} />
      </div>
    {/if}
  </aside>
{:else if variant === "carousel"}
  <section class={`space-y-5 ${className}`}>
    <div class="flex flex-wrap items-end justify-between gap-4">
      <div>
        {#if titleProps}
          <CardTitle
            {...titleProps}
            className={titleProps.className ?? "text-xl font-semibold tracking-tight text-zinc-100 sm:text-2xl"}
          />
        {/if}
        {#if descriptionProps}
          <CardDescription {...descriptionProps} />
        {/if}
      </div>

      <div class="flex items-center gap-2">
        <button
          type="button"
          onclick={() => scrollCarousel(-1)}
          aria-label="Anterior"
          class="flex size-9 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300 transition hover:border-blue-500 hover:text-white"
        >
          <Icon name="ChevronLeft" className="size-4" />
        </button>
        <button
          type="button"
          onclick={() => scrollCarousel(1)}
          aria-label="Seguinte"
          class="flex size-9 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300 transition hover:border-blue-500 hover:text-white"
        >
          <Icon name="ChevronRight" className="size-4" />
        </button>
      </div>
    </div>

    <div
      bind:this={trackEl}
      class="grid grid-flow-col auto-cols-[minmax(240px,1fr)] gap-4 overflow-x-auto pb-2 scroll-smooth"
    >
      {#each items as item, idx (idx)}
        <CardProduct {...item} variant={item.variant ?? "01"} />
      {/each}
    </div>

    {#if moreButtonProps}
      <div class="flex justify-center pt-2">
        <CardButton variant="outline" {...moreButtonProps} />
      </div>
    {/if}
  </section>
{:else}
  <section class={`space-y-6 ${className}`}>
    <div
      class={headerAlign === "center"
        ? "flex flex-col items-center text-center space-y-1.5"
        : "flex flex-wrap items-end justify-between gap-4"}
    >
      <div>
        {#if titleProps}
          <CardTitle
            {...titleProps}
            className={titleProps.className ?? "text-xl font-semibold tracking-tight text-zinc-100 sm:text-2xl"}
          />
        {/if}
        {#if descriptionProps}
          <CardDescription {...descriptionProps} />
        {/if}
      </div>

      {#if headerAlign === "center"}
        <span class="h-1 w-12 rounded-full bg-blue-500"></span>
      {:else if moreButtonProps}
        <CardButton variant="outline" {...moreButtonProps} />
      {/if}
    </div>

    <div class={`grid gap-4 ${gridColsClass}`}>
      {#each items as item, idx (idx)}
        <CardProduct {...item} variant={item.variant ?? "01"} />
      {/each}
    </div>

    {#if headerAlign === "center" && moreButtonProps}
      <div class="flex justify-center pt-2">
        <CardButton variant="outline" {...moreButtonProps} />
      </div>
    {/if}
  </section>
{/if}
