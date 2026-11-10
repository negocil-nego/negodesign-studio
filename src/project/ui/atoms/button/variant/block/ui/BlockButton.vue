<script setup lang="ts">
import { computed } from "vue";
import { StyleComponent } from "../../../../../../data/style.component";
import ButtonBase from "../../../shared/base/ui/ButtonBase.vue";
import type { BlockButtonProps } from "../data/types";

const props = withDefaults(defineProps<BlockButtonProps>(), {
  variant: "primary",
  className: "",
});

const resolvedClass = computed(() => {
  const base =
    props.variant === "outline"
      ? `${StyleComponent.button.outline.default} btn-lg btn-block`
      : props.variant === "secondary"
        ? `${StyleComponent.button.default} btn-secondary btn-lg btn-block`
        : StyleComponent.button.block.default;
  return `${base} ${props.className ?? ""}`.trim();
});
</script>

<template>
  <ButtonBase v-bind="props" :class-name="resolvedClass" />
</template>
