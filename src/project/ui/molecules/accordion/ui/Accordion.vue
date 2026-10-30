<script setup lang="ts">
import { StyleComponent } from "../../../../data/style.component";
withDefaults(defineProps<{ title: string; count?: string; open?: boolean }>(), { open: false });
</script>
<template>
  <details :class="StyleComponent.accordion.root" :open="open">
    <summary :class="StyleComponent.accordion.summary">
      <span>{{ title }}</span>
      <span :class="StyleComponent.accordion.meta"><span v-if="count">{{ count }}</span><span :class="StyleComponent.accordion.indicator" aria-hidden="true">⌄</span></span>
    </summary>
    <div :class="StyleComponent.accordion.content"><slot /></div>
  </details>
</template>
