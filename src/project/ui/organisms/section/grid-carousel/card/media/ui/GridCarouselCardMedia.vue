<script setup lang="ts">
import type { GridCarouselCardMediaProps } from "../data/types";
import CardMedia from "../../../../../../molecules/card/media/ui/CardMedia.vue";
import SectionGridCarousel from "../../../ui/SectionGridCarousel.vue";

const props = defineProps<GridCarouselCardMediaProps>();
</script>

<template>
  <SectionGridCarousel v-bind="props.sectionGridCarouselProps">
    <CardMedia
      v-for="(item, index) in props.items"
      :key="index"
      v-bind="item"
    />
  </SectionGridCarousel>
</template>
