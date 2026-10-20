<script lang="ts">
  import Input from "../../../core/ui/Input.svelte";
  import InputLabel from "../../label/ui/InputLabel.svelte";
  import type { InputEmailProps } from "../data/types";

  let { labelProps, inputProps }: InputEmailProps = $props();

  const emailProps = $derived({ ...inputProps, type: "email" as const });
</script>

{#if labelProps}
  <InputLabel {labelProps} inputProps={emailProps} />
{:else}
  <Input {...emailProps} />
{/if}
