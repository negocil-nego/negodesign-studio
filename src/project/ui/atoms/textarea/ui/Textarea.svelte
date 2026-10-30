<script lang="ts">
  import type { HTMLTextareaAttributes } from "svelte/elements";
  import type { TextareaProps } from "../data/types";
  import { StyleComponent } from "../../../../data/style.component";
  import Label from "../../label/ui/Label.svelte";

  type Props = TextareaProps & Omit<HTMLTextareaAttributes, keyof TextareaProps | "class"> & { class?: string };
  let {
    id, name, value = $bindable(""), placeholder, rows = 4, cols, disabled,
    required, readonly, maxlength, labelProps, class: textareaClass, ...rest
  }: Props = $props();
</script>

<div class={StyleComponent.textarea.form}>
  {#if labelProps}<Label {...labelProps} />{/if}
  <textarea
    {...rest}
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
    class={`${StyleComponent.textarea.default} ${textareaClass ?? ""}`}
  ></textarea>
</div>
