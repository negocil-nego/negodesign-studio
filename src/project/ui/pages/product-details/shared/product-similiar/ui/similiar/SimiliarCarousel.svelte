<script lang="ts">
  import Icon from "../../../../../../atoms/icon/ui/Icon.svelte";
  import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct.svelte";
  import CardButton from "../../../../../../molecules/card/shared/card-button/ui/CardButton.svelte";
  import CardDescription from "../../../../../../molecules/card/shared/card-description/ui/CardDescription.svelte";
  import CardTitle from "../../../../../../molecules/card/shared/card-title/ui/CardTitle.svelte";
  import type { SimiliarCarouselProps } from "../../data/types";

  let { titleProps, descriptionProps, items = [], moreButtonProps, className = "" }: SimiliarCarouselProps = $props();

  let trackEl = $state<HTMLDivElement | null>(null);

  function scrollCarousel(dir: -1 | 1) {
    if (!trackEl) return;
    trackEl.scrollBy({ left: dir * 320, behavior: "smooth" });
  }
</script>

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
