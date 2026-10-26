<script setup lang="ts">
import { computed, ref, useAttrs } from "vue";
import type { ButtonHTMLAttributes } from "vue";
import type { ToggleProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

type Props = ToggleProps & Omit<ButtonHTMLAttributes, keyof ToggleProps | "type">;
defineOptions({ inheritAttrs: false });
const props = defineProps<Props>();
const attrs = useAttrs();
const nativeAttrs = computed(() => Object.fromEntries([...Object.entries(props), ...Object.entries(attrs)].filter(([key]) => !["id", "name", "value", "checked", "disabled", "label", "labelFirst", "onCheckedChange", "onClick", "class"].includes(key))));
const checked = ref(props.checked ?? false);

function handleClick(event: MouseEvent) {
    if (props.disabled) return;
    checked.value = !checked.value;
    props.onClick?.(event);
    props.onCheckedChange?.(checked.value);
}
</script>

<template>
    <label :class="[StyleComponent.toggle.root, props.disabled ? 'opacity-50 cursor-not-allowed' : '']" :for="props.id">
        <button
            v-bind="nativeAttrs"
            type="button"
            :id="props.id"
            :name="props.name"
            :value="props.value"
            role="switch"
            :aria-checked="checked"
            :disabled="props.disabled"
            @click="handleClick"
            :class="[StyleComponent.toggle.track, checked ? StyleComponent.toggle.on : '', attrs.class]"
        >
            <span :class="[StyleComponent.toggle.thumb, checked ? StyleComponent.toggle.thumbOn : '']" />
        </button>
        <span v-if="props.label" :class="StyleComponent.toggle.label">{{ typeof props.label === 'string' ? props.label : props.label.label }}</span>
    </label>
</template>