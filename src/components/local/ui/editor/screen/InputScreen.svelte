<script lang="ts">
  import InputTextTerminal from "../terminal/input/InputTextTerminal.svelte";
  import InputEmailTerminal from "../terminal/input/InputEmailTerminal.svelte";
  import InputPhoneTerminal from "../terminal/input/InputPhoneTerminal.svelte";
</script>

<div class="space-y-5">
  <div>
    <p
      class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400"
    >
      Átomos / Input
    </p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Campos de entrada</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">
      Cada exemplo inclui uma pré-visualização interactiva e o código-fonte do
      componente nas linguagens disponíveis. O código apresentado é carregado
      directamente dos ficheiros do projecto.
    </p>
  </div>

  <InputTextTerminal />
  <InputEmailTerminal />
  <InputPhoneTerminal />
</div>
