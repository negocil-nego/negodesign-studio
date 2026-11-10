<script setup lang="ts">
import { StyleComponent } from "../../../../data/style.component";
import type { ContainersProps } from "../data/types";

const props = withDefaults(defineProps<ContainersProps>(), {
  title: "Hello, world!",
  lead: "This is a simple hero unit, a simple jumbotron-style component for calling extra attention to featured content or information.",
  body: "It uses utility classes for typography and spacing to space content out within the larger container.",
  actionLabel: "Learn more",
});
</script>

<template>
  <div :class="StyleComponent.containers.root">
    <div :class="StyleComponent.containers.jumbotron">
      <h2 :class="StyleComponent.containers.title">{{ props.title }}</h2>
      <p :class="StyleComponent.containers.lead">{{ props.lead }}</p>
      <hr :class="StyleComponent.containers.divider" />
      <p :class="StyleComponent.containers.body">{{ props.body }}</p>
      <div v-if="props.actionLabel" :class="StyleComponent.containers.actionWrap">
        <button type="button" class="btn btn-primary btn-lg" @click="props.onAction?.()">
          {{ props.actionLabel }}
        </button>
      </div>
    </div>
  </div>
</template>
