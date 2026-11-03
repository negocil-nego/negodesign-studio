<script lang="ts">
  import SectionGridTerminal from "../terminal/grid/SectionGridTerminal.svelte";
  import GridCardMediaTerminal from "../terminal/grid/GridCardMediaTerminal.svelte";
  import GridCardProductTerminal from "../terminal/grid/GridCardProductTerminal.svelte";
  import GridCardProfileTerminal from "../terminal/grid/GridCardProfileTerminal.svelte";

  let { showHeader = true }: { showHeader?: boolean } = $props();
</script>

<div class="space-y-5">
  {#if showHeader}
    <header>
      <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">Organismos / Grid</p>
      <h2 class="mt-1 text-lg font-semibold text-zinc-100">Grid</h2>
      <p class="mt-2 text-xs leading-5 text-zinc-400">
        Pré-visualize cartões de media, produto e perfil numa grelha responsiva.
      </p>
    </header>
  {/if}

  <div class="space-y-5">
    <SectionGridTerminal />
    <GridCardMediaTerminal />
    <GridCardProductTerminal />
    <GridCardProfileTerminal />
  </div>
</div>
