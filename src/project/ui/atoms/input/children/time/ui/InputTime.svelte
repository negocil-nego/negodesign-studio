<script lang="ts">
  import Input from "../../../core/ui/Input.svelte";
  import InputLabel from "../../label/ui/InputLabel.svelte";
  import type { InputTimeProps } from "../data/types";

  let { labelProps, inputProps }: InputTimeProps = $props();
  const componentProps = $derived({ ...inputProps, type: "time" as const });
</script>

{#if labelProps}
  <InputLabel {labelProps} inputProps={componentProps} />
{:else}
  <Input {...componentProps} />
{/if}
