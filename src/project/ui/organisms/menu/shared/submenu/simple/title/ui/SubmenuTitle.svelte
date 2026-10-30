<script lang="ts">
  import { StyleComponent } from "../../../../../../../../../components/local/data/style.component";
  import MenuText from "../../../../menu/menu-grid/ui/MenuText.svelte";
  import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.svelte";
  import type { SubmenuTitleProps } from "../data/types";

  let { title, items }: SubmenuTitleProps = $props();
</script>

<div class={StyleComponent.menu.submenu.title.root}>
  {#if title}
    <MenuText {...title} />
  {/if}
  {#if items}
    <div class={StyleComponent.menu.submenu.title.groups}>
      {#each items as item}
        <MenuTitle {...item} />
      {/each}
    </div>
  {/if}
</div>
