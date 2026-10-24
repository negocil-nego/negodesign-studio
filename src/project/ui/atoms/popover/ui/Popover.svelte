<script lang="ts">
  import type { Snippet } from "svelte";
  import { StyleComponent } from "../../../../data/style.component";

  type Props = {
    id: string;
    trigger: Snippet;
    panel: Snippet;
    role?: string;
    ariaLabel?: string;
  };

  let { id, trigger, panel, role = "dialog", ariaLabel }: Props = $props();
</script>

<div class={StyleComponent.popover.root}>
  {@render trigger()}
  <div {id} popover="auto" {role} aria-label={ariaLabel}>
    {@render panel()}
  </div>
</div>
