<script lang="ts">
  import MenuText from "../../../../menu/menu-grid/ui/MenuText.svelte";
  import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle.svelte";
  import type { SubmenuGridTitleProps } from "../data/types";

  let { title, items }: SubmenuGridTitleProps = $props();
</script>

<div class="flex flex-col gap-3">
  {#if title}
    <MenuText {...title} />
  {/if}
  {#if items}
    <div class="grid grid-cols-1 md:grid-cols-2">
      {#each items as item}
        <SubmenuTitle {...item} />
      {/each}
    </div>
  {/if}
</div>
