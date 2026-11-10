<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { RadioButtonProps } from "../data/types";

  let {
    id,
    name = "btnradio",
    options = [],
    value = $bindable(""),
    multiple = false,
    values = $bindable<string[]>([]),
    disabled = false,
    onValueChange,
    onValuesChange,
  }: RadioButtonProps = $props();

  function isSelected(optionValue: string): boolean {
    return multiple ? values.includes(optionValue) : value === optionValue;
  }

  function handleSelect(optionValue: string) {
    if (disabled) return;
    if (multiple) {
      values = values.includes(optionValue)
        ? values.filter((item) => item !== optionValue)
        : [...values, optionValue];
      onValuesChange?.(values);
    } else {
      value = optionValue;
      onValueChange?.(value);
    }
  }
</script>

<div class={StyleComponent.radioButton.group} role="group" aria-label={name}>
  {#each options as option, idx (option.value)}
    {@const active = isSelected(option.value)}
    {@const isDisabled = disabled || Boolean(option.disabled)}
    <label
      for={id ? `${id}-${idx}` : undefined}
      class={`
        ${StyleComponent.radioButton.item}
        ${active ? StyleComponent.radioButton.active : StyleComponent.radioButton.inactive}
        ${isDisabled ? StyleComponent.radioButton.disabled : ""}
      `}
    >
      <input
        id={id ? `${id}-${idx}` : undefined}
        type={multiple ? "checkbox" : "radio"}
        {name}
        value={option.value}
        checked={active}
        disabled={isDisabled}
        onchange={() => handleSelect(option.value)}
        class={StyleComponent.radioButton.input}
      />
      <span>{option.label}</span>
    </label>
  {/each}
</div>
