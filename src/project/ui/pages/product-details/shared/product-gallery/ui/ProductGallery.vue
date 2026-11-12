<script setup lang="ts">
import { computed, ref } from "vue";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";
import CardImage from "../../../../../molecules/card/shared/card-image/ui/CardImage.vue";
import CardMinFile from "../../../../../molecules/card/shared/card-min-file/ui/CardMinFile.vue";
import type { ProductGalleryProps } from "../data/types";

const props = withDefaults(defineProps<ProductGalleryProps>(), {
  layout: "bottom-thumbs",
  showArrows: false,
  minFileProps: () => [],
  galleryImages: () => [],
  className: "",
});

const activeIdx = ref(0);

const allThumbs = computed(() =>
  props.minFileProps.length > 0
    ? props.minFileProps
    : props.galleryImages.map((img, i) => ({
        imageProps: img,
        active: i === activeIdx.value,
      })),
);

const currentImage = computed(
  () =>
    allThumbs.value[activeIdx.value]?.imageProps ??
    props.imageProps ?? {
      src: "/img/ux-01.jpg",
      alt: "Product preview",
    },
);

function selectIndex(idx: number) {
  activeIdx.value = idx;
}
</script>

<template>
  <div v-if="layout === 'left-thumbs'" :class="`grid grid-cols-1 gap-4 sm:grid-cols-[5.5rem_minmax(0,1fr)] ${className}`">
    <div v-if="allThumbs.length > 0" class="order-2 flex gap-2.5 overflow-x-auto sm:order-1 sm:flex-col">
      <div v-for="(thumb, idx) in allThumbs" :key="idx" class="size-20 shrink-0">
        <CardMinFile
          :imageProps="thumb.imageProps"
          :active="activeIdx === idx"
          :onClick="() => selectIndex(idx)"
        />
      </div>
    </div>
    <div class="order-1 relative aspect-square w-full overflow-hidden rounded-2xl border border-blender-border bg-blender-surface sm:order-2">
      <CardImage v-bind="currentImage" className="h-full w-full object-cover" />
    </div>
  </div>
  <div v-else :class="`space-y-3.5 ${className}`">
    <div class="relative aspect-[4/3.6] w-full overflow-hidden rounded-2xl border border-blender-border bg-blender-surface">
      <CardImage v-bind="currentImage" className="h-full w-full object-cover" />
      <button
        v-if="showArrows && allThumbs.length > 1"
        type="button"
        class="absolute left-3 top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-lg border border-blender-border bg-blender-card/90 text-zinc-200"
        @click="selectIndex((activeIdx - 1 + allThumbs.length) % allThumbs.length)"
      >
        <Icon name="ChevronLeft" className="size-4" />
      </button>
    </div>
    <div v-if="allThumbs.length > 0" class="grid grid-cols-4 gap-2.5 sm:grid-cols-5">
      <button
        v-for="(thumb, idx) in allThumbs"
        :key="idx"
        type="button"
        :class="`relative aspect-square overflow-hidden rounded-xl border bg-blender-surface ${
          activeIdx === idx ? 'border-blue-500 ring-2 ring-blue-500/30' : 'border-blender-border'
        }`"
        @click="selectIndex(idx)"
      >
        <CardImage v-if="thumb.imageProps" v-bind="thumb.imageProps" className="h-full w-full object-cover" />
      </button>
    </div>
  </div>
</template>
