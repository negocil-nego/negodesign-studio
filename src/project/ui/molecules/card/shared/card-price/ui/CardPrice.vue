<script setup lang="ts">
import type { CardPriceProps } from "../data/types";

const { number } = defineProps<CardPriceProps>();
</script>

<template>
    <div class="card-price" id="card-price">
        <div class="card-price-number">{{ number }}</div>
    </div>
</template>
