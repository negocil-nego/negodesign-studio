<script setup lang="ts">
import type { CardTagsProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";

const { tags, className, itemClassName } = defineProps<CardTagsProps>();
</script>

<template>
    <div :class="className ?? 'flex gap-4 w-full border-y border-gray-100 py-4'">
        <div
            v-for="(tag, index) in tags"
            :key="index"
            :class="itemClassName ?? 'flex flex-col items-center justify-center flex-1 first:border-l-0 border-l border-gray-100'"
        >
            <Icon v-if="tag.icon" :name="tag.icon" class="mb-1" />
            <span class="text-lg font-bold text-gray-900">{{ tag.title }}</span>
            <span v-if="tag.description" class="text-sm text-gray-500">{{ tag.description }}</span>
        </div>
    </div>
</template>
