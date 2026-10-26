<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import ButtonBase from "../../../shared/base/ui/ButtonBase.vue";
import type { SolidButtonProps } from "../data/types";

const props = defineProps<SolidButtonProps>();
</script>

<template>
    <ButtonBase :class-name="StyleComponent.button.solid.default" v-bind="props" />
</template>
