<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { InputOtpProps } from "../data/types";
import { StyleComponent } from "../../../../../../data/style.component";

const props = withDefaults(defineProps<InputOtpProps>(), {
  value: "",
  length: 6,
  separatorEvery: 3,
  disabled: false,
  required: false,
});
const code = ref(props.value);
const refs = ref<Array<HTMLInputElement | null>>([]);
const count = computed(() => Math.max(1, Math.min(12, props.length)));
const digits = computed(() => Array.from({ length: count.value }, (_, index) => code.value[index] ?? ""));
watch(() => props.value, (value) => (code.value = value ?? ""));

function update(index: number, digit: string) {
  const next = [...digits.value];
  next[index] = digit;
  code.value = next.join("");
  props.onValueChange?.(code.value);
}

function paste(index: number, event: ClipboardEvent) {
  const pasted = event.clipboardData?.getData("text").replace(/\D/g, "");
  if (!pasted) return;
  event.preventDefault();
  const next = [...digits.value];
  pasted.slice(0, count.value - index).split("").forEach((digit, offset) => (next[index + offset] = digit));
  code.value = next.join("");
  props.onValueChange?.(code.value);
  refs.value[Math.min(index + pasted.length, count.value - 1)]?.focus();
}
</script>

<template>
  <div :class="StyleComponent.input.otp.root">
    <label v-if="labelProps" :for="labelProps.for" :class="StyleComponent.input.otp.label" data-name="label">{{ labelProps.label }}</label>
    <div :class="StyleComponent.input.otp.group" role="group" :aria-label="labelProps?.label ?? label ?? 'Código de verificação'">
      <template v-for="(digit, index) in digits" :key="index">
        <span v-if="separatorEvery > 0 && index > 0 && index % separatorEvery === 0" :class="StyleComponent.input.otp.separator" aria-hidden="true">•</span>
        <input
          :ref="(element) => (refs[index] = element as HTMLInputElement)"
          :id="index === 0 ? (labelProps?.for ?? id) : `${id ?? labelProps?.for ?? 'otp'}-${index}`"
          type="text" inputmode="numeric" pattern="[0-9]*" maxlength="1"
          :autocomplete="index === 0 ? 'one-time-code' : 'off'"
          :value="digit" :disabled="disabled" :required="required"
          :aria-label="`Dígito ${index + 1} de ${count}`"
          :class="StyleComponent.input.otp.slot"
          @input="(event) => { const input = event.target as HTMLInputElement; const entered = input.value.replace(/\D/g, '').slice(-1); update(index, entered); if (entered && index < count - 1) refs[index + 1]?.focus(); }"
          @keydown="(event) => { if (event.key === 'ArrowLeft' && index > 0) { event.preventDefault(); refs[index - 1]?.focus(); } else if (event.key === 'ArrowRight' && index < count - 1) { event.preventDefault(); refs[index + 1]?.focus(); } else if (event.key === 'Backspace' && !digit && index > 0) refs[index - 1]?.focus(); }"
          @paste="(event) => paste(index, event)"
        />
      </template>
    </div>
    <input v-if="name" type="hidden" :name="name" :value="code" />
  </div>
</template>
