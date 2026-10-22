<script setup lang="ts">
import { computed } from "vue";
import { getIconClass, IconNamed } from "../../../../../hooks/icon.svelte";
import type { IconProps } from "../data/types";

const props = defineProps<IconProps>();

const iconClass = computed(() => getIconClass(IconNamed[props.name]));
</script>

<template>
  <i :class="[iconClass, 'text-base opacity-75']" aria-hidden="true" />
</template>
