<script lang="ts">
  import Input from "../../../core/ui/Input.svelte";
  import InputLabel from "../../label/ui/InputLabel.svelte";
  import type { InputNumberProps } from "../data/types";

  let { labelProps, inputProps }: InputNumberProps = $props();
  const componentProps = $derived({ ...inputProps, type: "number" as const, icon: inputProps.icon ?? "Number" });
</script>

{#if labelProps}
  <InputLabel {labelProps} inputProps={componentProps} />
{:else}
  <Input {...componentProps} />
{/if}
