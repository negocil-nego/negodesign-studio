<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { StyleComponent } from "../../../../data/style.component";
import type { PaginationNavProps } from "../data/types";

const props = withDefaults(defineProps<PaginationNavProps>(), {
  totalPages: 5,
  currentPage: 1,
  size: "md",
  disabledFirst: false,
});

const current = ref(props.currentPage);
watch(() => props.currentPage, (next) => {
  current.value = next;
});

const pages = computed(() =>
  Array.from({ length: props.totalPages }, (_, idx) => idx + 1),
);

function goTo(next: number) {
  if (next < 1 || next > props.totalPages) return;
  current.value = next;
  props.onPageChange?.(next);
}
</script>

<template>
  <nav aria-label="Pagination" :class="StyleComponent.pagination.nav">
    <ul :class="[StyleComponent.pagination.list, StyleComponent.pagination.size[props.size]]">
      <li :class="StyleComponent.pagination.item">
        <button
          type="button"
          :disabled="props.disabledFirst || current <= 1"
          :class="StyleComponent.pagination.link"
          @click="goTo(current - 1)"
        >
          &laquo;
        </button>
      </li>
      <li
        v-for="num in pages"
        :key="num"
        :class="[
          StyleComponent.pagination.item,
          num === current ? StyleComponent.pagination.itemActive : '',
        ]"
      >
        <button
          type="button"
          :class="[
            StyleComponent.pagination.link,
            num === current ? StyleComponent.pagination.linkActive : '',
          ]"
          @click="goTo(num)"
        >
          {{ num }}
        </button>
      </li>
      <li :class="StyleComponent.pagination.item">
        <button
          type="button"
          :disabled="current >= props.totalPages"
          :class="StyleComponent.pagination.link"
          @click="goTo(current + 1)"
        >
          &raquo;
        </button>
      </li>
    </ul>
  </nav>
</template>
