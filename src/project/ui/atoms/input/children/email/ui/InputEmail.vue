<script setup lang="ts">
import { computed } from "vue";
import Input from "../../../core/ui/Input.vue";
import InputLabel from "../../label/ui/InputLabel.vue";
import type { InputEmailProps } from "../data/types";

const { labelProps, inputProps } = defineProps<InputEmailProps>();

const emailProps = computed(() => ({ ...inputProps, type: "email" as const }));
</script>

<template>
    <template v-if="labelProps">
        <InputLabel :labelProps="labelProps" :inputProps="emailProps" />
    </template>
    <template v-else>
        <Input v-bind="emailProps" />
    </template>
</template>
