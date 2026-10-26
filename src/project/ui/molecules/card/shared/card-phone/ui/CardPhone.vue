<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardPhoneProps } from "../data/types";

const { phone } = defineProps<CardPhoneProps>();
</script>

<template>
    <div :class="StyleComponent.card.phone.root" id="card-phone">
        <div :class="StyleComponent.card.phone.number">{{ phone }}</div>
    </div>
</template>
