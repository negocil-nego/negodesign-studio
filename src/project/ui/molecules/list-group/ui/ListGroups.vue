<script setup lang="ts">
import { ref } from "vue";
import { StyleComponent } from "../../../../data/style.component";
import type { ListGroupEntry, ListGroupsProps } from "../data/types";

const props = withDefaults(defineProps<ListGroupsProps>(), {
  items: () => [
    { id: "item-1", label: "Cras justo odio", badge: 14, active: true },
    { id: "item-2", label: "Dapibus ac facilisis in", badge: 2 },
    { id: "item-3", label: "Morbi leo risus", badge: 1 },
  ],
});

const selectedId = ref<string | undefined>(props.activeId);

function isActive(item: ListGroupEntry): boolean {
  return selectedId.value !== undefined
    ? selectedId.value === item.id
    : Boolean(item.active);
}

function select(item: ListGroupEntry) {
  if (item.disabled) return;
  selectedId.value = item.id;
  props.onSelect?.(item.id);
}
</script>

<template>
  <ul :class="StyleComponent.listGroup.root">
    <li v-for="item in props.items" :key="item.id">
      <button
        type="button"
        :disabled="item.disabled"
        :class="[
          StyleComponent.listGroup.item,
          StyleComponent.listGroup.action,
          isActive(item) ? StyleComponent.listGroup.active : '',
          item.disabled ? StyleComponent.listGroup.disabled : '',
        ]"
        @click="select(item)"
      >
        <div v-if="item.description" class="w-full text-left">
          <div :class="StyleComponent.listGroup.richHeader">
            <span :class="StyleComponent.listGroup.richTitle">{{ item.label }}</span>
            <span v-if="item.meta" :class="StyleComponent.listGroup.richMeta">{{ item.meta }}</span>
          </div>
          <p :class="StyleComponent.listGroup.richBody">{{ item.description }}</p>
          <small v-if="item.footer" :class="StyleComponent.listGroup.richFooter">{{ item.footer }}</small>
        </div>
        <template v-else>
          <span>{{ item.label }}</span>
          <span v-if="item.badge !== undefined" class="badge rounded-pill text-bg-primary">
            {{ item.badge }}
          </span>
        </template>
      </button>
    </li>
  </ul>
</template>
