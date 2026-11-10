<script setup lang="ts">
import { StyleComponent } from "../../../../data/style.component";
import type { BreadcrumbItem, BreadcrumbsProps } from "../data/types";

const props = withDefaults(defineProps<BreadcrumbsProps>(), {
  items: () => [
    { label: "Home", href: "#home" },
    { label: "Library", href: "#library" },
    { label: "Data", active: true },
  ],
});

function handleClick(item: BreadcrumbItem, idx: number, event: MouseEvent) {
  if (props.onSelect) {
    event.preventDefault();
    props.onSelect(item, idx);
  }
}
</script>

<template>
  <nav aria-label="breadcrumb" :class="StyleComponent.breadcrumb.nav">
    <ol :class="StyleComponent.breadcrumb.list">
      <li
        v-for="(item, idx) in props.items"
        :key="idx"
        :class="[
          StyleComponent.breadcrumb.item,
          idx === props.items.length - 1 || item.active
            ? StyleComponent.breadcrumb.itemActive
            : '',
        ]"
      >
        <a
          v-if="idx !== props.items.length - 1 && !item.active && item.href"
          :href="item.href"
          :class="StyleComponent.breadcrumb.link"
          @click="handleClick(item, idx, $event)"
        >
          {{ item.label }}
        </a>
        <span v-else>{{ item.label }}</span>
      </li>
    </ol>
  </nav>
</template>
