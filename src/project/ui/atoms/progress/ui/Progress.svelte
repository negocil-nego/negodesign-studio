<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { ProgressBarSegment, ProgressProps, ProgressVariant } from "../data/types";

  let {
    value = 25,
    variant = "primary",
    striped = false,
    animated = false,
    bars = [],
  }: ProgressProps = $props();

  const resolvedBars = $derived<ProgressBarSegment[]>(
    bars.length
      ? bars
      : [{ value, variant, striped, animated }],
  );

  function variantClass(v: ProgressVariant = "primary"): string {
    return `bg-${v}`;
  }
</script>

<div class={StyleComponent.progress.track} role="progressbar" aria-valuemin={0} aria-valuemax={100}>
  {#each resolvedBars as bar, idx (idx)}
    <div
      class={`
        ${StyleComponent.progress.bar}
        ${variantClass(bar.variant)}
        ${bar.striped || bar.animated ? StyleComponent.progress.striped : ""}
        ${bar.animated ? StyleComponent.progress.animated : ""}
      `}
      style={`width: ${Math.max(0, Math.min(100, bar.value))}%`}
    >
      {#if bar.label}{bar.label}{/if}
    </div>
  {/each}
</div>
