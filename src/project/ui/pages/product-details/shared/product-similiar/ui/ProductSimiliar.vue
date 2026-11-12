<script setup lang="ts">
import { computed } from "vue";
import CardProduct from "../../../../../molecules/card/product/ui/CardProduct.vue";
import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton.vue";
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.vue";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.vue";
import type { ProductSimiliarProps } from "../data/types";

const props = withDefaults(defineProps<ProductSimiliarProps>(), {
  variant: "grid",
  headerAlign: "left",
  titleProps: () => ({ title: "Similar Products" }),
  columns: 4,
  items: () => [],
  className: "",
});

const gridColsClass = computed(() =>
  props.columns === 5
    ? "grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5"
    : props.columns === 3
      ? "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3"
      : "grid-cols-1 sm:grid-cols-2 lg:grid-cols-4",
);
</script>

<template>
  <aside
    v-if="variant === 'sidebar'"
    :class="`space-y-4 rounded-2xl border border-blender-border bg-blender-card p-5 shadow-sm ${className}`"
  >
    <div v-if="titleProps" class="border-b border-blender-border pb-3 text-center">
      <CardTitle v-bind="titleProps" />
    </div>
    <div class="space-y-3">
      <CardProduct
        v-for="(item, idx) in items"
        :key="idx"
        v-bind="item"
        :variant="item.variant ?? '02'"
      />
    </div>
    <div v-if="moreButtonProps" class="pt-2 text-center">
      <CardButton variant="outline" v-bind="moreButtonProps" />
    </div>
  </aside>

  <section v-else :class="`space-y-6 ${className}`">
    <div
      :class="
        headerAlign === 'center'
          ? 'flex flex-col items-center text-center space-y-1.5'
          : 'flex flex-wrap items-end justify-between gap-4'
      "
    >
      <div>
        <CardTitle v-if="titleProps" v-bind="titleProps" />
        <CardDescription v-if="descriptionProps" v-bind="descriptionProps" />
      </div>
      <CardButton v-if="moreButtonProps" variant="outline" v-bind="moreButtonProps" />
    </div>

    <div :class="`grid gap-4 ${gridColsClass}`">
      <CardProduct
        v-for="(item, idx) in items"
        :key="idx"
        v-bind="item"
        :variant="item.variant ?? '01'"
      />
    </div>
  </section>
</template>
