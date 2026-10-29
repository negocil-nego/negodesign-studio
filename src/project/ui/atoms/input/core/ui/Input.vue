<script setup lang="ts">
import { computed, useAttrs } from "vue";
import type { InputHTMLAttributes } from "vue";
import type { InputProps } from "../data/types";
import { getIconClass } from "../../../../../../hooks/icon.svelte";
import { StyleComponent } from "../../../../../data/style.component";

type Props = InputProps & Omit<InputHTMLAttributes, keyof InputProps>;
defineOptions({ inheritAttrs: false });
const props = defineProps<Props>();
const attrs = useAttrs();
const inputAttrs = computed(() => Object.fromEntries(
    [...Object.entries(props), ...Object.entries(attrs)].filter(([key]) => !["id", "icon", "type", "required", "disabled", "placeholder", "value", "isInvalid"].includes(key)),
));
const fieldClass = computed(
    () => `${(StyleComponent.input as unknown as Record<string, { root?: string }>)[props.type ?? ""]?.root ?? StyleComponent.input.form} ${props.isInvalid ? StyleComponent.input.error : ""}`,
);
</script>

<template>
    <div :class="fieldClass">
        <i v-if="props.icon" :class="getIconClass(props.icon)" aria-hidden="true" />
        <input
            v-bind="inputAttrs"
            :id="props.id"
            :type="props.type"
            :required="props.required"
            :disabled="props.disabled"
            :placeholder="props.placeholder"
            :value="props.value"
            :aria-invalid="props.isInvalid || undefined"
            data-name="label"
        />
    </div>
</template>
