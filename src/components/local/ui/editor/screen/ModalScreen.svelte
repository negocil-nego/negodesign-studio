<script lang="ts">
  import ModalTerminal from "../terminal/modal/ModalTerminal.svelte";
  import ModalFormTerminal from "../terminal/modal/ModalFormTerminal.svelte";
</script>

<div class="space-y-5">
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">Organismos / Modal</p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Modais</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">
      Exemplos interactivos do modal base e do formulário apresentado num modal.
    </p>
  </div>

  <div class="space-y-5">
    <ModalTerminal />
    <ModalFormTerminal />
  </div>
</div>
