<script setup lang="ts">
import type { AvatarProps } from "../data/types";
import Skeleton from "../../skeleton/ui/Skeleton.vue";
import { StyleComponent } from "../../../../data/style.component";

defineProps<AvatarProps>();
</script>

<template>
    <Skeleton v-if="isLoading" :class-name="className || StyleComponent.avatar.default" />
    <button v-else-if="src" :class="className" @click="onClick">
        <img :src="src" :alt="src" :class="className || StyleComponent.avatar.default" />
    </button>
</template>
