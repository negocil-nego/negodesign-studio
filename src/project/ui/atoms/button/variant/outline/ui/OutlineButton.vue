<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import ButtonBase from "../../../shared/base/ui/ButtonBase.vue";
import type { OutlineButtonProps } from "../data/types";

const props = defineProps<OutlineButtonProps>();
</script>

<template>
    <ButtonBase :class-name="StyleComponent.button.outline.default" v-bind="props" />
</template>
