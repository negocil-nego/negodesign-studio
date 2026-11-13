<script setup lang="ts">
import type { ProductInfoOptionsProps } from "../data/types";

withDefaults(defineProps<ProductInfoOptionsProps>(), {
  colorsLabel: "Colors",
  colors: () => [],
  sizesLabel: "Size",
  sizes: () => [],
  showQuantity: false,
  quantity: 1,
  selectedColor: "",
  selectedSize: "",
});
const emit = defineEmits<{
  (e: "update:quantity", v: number): void;
  (e: "update:selectedColor", v: string): void;
  (e: "update:selectedSize", v: string): void;
}>();
</script>

<template>
  <div v-if="colors.length > 0" class="space-y-2">
    <span class="block text-xs font-semibold text-zinc-300">{{ colorsLabel }}</span>
    <div class="flex flex-wrap items-center gap-2.5">
      <button
        v-for="c in colors"
        :key="c.value"
        type="button"
        :style="{ backgroundColor: c.hex }"
        class="flex size-7 items-center justify-center rounded-md border border-blender-border text-[10px] font-bold text-white"
        @click="emit('update:selectedColor', c.value)"
      >
        {{ selectedColor === c.value ? "✓" : "" }}
      </button>
    </div>
  </div>

  <div v-if="sizes.length > 0" class="space-y-2">
    <span class="block text-xs font-semibold text-zinc-300">{{ sizesLabel }}</span>
    <div class="flex flex-wrap items-center gap-2">
      <button
        v-for="s in sizes"
        :key="s.value"
        type="button"
        :class="`rounded-lg border px-3 py-1.5 text-xs font-medium ${
          selectedSize === s.value
            ? 'border-blue-500 bg-blue-600 text-white'
            : 'border-blender-border bg-blender-surface text-zinc-300'
        }`"
        @click="emit('update:selectedSize', s.value)"
      >
        {{ s.priceLabel ? `${s.label} = ${s.priceLabel}` : s.label }}
      </button>
    </div>
  </div>

  <div v-if="showQuantity" class="flex items-center gap-4">
    <div class="inline-flex items-center rounded-lg border border-blender-border bg-blender-surface">
      <button type="button" class="px-3 py-1.5 text-xs font-bold text-zinc-300" @click="emit('update:quantity', Math.max(1, quantity - 1))">−</button>
      <span class="min-w-8 text-center text-xs font-semibold text-zinc-100">{{ quantity }}</span>
      <button type="button" class="px-3 py-1.5 text-xs font-bold text-zinc-300" @click="emit('update:quantity', quantity + 1)">+</button>
    </div>
  </div>
</template>
