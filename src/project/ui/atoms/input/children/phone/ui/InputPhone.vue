<script setup lang="ts">
import { computed } from "vue";
import Input from "../../../core/ui/Input.vue";
import InputLabel from "../../label/ui/InputLabel.vue";
import type { InputPhoneProps } from "../data/types";

const { labelProps, inputProps } = defineProps<InputPhoneProps>();

const phoneProps = computed(() => ({ ...inputProps, type: "tel" as const, icon: inputProps.icon ?? "Phone" }));
</script>

<template>
    <template v-if="labelProps">
        <InputLabel :labelProps="labelProps" :inputProps="phoneProps" />
    </template>
    <template v-else>
        <Input v-bind="phoneProps" />
    </template>
</template>
