<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardPriceProps } from "../data/types";

const { number } = defineProps<CardPriceProps>();
</script>

<template>
    <div :class="StyleComponent.card.price.root" id="card-price">
        <div :class="StyleComponent.card.price.number">{{ number }}</div>
    </div>
</template>
