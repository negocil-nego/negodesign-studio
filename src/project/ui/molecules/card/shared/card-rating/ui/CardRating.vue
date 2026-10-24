<script setup lang="ts">
import type { CardRatingProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";

const { rating } = defineProps<CardRatingProps>();

const MAX_STARS = 5;
const filled = Math.min(MAX_STARS, Math.max(0, Math.round(rating)));
</script>

<template>
    <div class="card-rating flex gap-2" role="img" :aria-label="`rating ${filled} de ${MAX_STARS}`">
        <Icon
            v-for="(_, index) in MAX_STARS"
            :key="index"
            name="Star"
            :class="index < filled ? 'text-yellow-600' : ''"
        />
    </div>
</template>
