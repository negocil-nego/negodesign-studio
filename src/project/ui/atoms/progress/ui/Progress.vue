<script setup lang="ts">
import { computed } from "vue";
import { StyleComponent } from "../../../../data/style.component";
import type { ProgressBarSegment, ProgressProps } from "../data/types";

const props = withDefaults(defineProps<ProgressProps>(), {
  value: 25,
  variant: "primary",
  striped: false,
  animated: false,
  bars: () => [],
});

const resolvedBars = computed<ProgressBarSegment[]>(() =>
  props.bars.length
    ? props.bars
    : [
        {
          value: props.value,
          variant: props.variant,
          striped: props.striped,
          animated: props.animated,
        },
      ],
);
</script>

<template>
  <div :class="StyleComponent.progress.track" role="progressbar" :aria-valuemin="0" :aria-valuemax="100">
    <div
      v-for="(bar, idx) in resolvedBars"
      :key="idx"
      :class="[
        StyleComponent.progress.bar,
        `bg-${bar.variant ?? 'primary'}`,
        bar.striped || bar.animated ? StyleComponent.progress.striped : '',
        bar.animated ? StyleComponent.progress.animated : '',
      ]"
      :style="{ width: `${Math.max(0, Math.min(100, bar.value))}%` }"
    >
      {{ bar.label }}
    </div>
  </div>
</template>
