<script lang="ts">
  import Input from "../../../core/ui/Input.svelte";
  import InputLabel from "../../label/ui/InputLabel.svelte";
  import type { InputTextProps } from "../data/types";

  let { labelProps, inputProps }: InputTextProps = $props();

  const textProps = $derived({ ...inputProps, type: "text" as const, icon: inputProps.icon ?? "InputCursorText" });
</script>

{#if labelProps}
  <InputLabel {labelProps} inputProps={textProps} />
{:else}
  <Input {...textProps} />
{/if}
