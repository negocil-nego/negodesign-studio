<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardMinFileProps } from "../data/types";
import CardImage from "../../card-image/ui/CardImage.vue";

const { imageProps, active = false, onClick } = defineProps<CardMinFileProps>();
</script>

<template>
    <button
        type="button"
        :class="[StyleComponent.card.cardMinFile.root, active ? StyleComponent.card.cardMinFile.active : '']"
        @click="onClick?.()"
    >
        <CardImage v-if="imageProps" v-bind="imageProps" />
    </button>
</template>
