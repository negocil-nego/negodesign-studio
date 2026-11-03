<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardTagsProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";

const { tags, variant = "stats", className, itemClassName, titleClassName, descriptionClassName } = defineProps<CardTagsProps>();
const tagStyle = variant === "buttons" ? StyleComponent.card.tags.buttons : StyleComponent.card.tags;
</script>

<template>
    <div :class="className ?? tagStyle.root">
        <div
            v-for="(tag, index) in tags"
            :key="index"
            :class="itemClassName ?? tagStyle.item"
        >
            <Icon v-if="tag.icon" :name="tag.icon" :class="tagStyle.icon" />
            <span :class="titleClassName ?? tagStyle.value">{{ tag.title }}</span>
            <span v-if="tag.description" :class="descriptionClassName ?? tagStyle.label">{{ tag.description }}</span>
        </div>
    </div>
</template>
