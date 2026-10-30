<script lang="ts">
  import SubmenuGridTitleTerminal from "../terminal/menu/SubmenuGridTitleTerminal.svelte";
  import SubmenuSidebarGridTerminal from "../terminal/menu/SubmenuSidebarGridTerminal.svelte";
</script>

<div class="space-y-5">
  <div>
    <p
      class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400"
    >
      Organismos / Menu
    </p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Menus</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">
      Pré-visualização interactiva e código-fonte dos componentes de menu.
    </p>
  </div>

  <div class="grid w-full grid-cols-1 items-stretch gap-5">
    <div class="min-w-0"><SubmenuGridTitleTerminal /></div>
    <div class="min-w-0"><SubmenuSidebarGridTerminal /></div>
  </div>
</div>
