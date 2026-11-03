<script lang="ts">
  import Icon from "../../../../atoms/icon/ui/Icon.svelte";
  import type { SectionGridCarouselProps } from "../data/types";

  let {
    titleProps,
    descriptionProps,
    className = "",
    rows = 2,
    columns = 4,
    children,
  }: SectionGridCarouselProps = $props();
  let viewport = $state<HTMLDivElement>();

  function scroll(direction: -1 | 1) {
    viewport?.scrollBy({
      left: direction * viewport.clientWidth,
      behavior: "smooth",
    });
  }
</script>

<section class={`w-full min-w-0 py-6 ${className}`.trim()}>
  <header class="mb-6 flex items-end justify-between gap-5">
    <div class="min-w-0">
      {#if titleProps}
        <h2
          class="text-2xl font-semibold leading-tight text-zinc-100 sm:text-3xl"
        >
          {titleProps.text}
        </h2>
      {/if}
      {#if descriptionProps}
        <p
          class="mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base"
        >
          {descriptionProps.text}
        </p>
      {/if}
    </div>
    <div class="flex shrink-0 gap-2">
      <button
        type="button"
        aria-label="Itens anteriores"
        onclick={() => scroll(-1)}
        class="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300"
      >
        <Icon name="ArrowLeft" className="size-4" />
      </button>
      <button
        type="button"
        aria-label="Itens seguintes"
        onclick={() => scroll(1)}
        class="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300"
      >
        <Icon name="ArrowRight" className="size-4" />
      </button>
    </div>
  </header>
  <div
    bind:this={viewport}
    class="grid grid-flow-col gap-5 overflow-x-auto snap-x snap-mandatory scroll-smooth [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
    style={`grid-template-rows: repeat(${Math.max(1, Math.floor(rows))}, minmax(0, 1fr)); grid-auto-columns: calc((100% - ${(Math.max(1, Math.floor(columns)) - 1) * 1.25}rem) / ${Math.max(1, Math.floor(columns))})`}
  >
    {@render children()}
  </div>
</section>
