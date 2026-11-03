<script setup lang="ts">
import type { CarouselCardProfileProps } from "../data/types";
import CardProfile from "../../../../../../molecules/card/profile/ui/CardProfile.vue";
import SectionCarouselButtons from "../../../buttons/ui/SectionCarouselButtons.vue";

const props = defineProps<CarouselCardProfileProps>();
</script>

<template>
  <SectionCarouselButtons v-bind="props.sectionCarouselButtonsProps">
    <CardProfile
      v-for="(item, index) in props.items"
      :key="index"
      v-bind="item"
    />
  </SectionCarouselButtons>
</template>
