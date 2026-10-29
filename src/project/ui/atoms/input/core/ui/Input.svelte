<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import type { InputProps } from "../data/types";
  import { getIconClass } from "../../../../../../hooks/icon.svelte";
  import { StyleComponent } from "../../../../../data/style.component";

  type Props = InputProps & Omit<HTMLInputAttributes, keyof InputProps | "class"> & { class?: string };
  let { id, icon, type, required, disabled, placeholder, value, isInvalid = false, class: inputClass, ...rest }: Props = $props();

  const fieldClass = $derived(
    (StyleComponent.input as unknown as Record<string, { root?: string }>)[type ?? ""]
      ?.root ?? StyleComponent.input.form,
  );
</script>

<div class="{fieldClass} {isInvalid ? StyleComponent.input.error : ''}">
  {#if icon}<i class={getIconClass(icon)} aria-hidden="true"></i>{/if}
  <input
    {...rest}
    {id}
    {type}
    {required}
    {disabled}
    {placeholder}
    {value}
    aria-invalid={isInvalid ? "true" : undefined}
    data-name="label"
    class={inputClass}
  />
</div>
