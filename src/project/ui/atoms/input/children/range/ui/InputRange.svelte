<script lang="ts">
  import { StyleComponent } from "../../../../../../data/style.component";
  import type { InputRangeProps } from "../data/types";

  let {
    id,
    name,
    label = "Example range",
    value = $bindable(50),
    min = 0,
    max = 100,
    step = 1,
    disabled = false,
    showValue = true,
    hint,
    onValueChange,
  }: InputRangeProps = $props();

  function handleInput(event: Event & { currentTarget: HTMLInputElement }) {
    value = Number(event.currentTarget.value);
    onValueChange?.(value);
  }
</script>

<div class={StyleComponent.input.range.field}>
  {#if label || showValue}
    <div class={StyleComponent.input.range.header}>
      {#if label}
        <label for={id} class={StyleComponent.input.range.label}>{label}</label>
      {/if}
      {#if showValue}
        <span class={StyleComponent.input.range.valueBadge}>{value}</span>
      {/if}
    </div>
  {/if}
  <input
    type="range"
    {id}
    {name}
    {min}
    {max}
    {step}
    {disabled}
    value={value}
    oninput={handleInput}
    class={StyleComponent.input.range.input}
  />
  {#if hint}
    <span class={StyleComponent.input.range.hint}>{hint}</span>
  {/if}
</div>
