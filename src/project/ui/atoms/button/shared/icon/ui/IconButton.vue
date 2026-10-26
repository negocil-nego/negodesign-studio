<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import Button from "../../../variant/default/ui/Button.vue";
import type { IconButtonProps } from "../data/types";

withDefaults(defineProps<IconButtonProps>(), { isLoading: false });
</script>

<template>
    <Button
        :class-name="StyleComponent.iconCenterButton.default"
        @click="onClick"
        :is-skeleton="isLoading"
        :icon="icon"
    />
</template>
