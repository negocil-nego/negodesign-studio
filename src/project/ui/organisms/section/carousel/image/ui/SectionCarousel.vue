<script setup lang="ts">
import { computed, ref } from "vue";
import { getCarouselBackground } from "../data/background";
import type { SectionImageCarouselProps } from "../data/types";

const props = withDefaults(defineProps<SectionImageCarouselProps>(), {
  items: () => [],
  className: "",
  backgroundColor: "",
});
const activeIndex = ref(0);
const currentIndex = computed(() => props.items.length ? Math.min(activeIndex.value, props.items.length - 1) : 0);
const activeItem = computed(() => props.items[currentIndex.value]);
const background = computed(() => getCarouselBackground(props.backgroundColor));
</script>

<template>
  <section
    v-if="activeItem"
    :class="`relative w-full overflow-hidden rounded-xl border border-blender-border bg-blender-card ${background.backgroundClass} ${props.className}`.trim()"
    :style="background.isCssColor ? { backgroundColor: props.backgroundColor } : undefined"
    aria-label="Carousel"
    aria-roledescription="carousel"
  >
    <div
      class="relative flex aspect-[4/5] min-h-[28rem] w-full items-end justify-center sm:aspect-[5/6]"
      aria-live="polite"
      aria-atomic="true"
      role="group"
      aria-roledescription="slide"
      :aria-label="`Slide ${currentIndex + 1} de ${props.items.length}`"
    >
      <div v-if="activeItem.backgroundImage" class="absolute inset-0 bg-cover bg-center bg-no-repeat" :style="{ backgroundImage: `url('${activeItem.backgroundImage}')` }" aria-hidden="true" />
      <div v-if="background.isCssGradient || background.isGradientClass" :class="`absolute inset-0 ${background.backgroundGradientClass}`" :style="background.isCssGradient ? { backgroundImage: props.backgroundColor } : undefined" aria-hidden="true" />
      <div class="pointer-events-none absolute inset-0 bg-gradient-to-b from-transparent via-black/15 to-black/70" aria-hidden="true" />
      <div class="relative z-10 mx-auto flex w-full max-w-xl flex-col items-center px-6 pb-14 text-center text-white sm:px-10 sm:pb-16">
        <h2 v-if="activeItem.title?.text" class="text-2xl font-semibold leading-tight drop-shadow-md sm:text-3xl">
          {{ activeItem.title.text }}
        </h2>
        <p v-if="activeItem.description?.text" class="mt-2 max-w-lg text-sm leading-relaxed text-white/80 drop-shadow sm:text-base">
          {{ activeItem.description.text }}
        </p>
      </div>
      <div v-if="props.items.length > 1" class="absolute inset-x-0 bottom-5 z-20 flex items-center justify-center gap-2" aria-label="Slides">
        <button
          v-for="(_, index) in props.items"
          :key="index"
          type="button"
          :class="`h-2 rounded-full border border-white/80 bg-black/25 transition-all hover:bg-white/70 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white ${currentIndex === index ? 'w-5 bg-white' : 'w-2'}`"
          :aria-label="`Mostrar slide ${index + 1}`"
          :aria-current="currentIndex === index ? 'true' : undefined"
          @click="activeIndex = index"
        />
      </div>
    </div>
  </section>
</template>
