<script setup lang="ts">
import ButtonBase from "../../../shared/base/ui/ButtonBase.vue";
import SolidButton from "../../solid/ui/SolidButton.vue";
import OutlineButton from "../../outline/ui/OutlineButton.vue";
import type { ButtonProps } from "../type/types";

defineOptions({ inheritAttrs: false });
const props = withDefaults(defineProps<ButtonProps>(), { variant: "solid" });
</script>

<template>
    <SolidButton v-if="props.variant === 'solid'" v-bind="$attrs" />
    <OutlineButton v-else-if="props.variant === 'outline'" v-bind="$attrs" />
    <ButtonBase v-else v-bind="$attrs" />
</template>
