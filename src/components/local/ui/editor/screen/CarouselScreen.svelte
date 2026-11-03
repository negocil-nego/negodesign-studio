<script lang="ts">
  import SectionCarouselButtonsTerminal from "../terminal/carousel/SectionCarouselButtonsTerminal.svelte";
  import SectionCarouselTerminal from "../terminal/carousel/SectionCarouselTerminal.svelte";
  import CarouselCardMediaTerminal from "../terminal/carousel/CarouselCardMediaTerminal.svelte";
  import CarouselCardProductTerminal from "../terminal/carousel/CarouselCardProductTerminal.svelte";
  import CarouselCardProfileTerminal from "../terminal/carousel/CarouselCardProfileTerminal.svelte";

  let { showHeader = true }: { showHeader?: boolean } = $props();
</script>

<div class="space-y-5">
  {#if showHeader}
    <header>
      <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">Organismos / Carousel</p>
      <h2 class="mt-1 text-lg font-semibold text-zinc-100">Carousels</h2>
      <p class="mt-2 text-xs leading-5 text-zinc-400">
        Explore os carousels de imagem e os carousels de cartões de media, produto e perfil.
      </p>
    </header>
  {/if}

  <div class="space-y-5">
    <SectionCarouselButtonsTerminal />
    <SectionCarouselTerminal />
    <CarouselCardMediaTerminal />
    <CarouselCardProductTerminal />
    <CarouselCardProfileTerminal />
  </div>
</div>
