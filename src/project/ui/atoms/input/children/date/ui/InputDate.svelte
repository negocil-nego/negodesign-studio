<script lang="ts">
  import Input from "../../../core/ui/Input.svelte";
  import InputLabel from "../../label/ui/InputLabel.svelte";
  import type { InputDateProps } from "../data/types";

  let { labelProps, inputProps }: InputDateProps = $props();
  const componentProps = $derived({ ...inputProps, type: "date" as const });
</script>

{#if labelProps}
  <InputLabel {labelProps} inputProps={componentProps} />
{:else}
  <Input {...componentProps} />
{/if}
