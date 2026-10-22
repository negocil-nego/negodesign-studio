<script lang="ts">
  import type { RadioProps } from "../data/types";

  let { name, options, value = $bindable(""), disabled, onchange }: RadioProps = $props();

  function handleChange(optionValue: string) {
    value = optionValue;
    onchange?.(value);
  }
</script>

<fieldset class="radio-group" {disabled}>
  {#each options as option}
    <label class="radio-item" class:radio-disabled={option.disabled || disabled}>
      <input
        type="radio"
        {name}
        value={option.value}
        checked={value === option.value}
        disabled={option.disabled || disabled}
        onchange={() => handleChange(option.value)}
        class="radio-input"
      />
      <span class="radio-dot"></span>
      <span class="radio-label">{option.label}</span>
    </label>
  {/each}
</fieldset>
