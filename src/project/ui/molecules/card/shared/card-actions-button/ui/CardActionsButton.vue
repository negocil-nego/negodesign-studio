<script setup lang="ts">
import OutlineButton from "../../../../../atoms/button/variant/outline/ui/OutlineButton.vue";
import type { CardActionsButtonProps } from "../data/types";

const { actions } = defineProps<CardActionsButtonProps>();
</script>

<template>
    <div class="flex gap-3 w-full">
        <OutlineButton
            v-for="(action, index) in actions"
            :key="index"
            :onClick="action.onClick"
            class="flex-1 justify-center py-2.5 rounded-2xl border-gray-200 text-gray-900 font-semibold flex items-center gap-2"
            :text="action.label"
            :icon="action.icon ? { name: action.icon, className: 'text-lg' } : undefined"
        />
    </div>
</template>
