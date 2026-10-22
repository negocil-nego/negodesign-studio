<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    id: string;
    trigger: Snippet;
    panel: Snippet;
    role?: string;
    ariaLabel?: string;
  };

  let { id, trigger, panel, role = "dialog", ariaLabel }: Props = $props();
</script>

<div class="popover w-full">
  {@render trigger()}
  <div {id} popover="auto" {role} aria-label={ariaLabel}>
    {@render panel()}
  </div>
</div>
