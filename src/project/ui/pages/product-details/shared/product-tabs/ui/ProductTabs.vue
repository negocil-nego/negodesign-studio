<script setup lang="ts">
import { computed, ref } from "vue";
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.vue";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.vue";
import UnderlineNav from "../../../../../molecules/underline-nav/ui/UnderlineNav.vue";
import type { ProductTabsProps } from "../data/types";

const props = withDefaults(defineProps<ProductTabsProps>(), {
  paragraphs: () => [],
  specs: () => [],
  highlights: () => [],
  tabContents: () => [],
  specsStyle: "inline",
  className: "",
});

const navItems = computed(
  () =>
    props.underlineNavProps?.items ?? [
      { value: "description", label: "Description" },
      { value: "reviews", label: "Reviews (150)" },
      { value: "support", label: "Support" },
    ],
);

const activeTab = ref(props.underlineNavProps?.value ?? "description");
const activeContent = computed(() => props.tabContents.find((t) => t.value === activeTab.value));
const currentTitle = computed(() => activeContent.value?.titleProps ?? props.titleProps);
const currentDescription = computed(() => activeContent.value?.descriptionProps ?? props.descriptionProps);
const currentParagraphs = computed(() => activeContent.value?.paragraphs ?? props.paragraphs);
const currentSpecs = computed(() => activeContent.value?.specs ?? props.specs);
const currentHighlights = computed(() => activeContent.value?.highlights ?? props.highlights);
</script>

<template>
  <div :class="`space-y-5 ${className}`">
    <UnderlineNav
      :items="navItems"
      :value="activeTab"
      :onValueChange="(val: string) => (activeTab = val)"
    />
    <div class="space-y-4 pt-1">
      <CardTitle v-if="currentTitle" v-bind="currentTitle" />
      <div
        v-if="currentSpecs.length > 0 && specsStyle === 'table'"
        class="divide-y divide-blender-border/60 rounded-xl border border-blender-border bg-blender-surface/40"
      >
        <div
          v-for="spec in currentSpecs"
          :key="spec.label"
          class="grid grid-cols-2 gap-4 px-4 py-2.5 text-xs"
        >
          <span class="text-zinc-400">{{ spec.label }}</span>
          <span class="font-medium text-zinc-200">{{ spec.value }}</span>
        </div>
      </div>
      <dl v-else-if="currentSpecs.length > 0" class="grid gap-2 text-xs sm:grid-cols-2">
        <div v-for="(spec, idx) in currentSpecs" :key="idx" class="flex items-baseline gap-2">
          <dt class="font-medium text-zinc-400">{{ spec.label }}:</dt>
          <dd class="text-zinc-200">{{ spec.value }}</dd>
        </div>
      </dl>
      <CardDescription v-if="currentDescription" v-bind="currentDescription" />
      <div v-if="currentParagraphs.length > 0" class="space-y-3 text-xs leading-relaxed text-zinc-400">
        <p v-for="p in currentParagraphs" :key="p">{{ p }}</p>
      </div>
      <ul v-if="currentHighlights.length > 0" class="list-disc space-y-1.5 pl-5 text-xs text-zinc-300">
        <li v-for="item in currentHighlights" :key="item">{{ item }}</li>
      </ul>
    </div>
  </div>
</template>
