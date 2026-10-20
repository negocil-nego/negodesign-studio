<script lang="ts">
  import Input from "../../../core/ui/Input.svelte";
  import InputLabel from "../../label/ui/InputLabel.svelte";
  import type { InputPhoneProps } from "../data/types";

  let { labelProps, inputProps }: InputPhoneProps = $props();

  const phoneProps = $derived({ ...inputProps, type: "tel" as const });
</script>

{#if labelProps}
  <InputLabel {labelProps} inputProps={phoneProps} />
{:else}
  <Input {...phoneProps} />
{/if}
