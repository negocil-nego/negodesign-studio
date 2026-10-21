<script lang="ts">
  import Input from "../../../core/ui/Input.svelte";
  import InputLabel from "../../label/ui/InputLabel.svelte";
  import type { InputAgeProps } from "../data/types";

  let { labelProps, inputProps }: InputAgeProps = $props();
  const componentProps = $derived({ ...inputProps, type: "number" as const });
</script>

{#if labelProps}
  <InputLabel {labelProps} inputProps={componentProps} />
{:else}
  <Input {...componentProps} />
{/if}
