<script setup lang="ts">
import { ref } from "vue";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";
import type { SectionCarouselButtonsProps } from "../data/types";

const props = withDefaults(defineProps<Omit<SectionCarouselButtonsProps, "children">>(), {
  className: "",
});
const viewport = ref<HTMLElement | null>(null);

function scroll(direction: -1 | 1) {
  const element = viewport.value;
  element?.scrollBy({ left: direction * element.clientWidth * 0.8, behavior: "smooth" });
}
</script>

<template>
  <section :class="`w-full min-w-0 py-6 ${props.className}`.trim()">
    <header class="mb-6 flex items-end justify-between gap-5">
      <div class="min-w-0">
        <h2 v-if="props.titleProps" class="text-2xl font-semibold leading-tight text-zinc-100 sm:text-3xl">{{ props.titleProps.text }}</h2>
        <p v-if="props.descriptionProps" class="mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base">{{ props.descriptionProps.text }}</p>
      </div>
      <div class="flex shrink-0 items-center gap-2">
        <button type="button" class="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300 transition-colors hover:border-blue-500/60 hover:bg-blender-hover hover:text-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500" aria-label="Itens anteriores" @click="scroll(-1)"><Icon name="ArrowLeft" className="size-4" /></button>
        <button type="button" class="flex size-10 items-center justify-center rounded-full border border-blender-border bg-blender-card text-zinc-300 transition-colors hover:border-blue-500/60 hover:bg-blender-hover hover:text-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500" aria-label="Itens seguintes" @click="scroll(1)"><Icon name="ArrowRight" className="size-4" /></button>
      </div>
    </header>
    <div ref="viewport" class="flex w-full snap-x snap-mandatory items-stretch gap-5 overflow-x-auto scroll-smooth [scrollbar-width:none] [&::-webkit-scrollbar]:hidden [&>*]:w-[min(17rem,78vw)] [&>*]:shrink-0 [&>*]:snap-start">
      <slot />
    </div>
  </section>
</template>
