<script lang="ts">
  import type { TextareaProps } from "../data/types";

  let { id, name, value = $bindable(""), placeholder, rows = 4, cols, disabled, required, readonly, maxlength }: TextareaProps = $props();
</script>

<div class="input-form">
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
  ></textarea>
</div>
