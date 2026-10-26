<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardTitleProps } from "../data/types";

const { title, className } = defineProps<CardTitleProps>();
</script>

<template>
    <div :class="[StyleComponent.card.title.root, className]" id="card-title">
        <div :class="StyleComponent.card.title.text">{{ title }}</div>
    </div>
</template>
