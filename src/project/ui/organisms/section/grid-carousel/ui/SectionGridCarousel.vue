<script setup lang="ts">
import { ref } from "vue";
import Icon from "../../../../atoms/icon/ui/Icon.vue";
import type { SectionGridCarouselProps } from "../data/types";

const props = withDefaults(defineProps<Omit<SectionGridCarouselProps, "children">>(), {
  className: "",
  rows: 1,
  columns: 4,
});
const viewport = ref<HTMLElement | null>(null);

function scroll(direction: -1 | 1) {
  const element = viewport.value;
  element?.scrollBy({ left: direction * element.clientWidth, behavior: "smooth" });
}
</script>
<template>
  <section :class="`w-full min-w-0 py-6 ${props.className}`.trim()">
    <header class="mb-6 flex items-end justify-between gap-5">
      <div class="min-w-0">
        <h2 v-if="props.titleProps" class="text-2xl font-semibold leading-tight text-zinc-100 sm:text-3xl">
          {{ props.titleProps.text }}
        </h2>
        <p v-if="props.descriptionProps" class="mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base">
          {{ props.descriptionProps.text }}
        </p>
      </div>
      <div class="flex shrink-0 gap-2">
        <button
          type="button"
          aria-label="Itens anteriores"
          class="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300"
          @click="scroll(-1)"
        >
          <Icon name="ArrowLeft" className="size-4" />
        </button>
        <button
          type="button"
          aria-label="Itens seguintes"
          class="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300"
          @click="scroll(1)"
        >
          <Icon name="ArrowRight" className="size-4" />
        </button>
      </div>
    </header>
    <div
      ref="viewport"
      class="grid grid-flow-col gap-5 overflow-x-auto snap-x snap-mandatory scroll-smooth [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
      :style="{
        gridTemplateRows: `repeat(${Math.max(1, Math.floor(props.rows))}, minmax(0, 1fr))`,
        gridAutoColumns: `calc((100% - ${(Math.max(1, Math.floor(props.columns)) - 1) * 20}px) / ${Math.max(1, Math.floor(props.columns))})`,
      }"
    >
      <slot />
    </div>
  </section>
</template>
