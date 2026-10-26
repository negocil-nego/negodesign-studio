<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardRatingProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";

const { rating } = defineProps<CardRatingProps>();

const MAX_STARS = 5;
const filled = Math.min(MAX_STARS, Math.max(0, Math.round(rating)));
</script>

<template>
    <div :class="StyleComponent.card.rating.root" role="img" :aria-label="`rating ${filled} de ${MAX_STARS}`">
        <Icon
            v-for="(_, index) in MAX_STARS"
            :key="index"
            name="Star"
            :class="index < filled ? StyleComponent.card.rating.filled : ''"
        />
    </div>
</template>
