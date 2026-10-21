<script lang="ts">
  import InputTextTerminal from "../terminal/input/InputTextTerminal.svelte";
  import InputEmailTerminal from "../terminal/input/InputEmailTerminal.svelte";
  import InputPhoneTerminal from "../terminal/input/InputPhoneTerminal.svelte";
  import InputPhoneFlagTerminal from "../terminal/input/InputPhoneFlagTerminal.svelte";
  import InputPasswordTerminal from "../terminal/input/InputPasswordTerminal.svelte";
  import InputDateTerminal from "../terminal/input/InputDateTerminal.svelte";
  import InputTimeTerminal from "../terminal/input/InputTimeTerminal.svelte";
  import InputNumberTerminal from "../terminal/input/InputNumberTerminal.svelte";
  import InputAgeTerminal from "../terminal/input/InputAgeTerminal.svelte";
</script>

<div class="space-y-5">
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">
      Átomos / Input
    </p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Campos de entrada</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">
      Pré-visualização interactiva e código-fonte dos componentes de entrada.
    </p>
  </div>

  <InputTextTerminal />
  <InputEmailTerminal />
  <InputPhoneTerminal />
  <InputPhoneFlagTerminal />
  <InputPasswordTerminal />
  <InputDateTerminal />
  <InputTimeTerminal />
  <InputNumberTerminal />
  <InputAgeTerminal />
</div>
