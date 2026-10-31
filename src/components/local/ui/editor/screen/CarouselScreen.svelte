<script lang="ts">
  import SectionCarouselButtonsTerminal from "../terminal/carousel/SectionCarouselButtonsTerminal.svelte";
  import SectionCarouselTerminal from "../terminal/carousel/SectionCarouselTerminal.svelte";
</script>

<div class="space-y-5">
  <header>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">Organismos / Carousel</p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Carousels</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">
      Explore o carousel com conteúdo recebido por children e o carousel de imagem com navegação por indicadores.
    </p>
  </header>

  <div class="space-y-5">
    <SectionCarouselButtonsTerminal />
    <SectionCarouselTerminal />
  </div>
</div>
