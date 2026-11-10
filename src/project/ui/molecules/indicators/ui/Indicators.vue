<script setup lang="ts">
import { computed, ref } from "vue";
import { StyleComponent } from "../../../../data/style.component";
import type { IndicatorsProps } from "../data/types";

const props = withDefaults(defineProps<IndicatorsProps>(), {
  alerts: () => [
    {
      id: "warn-1",
      variant: "warning",
      heading: "Warning!",
      message: "Best check yo self, you're not looking too good.",
      dismissible: true,
    },
    {
      id: "success-1",
      variant: "success",
      message: "Well done! You successfully read this important alert message.",
      dismissible: true,
    },
  ],
  badges: () => [
    { label: "Primary", variant: "primary" },
    { label: "Secondary", variant: "secondary" },
    { label: "Success", variant: "success" },
    { label: "Danger", variant: "danger" },
    { label: "Warning", variant: "warning" },
    { label: "Info", variant: "info" },
  ],
});

const dismissed = ref<string[]>([]);
const visibleAlerts = computed(() =>
  props.alerts.filter((a) => !dismissed.value.includes(a.id)),
);

function dismiss(id: string) {
  dismissed.value.push(id);
  props.onDismissAlert?.(id);
}
</script>

<template>
  <div :class="StyleComponent.indicators.root">
    <div class="space-y-3">
      <div
        v-for="alert in visibleAlerts"
        :key="alert.id"
        :class="[StyleComponent.indicators.alert, `alert-${alert.variant}`]"
        role="alert"
      >
        <h4 v-if="alert.heading" :class="StyleComponent.indicators.alertHeading">
          {{ alert.heading }}
        </h4>
        <p>{{ alert.message }}</p>
        <button
          v-if="alert.dismissible !== false"
          type="button"
          :class="StyleComponent.indicators.alertClose"
          @click="dismiss(alert.id)"
        >
          &times;
        </button>
      </div>
    </div>
    <div :class="StyleComponent.indicators.badgesRow">
      <span
        v-for="(badge, idx) in props.badges"
        :key="idx"
        :class="[
          StyleComponent.indicators.badge,
          `text-bg-${badge.variant}`,
          badge.pill ? StyleComponent.indicators.badgePill : '',
        ]"
      >
        {{ badge.label }}
      </span>
    </div>
  </div>
</template>
