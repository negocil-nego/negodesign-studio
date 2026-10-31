<script setup lang="ts">
import { computed, ref } from "vue";
import Input from "../../../core/ui/Input.vue";
import type { InputUsernameProps } from "../data/types";
import { getUsernameValidationMessage, isValidUsername } from "../data/validation";
import { StyleComponent } from "../../../../../../data/style.component";

const props = defineProps<InputUsernameProps>();
const value = ref(String(props.inputProps.value ?? ""));
const touched = ref(false);
const valid = computed(() => value.value.trim() ? isValidUsername(value.value, props.validationMode ?? "any") : !props.inputProps.required);
const invalid = computed(() => touched.value && !valid.value);
const resolvedErrorMessage = computed(() => props.errorMessage ?? getUsernameValidationMessage(props.validationMode ?? "any"));
const errorId = `${props.inputProps.id ?? props.labelProps?.for ?? "input-username"}-error`;

function update(event: Event) {
  value.value = (event.currentTarget as HTMLInputElement).value;
  touched.value = true;
  props.onValueChange?.(value.value);
  props.onValidityChange?.(value.value.trim() ? isValidUsername(value.value, props.validationMode ?? "any") : !props.inputProps.required);
}

function blur() {
  touched.value = true;
  props.onValidityChange?.(valid.value);
}
</script>

<template>
  <div class="space-y-1.5">
    <label v-if="labelProps" :for="labelProps.for" :class="StyleComponent.input.text.label">{{ labelProps.label }}</label>
    <Input
      v-bind="inputProps"
      :icon="inputProps.icon ?? 'User'"
      type="text"
      :value="value"
      :isInvalid="invalid"
      autoComplete="username"
      :aria-describedby="invalid ? errorId : undefined"
      @input="update"
      @blur="blur"
    />
    <p v-if="invalid" :id="errorId" class="text-xs text-destructive" role="alert">{{ resolvedErrorMessage }}</p>
  </div>
</template>
