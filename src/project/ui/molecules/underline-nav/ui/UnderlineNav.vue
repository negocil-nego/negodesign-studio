<script setup lang="ts">
import { ref, watch } from "vue";
import { StyleComponent } from "../../../../data/style.component";
import type { UnderlineNavProps } from "../data/types";

const props = withDefaults(defineProps<UnderlineNavProps>(), {
  items: () => [
    { value: "active", label: "Active" },
    { value: "link-1", label: "Link" },
    { value: "link-2", label: "Link" },
    { value: "disabled", label: "Disabled", disabled: true },
  ],
  value: "active",
});

const activeValue = ref(props.value);
watch(() => props.value, (next) => {
  activeValue.value = next;
});

function select(next: string, disabled?: boolean) {
  if (disabled) return;
  activeValue.value = next;
  props.onValueChange?.(next);
}
</script>

<template>
  <ul :class="StyleComponent.underlineNav.root" role="tablist">
    <li v-for="item in props.items" :key="item.value" :class="StyleComponent.underlineNav.item">
      <button
        type="button"
        role="tab"
        :aria-selected="item.value === activeValue"
        :disabled="item.disabled"
        :class="[
          StyleComponent.underlineNav.link,
          item.value === activeValue ? StyleComponent.underlineNav.linkActive : '',
          item.disabled ? StyleComponent.underlineNav.linkDisabled : '',
        ]"
        @click="select(item.value, item.disabled)"
      >
        {{ item.label }}
      </button>
    </li>
  </ul>
</template>
