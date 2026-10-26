<script setup lang="ts">
import { ref } from "vue";
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardThumbnailVideoProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";
import CardImage from "../../card-image/ui/CardImage.vue";
import CardVideo from "../../card-video/ui/CardVideo.vue";

const { videoProps, imageProps, visibleButtonPlay = false } = defineProps<CardThumbnailVideoProps>();

const isHoverImage = ref(false);
</script>

<template>
    <div
        :class="StyleComponent.card.cardThumbnailVideo.container"
        role="presentation"
        @mouseenter="isHoverImage = true"
        @mouseleave="isHoverImage = false"
    >
        <!-- Imagem pré-carregada (loading eager) — define a altura e fica sempre por baixo -->
        <CardImage v-bind="imageProps" loading="eager" />

        <!-- Vídeo montado desde o início com preload="auto": ao passar o rato já está pré-carregado e a reproduzir, pelo que o hover apenas o torna visível. -->
        <div :class="[StyleComponent.card.cardThumbnailVideo.video, !isHoverImage ? 'invisible' : '']">
            <CardVideo v-bind="videoProps" preload="auto" />
        </div>

        <div
            v-if="visibleButtonPlay"
            :class="[StyleComponent.card.cardThumbnailVideo.playButton, isHoverImage ? 'invisible' : '']"
        >
            <Icon name="Play" :class="StyleComponent.card.cardThumbnailVideo.playIcon" />
        </div>
    </div>
</template>
