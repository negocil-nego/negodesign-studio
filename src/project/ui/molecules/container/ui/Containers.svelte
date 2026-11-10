<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { ContainersProps } from "../data/types";

  let {
    title = "Hello, world!",
    lead = "This is a simple hero unit, a simple jumbotron-style component for calling extra attention to featured content or information.",
    body = "It uses utility classes for typography and spacing to space content out within the larger container.",
    actionLabel = "Learn more",
    onAction,
  }: ContainersProps = $props();
</script>

<div class={StyleComponent.containers.root}>
  <div class={StyleComponent.containers.jumbotron}>
    <h2 class={StyleComponent.containers.title}>{title}</h2>
    <p class={StyleComponent.containers.lead}>{lead}</p>
    <hr class={StyleComponent.containers.divider} />
    <p class={StyleComponent.containers.body}>{body}</p>
    {#if actionLabel}
      <div class={StyleComponent.containers.actionWrap}>
        <button
          type="button"
          class="btn btn-primary btn-lg"
          onclick={onAction}
        >
          {actionLabel}
        </button>
      </div>
    {/if}
  </div>
</div>
