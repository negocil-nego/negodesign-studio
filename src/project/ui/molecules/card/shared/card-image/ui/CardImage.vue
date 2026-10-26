<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardImageProps } from "../data/types";

const { src, alt, srcset, sizes, loading = "lazy", fetchpriority } = defineProps<CardImageProps>();
</script>

<template>
    <img
        :src="src"
        :srcset="srcset"
        :sizes="sizes"
        :alt="alt"
        :fetchpriority="fetchpriority"
        :loading="loading"
        decoding="async"
        :class="`${StyleComponent.card.media.default} transition-opacity duration-500`"
    />
</template>
