<script setup lang="ts">
import type { GridCardMediaProps } from "../data/types";
import CardMedia from "../../../../../../molecules/card/media/ui/CardMedia.vue";
import SectionGrid from "../../../ui/SectionGrid.vue";

const props = defineProps<GridCardMediaProps>();
</script>

<template>
  <SectionGrid v-bind="props.sectionGridProps">
    <CardMedia
      v-for="(item, index) in props.items"
      :key="index"
      v-bind="item"
    />
  </SectionGrid>
</template>
