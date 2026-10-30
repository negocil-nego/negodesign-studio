<script lang="ts">
  import Button from "../../../../../../project/ui/atoms/button/variant/default/ui/Button.svelte";
  import Modal from "../../../../../../project/ui/organisms/modal/core/ui/Modal.svelte";

  let open = $state(false);
</script>

<div class="flex min-h-48 items-center justify-center rounded-lg border border-blender-border bg-blender-base p-6">
  <Button text="Abrir modal" onClick={() => (open = true)} />

  <Modal bind:open title="Detalhes do projecto" onClose={() => (open = false)}>
    <div class="w-full max-w-md space-y-4 p-5">
      <div>
        <p class="text-xs font-semibold uppercase tracking-wider text-blue-400">Negodesign</p>
        <h2 class="mt-1 text-lg font-semibold text-zinc-100">Detalhes do projecto</h2>
        <p class="mt-2 text-sm leading-6 text-zinc-400">
          Use o modal para apresentar conteúdo contextual sem sair da página actual.
        </p>
      </div>
      <div class="flex justify-end">
        <Button text="Fechar" onClick={() => (open = false)} />
      </div>
    </div>
  </Modal>
</div>
