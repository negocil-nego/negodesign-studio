<script setup lang="ts">
import type { GridCardProfileProps } from "../data/types";
import CardProfile from "../../../../../../molecules/card/profile/ui/CardProfile.vue";
import SectionGrid from "../../../ui/SectionGrid.vue";

const props = defineProps<GridCardProfileProps>();
</script>

<template>
  <SectionGrid v-bind="props.sectionGridProps">
    <CardProfile
      v-for="(item, index) in props.items"
      :key="index"
      v-bind="item"
    />
  </SectionGrid>
</template>
