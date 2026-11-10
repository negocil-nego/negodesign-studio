<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { InputOtpProps } from "../data/types";
import { StyleComponent } from "../../../../../../data/style.component";
import {
  applyOtpPaste,
  clampOtpLength,
  createOtpDigits,
  updateOtpDigit,
} from "../../../../../../hooks/otp-input.hook";

const props = withDefaults(defineProps<InputOtpProps>(), {
  value: "",
  length: 6,
  separatorEvery: 3,
  disabled: false,
  required: false,
  position: "start",
});

const code = ref(props.value);
const refs = ref<Array<HTMLInputElement | null>>([]);
const count = computed(() => clampOtpLength(props.length));
const digits = computed(() => createOtpDigits(code.value, count.value));
const s = StyleComponent.input.otp;

watch(
  () => props.value,
  (value) => (code.value = value ?? "")
);

function handleInput(index: number, event: Event) {
  const input = event.target as HTMLInputElement;
  const entered = input.value.replace(/\D/g, "").slice(-1);
  code.value = updateOtpDigit(digits.value, index, entered);
  props.onValueChange?.(code.value);
  if (entered && index < count.value - 1) refs.value[index + 1]?.focus();
}

function handleKeydown(index: number, digit: string, event: KeyboardEvent) {
  if (event.key === "ArrowLeft" && index > 0) {
    event.preventDefault();
    refs.value[index - 1]?.focus();
  } else if (event.key === "ArrowRight" && index < count.value - 1) {
    event.preventDefault();
    refs.value[index + 1]?.focus();
  } else if (event.key === "Backspace" && !digit && index > 0) {
    refs.value[index - 1]?.focus();
  }
}

function handlePaste(index: number, event: ClipboardEvent) {
  const raw = event.clipboardData?.getData("text") ?? "";
  const result = applyOtpPaste(digits.value, index, raw, count.value);
  if (!result) return;
  event.preventDefault();
  code.value = result.code;
  props.onValueChange?.(code.value);
  refs.value[result.focusIndex]?.focus();
}
</script>

<template>
  <div :class="`${s.root} ${position === 'center' ? 'mx-auto w-fit' : ''}`">
    <label
      v-if="labelProps"
      :for="labelProps.for"
      :class="`${s.label} ${s.labelPosition[position]} block`"
      data-name="label"
    >
      {{ labelProps.label }}
    </label>
    <div :class="s.group" role="group" :aria-label="labelProps?.label ?? label ?? 'Código de verificação'">
      <template v-for="(digit, index) in digits" :key="index">
        <span v-if="separatorEvery > 0 && index > 0 && index % separatorEvery === 0" :class="s.separator" aria-hidden="true">•</span>
        <input
          :ref="(element) => (refs[index] = element as HTMLInputElement)"
          :id="index === 0 ? (labelProps?.for ?? id) : `${id ?? labelProps?.for ?? 'otp'}-${index}`"
          type="text"
          inputmode="numeric"
          pattern="[0-9]*"
          maxlength="1"
          :autocomplete="index === 0 ? 'one-time-code' : 'off'"
          :value="digit"
          :disabled="disabled"
          :required="required"
          :aria-label="`Dígito ${index + 1} de ${count}`"
          :class="s.slot"
          @input="handleInput(index, $event)"
          @keydown="handleKeydown(index, digit, $event)"
          @paste="handlePaste(index, $event)"
        />
      </template>
    </div>
    <input v-if="name" type="hidden" :name="name" :value="code" />
  </div>
</template>
