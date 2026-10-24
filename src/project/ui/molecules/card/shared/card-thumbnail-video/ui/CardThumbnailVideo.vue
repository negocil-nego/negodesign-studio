<script setup lang="ts">
import { ref } from "vue";
import type { CardThumbnailVideoProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";
import CardImage from "../../card-image/ui/CardImage.vue";
import CardVideo from "../../card-video/ui/CardVideo.vue";

const { videoProps, imageProps, visibleButtonPlay = false } = defineProps<CardThumbnailVideoProps>();

const isHoverImage = ref(false);
</script>

<template>
    <div
        class="relative cursor-pointer"
        role="presentation"
        @mouseenter="isHoverImage = true"
        @mouseleave="isHoverImage = false"
    >
        <!-- Imagem pré-carregada (loading eager) — define a altura e fica sempre por baixo -->
        <CardImage v-bind="imageProps" loading="eager" />

        <!-- Vídeo montado desde o início: o hover apenas o torna visível -->
        <div :class="['absolute inset-0', { invisible: !isHoverImage }]">
            <CardVideo v-bind="videoProps" preload="auto" />
        </div>

        <div
            v-if="visibleButtonPlay"
            :class="['absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white/80 size-12 rounded-full flex items-center justify-center z-50', { invisible: isHoverImage }]"
        >
            <Icon name="Play" class="text-7xl" />
        </div>
    </div>
</template>
