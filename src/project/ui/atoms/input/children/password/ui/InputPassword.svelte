<script lang="ts">
  import Input from "../../../core/ui/Input.svelte";
  import InputLabel from "../../label/ui/InputLabel.svelte";
  import type { InputPasswordProps } from "../data/types";

  let { labelProps, inputProps }: InputPasswordProps = $props();
  const componentProps = $derived({ ...inputProps, type: "password" as const, icon: inputProps.icon ?? "Password" });
</script>

{#if labelProps}
  <InputLabel {labelProps} inputProps={componentProps} />
{:else}
  <Input {...componentProps} />
{/if}
