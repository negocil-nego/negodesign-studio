<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import OutlineButton from "../../../../../atoms/button/variant/outline/ui/OutlineButton.vue";
import type { CardActionsButtonProps } from "../data/types";

const { actions } = defineProps<CardActionsButtonProps>();
</script>

<template>
    <div :class="StyleComponent.card.cardActionsButton.root">
        <OutlineButton
            v-for="(action, index) in actions"
            :key="index"
            :onClick="action.onClick"
            :class="StyleComponent.card.cardActionsButton.button"
            :text="action.label"
            :icon="action.icon ? { name: action.icon, className: 'text-lg' } : undefined"
        />
    </div>
</template>
