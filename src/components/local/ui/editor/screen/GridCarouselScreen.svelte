<script lang="ts">
  import SectionGridCarouselTerminal from "../terminal/grid-carousel/SectionGridCarouselTerminal.svelte";
  import GridCarouselCardMediaTerminal from "../terminal/grid-carousel/GridCarouselCardMediaTerminal.svelte";
  import GridCarouselCardProductTerminal from "../terminal/grid-carousel/GridCarouselCardProductTerminal.svelte";
  import GridCarouselCardProfileTerminal from "../terminal/grid-carousel/GridCarouselCardProfileTerminal.svelte";

  let { showHeader = true }: { showHeader?: boolean } = $props();
</script>

<div class="space-y-5">
  {#if showHeader}
    <header>
      <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">
        Organismos / Grid Carousel
      </p>
      <h2 class="mt-1 text-lg font-semibold text-zinc-100">Grid Carousel</h2>
      <p class="mt-2 text-xs leading-5 text-zinc-400">
        Escolha linhas e colunas para apresentar cartões em páginas deslizáveis.
      </p>
    </header>
  {/if}

  <div class="space-y-5">
    <SectionGridCarouselTerminal />
    <GridCarouselCardMediaTerminal />
    <GridCarouselCardProductTerminal />
    <GridCarouselCardProfileTerminal />
  </div>
</div>
