<script setup lang="ts">
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
        class="h-50 w-full rounded-md object-cover transition-opacity duration-500"
    />
</template>
