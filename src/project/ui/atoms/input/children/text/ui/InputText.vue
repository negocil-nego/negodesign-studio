<script setup lang="ts">
import { computed } from "vue";
import Input from "../../../core/ui/Input.vue";
import InputLabel from "../../label/ui/InputLabel.vue";
import type { InputTextProps } from "../data/types";

const { labelProps, inputProps } = defineProps<InputTextProps>();

const textProps = computed(() => ({ ...inputProps, type: "text" as const, icon: inputProps.icon ?? "InputCursorText" }));
</script>

<template>
    <template v-if="labelProps">
        <InputLabel :labelProps="labelProps" :inputProps="textProps" />
    </template>
    <template v-else>
        <Input v-bind="textProps" />
    </template>
</template>
