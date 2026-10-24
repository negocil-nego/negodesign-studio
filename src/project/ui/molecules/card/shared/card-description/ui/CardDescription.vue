<script setup lang="ts">
import type { CardDescriptionProps } from "../data/types";

const { description, className } = defineProps<CardDescriptionProps>();
</script>

<template>
    <div :class="['card-description', className]" id="card-description">
        <div class="text-justify line-clamp-2 xl:line-clamp-3 2xl:line-clamp-4">
            {{ description }}
        </div>
    </div>
</template>
