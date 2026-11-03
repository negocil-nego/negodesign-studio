<script setup lang="ts">
import type { GridCarouselCardProfileProps } from "../data/types";
import CardProfile from "../../../../../../molecules/card/profile/ui/CardProfile.vue";
import SectionGridCarousel from "../../../ui/SectionGridCarousel.vue";

const props = defineProps<GridCarouselCardProfileProps>();
</script>

<template>
  <SectionGridCarousel v-bind="props.sectionGridCarouselProps">
    <CardProfile
      v-for="(item, index) in props.items"
      :key="index"
      v-bind="item"
    />
  </SectionGridCarousel>
</template>
