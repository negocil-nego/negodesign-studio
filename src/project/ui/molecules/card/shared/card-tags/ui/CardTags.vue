<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardTagsProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";

const { tags, className, itemClassName } = defineProps<CardTagsProps>();
</script>

<template>
    <div :class="className ?? StyleComponent.card.tags.root">
        <div
            v-for="(tag, index) in tags"
            :key="index"
            :class="itemClassName ?? StyleComponent.card.tags.item"
        >
            <Icon v-if="tag.icon" :name="tag.icon" :class="StyleComponent.card.tags.icon" />
            <span :class="StyleComponent.card.tags.value">{{ tag.title }}</span>
            <span v-if="tag.description" :class="StyleComponent.card.tags.label">{{ tag.description }}</span>
        </div>
    </div>
</template>
