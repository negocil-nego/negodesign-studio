<script setup lang="ts">
import { computed, ref, useAttrs } from "vue";
import type { InputHTMLAttributes } from "vue";
import type { CheckboxProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

type Props = CheckboxProps & Omit<InputHTMLAttributes, keyof CheckboxProps | "type">;
defineOptions({ inheritAttrs: false });
const props = defineProps<Props>();
const attrs = useAttrs();
const checked = ref(props.checked ?? false);
const inputAttrs = computed(() => Object.fromEntries(
    [...Object.entries(props), ...Object.entries(attrs)].filter(([key]) => !["id", "name", "value", "checked", "disabled", "required", "label", "onCheckedChange", "onChange", "class"].includes(key)),
));

function handleChange(event: Event) {
    checked.value = (event.target as HTMLInputElement).checked;
    props.onChange?.(event);
    props.onCheckedChange?.(checked.value);
}
</script>

<template>
    <label :class="[StyleComponent.checkbox.root, props.disabled ? StyleComponent.checkbox.disabled : '']" :for="props.id">
        <input
            v-bind="inputAttrs"
            type="checkbox"
            :id="props.id"
            :name="props.name"
            :value="props.value"
            :checked="checked"
            :disabled="props.disabled"
            :required="props.required"
            @change="handleChange"
            :class="[StyleComponent.checkbox.input, attrs.class]"
        />
        <span :class="StyleComponent.checkbox.box">
            <svg v-if="checked" viewBox="0 0 10 8" fill="none" aria-hidden="true" :class="StyleComponent.checkbox.tick">
                <path d="M1 4l3 3 5-6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
        </span>
        <span v-if="props.label" :class="StyleComponent.checkbox.label">{{ props.label }}</span>
    </label>
</template>