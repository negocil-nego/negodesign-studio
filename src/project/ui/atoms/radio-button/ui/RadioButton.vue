<script setup lang="ts">
import { ref, watch } from "vue";
import { StyleComponent } from "../../../../data/style.component";
import type { RadioButtonProps } from "../data/types";

const props = withDefaults(defineProps<RadioButtonProps>(), {
  name: "btnradio",
  value: "",
  multiple: false,
  values: () => [],
  disabled: false,
});

const selected = ref(props.value);
const selectedList = ref<string[]>([...props.values]);

watch(() => props.value, (next) => {
  selected.value = next;
});

function isSelected(optionValue: string): boolean {
  return props.multiple
    ? selectedList.value.includes(optionValue)
    : selected.value === optionValue;
}

function handleSelect(optionValue: string) {
  if (props.disabled) return;
  if (props.multiple) {
    selectedList.value = selectedList.value.includes(optionValue)
      ? selectedList.value.filter((item) => item !== optionValue)
      : [...selectedList.value, optionValue];
    props.onValuesChange?.(selectedList.value);
  } else {
    selected.value = optionValue;
    props.onValueChange?.(optionValue);
  }
}
</script>

<template>
  <div :class="StyleComponent.radioButton.group" role="group" :aria-label="props.name">
    <label
      v-for="(option, idx) in props.options"
      :key="option.value"
      :for="props.id ? `${props.id}-${idx}` : undefined"
      :class="[
        StyleComponent.radioButton.item,
        isSelected(option.value)
          ? StyleComponent.radioButton.active
          : StyleComponent.radioButton.inactive,
        props.disabled || option.disabled ? StyleComponent.radioButton.disabled : '',
      ]"
    >
      <input
        :id="props.id ? `${props.id}-${idx}` : undefined"
        :type="props.multiple ? 'checkbox' : 'radio'"
        :name="props.name"
        :value="option.value"
        :checked="isSelected(option.value)"
        :disabled="props.disabled || option.disabled"
        :class="StyleComponent.radioButton.input"
        @change="handleSelect(option.value)"
      />
      <span>{{ option.label }}</span>
    </label>
  </div>
</template>
