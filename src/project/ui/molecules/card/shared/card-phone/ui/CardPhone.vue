<script setup lang="ts">
import type { CardPhoneProps } from "../data/types";

const { phone } = defineProps<CardPhoneProps>();
</script>

<template>
    <div class="card-phone" id="card-phone">
        <div class="card-phone-number">{{ phone }}</div>
    </div>
</template>
