<script setup lang="ts">
import { ref, watch } from "vue";
import { StyleComponent } from "../../../../../../data/style.component";
import type { InputRangeProps } from "../data/types";

const props = withDefaults(defineProps<InputRangeProps>(), {
  label: "Example range",
  value: 50,
  min: 0,
  max: 100,
  step: 1,
  disabled: false,
  showValue: true,
});

const current = ref(props.value);
watch(() => props.value, (next) => {
  current.value = next;
});

function onInput(event: Event) {
  const next = Number((event.target as HTMLInputElement).value);
  current.value = next;
  props.onValueChange?.(next);
}
</script>

<template>
  <div :class="StyleComponent.input.range.field">
    <div v-if="props.label || props.showValue" :class="StyleComponent.input.range.header">
      <label v-if="props.label" :for="props.id" :class="StyleComponent.input.range.label">
        {{ props.label }}
      </label>
      <span v-if="props.showValue" :class="StyleComponent.input.range.valueBadge">
        {{ current }}
      </span>
    </div>
    <input
      type="range"
      :id="props.id"
      :name="props.name"
      :min="props.min"
      :max="props.max"
      :step="props.step"
      :disabled="props.disabled"
      :value="current"
      :class="StyleComponent.input.range.input"
      @input="onInput"
    />
    <span v-if="props.hint" :class="StyleComponent.input.range.hint">{{ props.hint }}</span>
  </div>
</template>
