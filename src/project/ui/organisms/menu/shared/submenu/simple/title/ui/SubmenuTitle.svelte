<script lang="ts">
  import MenuText from "../../../../menu/menu-grid/ui/MenuText.svelte";
  import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.svelte";
  import type { SubmenuTitleProps } from "../data/types";

  let { title, items }: SubmenuTitleProps = $props();
</script>

<div class="@container flex min-w-0 flex-col gap-3">
  {#if title}
    <MenuText {...title} />
  {/if}
  {#if items}
    <div
      class="grid min-w-0 grid-cols-1 @xl:grid-cols-[repeat(2,minmax(0,1fr))]"
    >
      {#each items as item}
        <MenuTitle {...item} />
      {/each}
    </div>
  {/if}
</div>
