<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import type { RadioProps } from "../data/types";
  import { StyleComponent } from "../../../../data/style.component";

  type Props = RadioProps & Omit<HTMLInputAttributes, keyof RadioProps | "type" | "value" | "checked" | "class"> & { class?: string };
  let {
    id, name, options, value = $bindable(""), disabled, onValueChange,
    onchange: onNativeChange, class: inputClass, ...rest
  }: Props = $props();

  function handleChange(optionValue: string, event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
    value = optionValue;
    onNativeChange?.(event);
    onValueChange?.(value);
  }
</script>

<fieldset class={StyleComponent.radio.group} {disabled}>
  {#each options as option, index (option.value)}
    {@const inputId = id ? `${id}-${index}` : undefined}
    <label for={inputId} class={`${StyleComponent.radio.item} ${option.disabled || disabled ? StyleComponent.radio.disabled : ""}`}>
      <input
        {...rest}
        type="radio"
        id={inputId}
        {name}
        value={option.value}
        checked={value === option.value}
        disabled={option.disabled || disabled}
        onchange={(event) => handleChange(option.value, event)}
        class={`${StyleComponent.radio.input} ${inputClass ?? ""}`}
      />
      <span class={StyleComponent.radio.dot}></span>
      <span class={StyleComponent.radio.label}>{option.label}</span>
    </label>
  {/each}
</fieldset>
