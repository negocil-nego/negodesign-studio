<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { AccordionProps } from "../data/types";

  let { title, count, open = false, children }: AccordionProps = $props();
</script>

<details class={StyleComponent.accordion.root} {open}>
  <summary class={StyleComponent.accordion.summary}>
    <span>{title}</span>
    <span class={StyleComponent.accordion.meta}>
      {#if count}<span>{count}</span>{/if}
      <span class={StyleComponent.accordion.indicator} aria-hidden="true">⌄</span>
    </span>
  </summary>
  <div class={StyleComponent.accordion.content}>
    {@render children()}
  </div>
</details>
