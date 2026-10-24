<script lang="ts">
  import type { RadioProps } from "../data/types";
  import { StyleComponent } from "../../../../data/style.component";

  let { name, options, value = $bindable(""), disabled, onchange }: RadioProps = $props();

  function handleChange(optionValue: string) {
    value = optionValue;
    onchange?.(value);
  }
</script>

<fieldset class={StyleComponent.radio.group} {disabled}>
  {#each options as option}
    <label class={`${StyleComponent.radio.item} ${option.disabled || disabled ? StyleComponent.radio.disabled : ""}`}>
      <input
        type="radio"
        {name}
        value={option.value}
        checked={value === option.value}
        disabled={option.disabled || disabled}
        onchange={() => handleChange(option.value)}
        class={StyleComponent.radio.input}
      />
      <span class={StyleComponent.radio.dot}></span>
      <span class={StyleComponent.radio.label}>{option.label}</span>
    </label>
  {/each}
</fieldset>
