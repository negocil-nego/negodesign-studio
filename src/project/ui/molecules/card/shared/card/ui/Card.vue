<script setup lang="ts">
import { useAttrs } from "vue";
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardProps } from "../data/types";

defineProps<CardProps>();
const attrs = useAttrs();
</script>

<template>
    <div :class="StyleComponent.card.root" id="card">
        <div v-bind="attrs" />
    </div>
</template>
