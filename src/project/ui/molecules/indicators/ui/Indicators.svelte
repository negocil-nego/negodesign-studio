<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { AlertItem, BadgeItem, IndicatorsProps } from "../data/types";

  const defaultAlerts: AlertItem[] = [
    {
      id: "warn-1",
      variant: "warning",
      heading: "Warning!",
      message: "Best check yo self, you're not looking too good.",
      linkText: "vel scelerisque nisl consectetur et",
      dismissible: true,
    },
    {
      id: "danger-1",
      variant: "danger",
      message: "Oh snap! Change a few things up and try submitting again.",
      dismissible: true,
    },
    {
      id: "success-1",
      variant: "success",
      message: "Well done! You successfully read this important alert message.",
      dismissible: true,
    },
    {
      id: "info-1",
      variant: "info",
      message: "Heads up! This alert needs your attention, but it's not super important.",
      dismissible: true,
    },
  ];

  const defaultBadges: BadgeItem[] = [
    { label: "Primary", variant: "primary" },
    { label: "Secondary", variant: "secondary" },
    { label: "Success", variant: "success" },
    { label: "Danger", variant: "danger" },
    { label: "Warning", variant: "warning" },
    { label: "Info", variant: "info" },
    { label: "Light", variant: "light" },
    { label: "Dark", variant: "dark" },
  ];

  let {
    alerts = defaultAlerts,
    badges = defaultBadges,
    onDismissAlert,
  }: IndicatorsProps = $props();

  let dismissedIds = $state<string[]>([]);
  const visibleAlerts = $derived(alerts.filter((a) => !dismissedIds.includes(a.id)));

  function dismiss(id: string) {
    dismissedIds = [...dismissedIds, id];
    onDismissAlert?.(id);
  }
</script>

<div class={StyleComponent.indicators.root}>
  <div class="space-y-3">
    {#each visibleAlerts as alert (alert.id)}
      <div class={`${StyleComponent.indicators.alert} alert-${alert.variant}`} role="alert">
        {#if alert.heading}
          <h4 class={StyleComponent.indicators.alertHeading}>{alert.heading}</h4>
        {/if}
        <p>
          {alert.message}
          {#if alert.linkText}
            <a href="#alert-link" class={StyleComponent.indicators.alertLink}>{alert.linkText}</a>
          {/if}
        </p>
        {#if alert.dismissible !== false}
          <button
            type="button"
            class={StyleComponent.indicators.alertClose}
            aria-label="Close"
            onclick={() => dismiss(alert.id)}
          >
            &times;
          </button>
        {/if}
      </div>
    {/each}
  </div>

  <div class={StyleComponent.indicators.badgesRow}>
    {#each badges as badge, idx (idx)}
      <span
        class={`
          ${StyleComponent.indicators.badge}
          text-bg-${badge.variant}
          ${badge.pill ? StyleComponent.indicators.badgePill : ""}
        `}
      >
        {badge.label}
      </span>
    {/each}
  </div>
</div>
