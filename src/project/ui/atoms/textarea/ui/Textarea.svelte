<script lang="ts">
  import type { TextareaProps } from "../data/types";
  import { StyleComponent } from "../../../../data/style.component";
  import Label from "../../label/ui/label.svelte";

  let { id, name, value = $bindable(""), placeholder, rows = 4, cols, disabled, required, readonly, maxlength, labelProps }: TextareaProps = $props();
</script>

<div class={StyleComponent.textarea.form}>
  {#if labelProps}
    <Label {...labelProps} />
  {/if}
  <textarea
    {id}
    {name}
    bind:value
    {placeholder}
    {rows}
    {cols}
    {disabled}
    {required}
    readonly={readonly}
    maxlength={maxlength}
    data-name="textarea"
    class={StyleComponent.textarea.default}
  ></textarea>
</div>
