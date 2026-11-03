<script setup lang="ts">
import type { CarouselCardMediaProps } from "../data/types";
import CardMedia from "../../../../../../molecules/card/media/ui/CardMedia.vue";
import SectionCarouselButtons from "../../../buttons/ui/SectionCarouselButtons.vue";

const props = defineProps<CarouselCardMediaProps>();
</script>

<template>
  <SectionCarouselButtons v-bind="props.sectionCarouselButtonsProps">
    <CardMedia
      v-for="(item, index) in props.items"
      :key="index"
      v-bind="item"
    />
  </SectionCarouselButtons>
</template>
