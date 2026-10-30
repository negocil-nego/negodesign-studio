<script lang="ts">
  import MenuText from "../../../../menu/menu-grid/ui/MenuText.svelte";
  import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.svelte";
  import type { SubmenuTitleProps } from "../data/types";

  let { title, items }: SubmenuTitleProps = $props();
</script>

<div class="flex flex-col gap-3">
  {#if title}
    <MenuText {...title} />
  {/if}
  {#if items}
    <div class="grid grid-cols-1 md:grid-cols-2">
      {#each items as item}
        <MenuTitle {...item} />
      {/each}
    </div>
  {/if}
</div>
