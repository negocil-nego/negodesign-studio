<script setup lang="ts">
import { onMounted, ref } from "vue";
import type { CardVideoProps } from "../data/types";

const { src, poster, preload = "metadata", controls = false } = defineProps<CardVideoProps>();

const video = ref<HTMLVideoElement | null>(null);

function playVideo() {
    if (!video.value) return;
    // O WebKit ignora por vezes o atributo `muted`; forçar a propriedade é o que permite o autoplay
    video.value.muted = true;
    video.value.play().catch(() => {});
}

onMounted(playVideo);
</script>

<template>
    <video
        ref="video"
        :src="src"
        :poster="poster"
        :preload="preload"
        :controls="controls"
        autoplay
        muted
        loop
        playsinline
        @canplay="playVideo"
        class="h-50 w-full rounded-md object-cover"
    />
</template>
