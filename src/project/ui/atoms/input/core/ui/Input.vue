<script setup lang="ts">
import { computed } from "vue";
import type { InputProps } from "../data/types";
import { getIconClass } from "../../../../../../hooks/icon.svelte";
import { StyleComponent } from "../../../../../data/style.component";

const props = defineProps<InputProps>();

const fieldClass = computed(
    () =>
        (StyleComponent.input as unknown as Record<string, { root?: string }>)[props.type ?? ""]?.root ??
        StyleComponent.input.form,
);
</script>

<template>
    <div :class="fieldClass">
        <i v-if="props.icon" :class="getIconClass(props.icon)" aria-hidden="true" />
        <input
            :id="props.id"
            :type="props.type"
            :required="props.required"
            :disabled="props.disabled"
            :placeholder="props.placeholder"
            :value="props.value"
            data-name="label"
        />
    </div>
</template>
