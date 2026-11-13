<script setup lang="ts">
import Icon from "../../../../../atoms/icon/ui/Icon.vue";
import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton.vue";
import CardCartButton from "../../../../../molecules/card/shared/card-cart-button/ui/CardCartButton.vue";
import CardFavoriteButton from "../../../../../molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.vue";
import CardTags from "../../../../../molecules/card/shared/card-tags/ui/CardTags.vue";
import ProductTabs from "../../product-tabs/ui/ProductTabs.vue";
import type { ProductInfoActionsProps } from "../data/types";

withDefaults(defineProps<ProductInfoActionsProps>(), {
  trustBadges: () => [],
});
</script>

<template>
  <div class="flex flex-wrap items-center gap-2.5 pt-1">
    <CardButton v-if="buttonProps" v-bind="buttonProps" />
    <CardButton v-if="secondaryButtonProps" variant="outline" v-bind="secondaryButtonProps" />
    <CardCartButton v-if="cartButtonProps" v-bind="cartButtonProps" />
    <CardFavoriteButton v-if="favoriteButtonProps" v-bind="favoriteButtonProps" />
  </div>

  <div v-if="trustBadges.length > 0" class="grid gap-2 pt-1 sm:grid-cols-3">
    <div
      v-for="b in trustBadges"
      :key="b.text"
      class="flex items-center gap-2 rounded-lg border border-blender-border/70 bg-blender-surface/50 px-3 py-2 text-[11px] text-zinc-300"
    >
      <Icon v-if="b.icon" :name="b.icon" className="size-4 text-blue-400" />
      <span>{{ b.text }}</span>
    </div>
  </div>

  <CardTags v-if="tagsProps" v-bind="tagsProps" />

  <div v-if="inlineTabsProps" class="border-t border-blender-border pt-4">
    <ProductTabs v-bind="inlineTabsProps" />
  </div>
</template>
