<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardDescriptionProps } from "../data/types";

const { description, className } = defineProps<CardDescriptionProps>();
</script>

<template>
    <div :class="['card-description', className]" id="card-description">
        <div :class="StyleComponent.card.description">
            {{ description }}
        </div>
    </div>
</template>
