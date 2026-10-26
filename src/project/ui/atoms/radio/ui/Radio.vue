<script setup lang="ts">
import { computed, ref, useAttrs } from "vue";
import type { InputHTMLAttributes } from "vue";
import type { RadioProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

type Props = RadioProps & Omit<InputHTMLAttributes, keyof RadioProps | "type" | "value" | "checked">;
defineOptions({ inheritAttrs: false });
const props = defineProps<Props>();
const attrs = useAttrs();
const nativeAttrs = computed(() => Object.fromEntries([...Object.entries(props), ...Object.entries(attrs)].filter(([key]) => !["id", "name", "options", "value", "disabled", "onValueChange", "onChange", "class"].includes(key))));
const selected = ref(props.value ?? "");

function handleChange(optionValue: string, event: Event) {
    selected.value = optionValue;
    props.onChange?.(event);
    props.onValueChange?.(optionValue);
}
</script>

<template>
    <fieldset :class="StyleComponent.radio.group" :disabled="props.disabled">
        <label
            v-for="(option, index) in props.options"
            :key="option.value"
            :for="props.id ? `${props.id}-${index}` : undefined"
            :class="[StyleComponent.radio.item, (option.disabled || props.disabled) ? StyleComponent.radio.disabled : '']"
        >
            <input
                v-bind="nativeAttrs"
                type="radio"
                :id="props.id ? `${props.id}-${index}` : undefined"
                :name="props.name"
                :value="option.value"
                :checked="selected === option.value"
                :disabled="option.disabled || props.disabled"
                @change="handleChange(option.value, $event)"
                :class="[StyleComponent.radio.input, attrs.class]"
            />
            <span :class="StyleComponent.radio.dot" />
            <span :class="StyleComponent.radio.label">{{ option.label }}</span>
        </label>
    </fieldset>
</template>