<script setup lang="ts">
import { computed } from "vue";
import { getIconClass, IconNamed } from "../../../../../hooks/icon.svelte";
import { StyleComponent } from "../../../../data/style.component";
import type { IconProps } from "../data/types";

const props = defineProps<IconProps>();

const iconClass = computed(() => getIconClass(IconNamed[props.name]));
</script>

<template>
    <span v-if="props.isLoading" :class="StyleComponent.icon.spinner"></span>
    <i v-else :class="`${iconClass} ${props.className} ${StyleComponent.icon.default}`" aria-hidden="true" />
</template>
