<script setup lang="ts">
import { computed } from "vue";
import Input from "../../../core/ui/Input.vue";
import InputLabel from "../../label/ui/InputLabel.vue";
import type { InputTextProps } from "../data/types";

const { labelProps, inputProps } = defineProps<InputTextProps>();

const phoneProps = computed(() => ({ ...inputProps, type: "tel" as const }));
</script>

<template>
    <template v-if="labelProps">
        <InputLabel :labelProps="labelProps" :inputProps="phoneProps" />
    </template>
    <template v-else>
        <Input v-bind="phoneProps" />
    </template>
</template>