<script setup lang="ts">
import type { InputLabelProps } from "../data/types";
import Input from "../../../core/ui/Input.vue";
import Label from "../../../../label/ui/Label.vue";

const { labelProps, inputProps } = defineProps<InputLabelProps>();
</script>

<template>
    <Label v-bind="labelProps" />
    <Input v-bind="inputProps" />
</template>