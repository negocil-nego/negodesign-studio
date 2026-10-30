<script lang="ts">
  import { StyleComponent } from "../../../../../../../../../components/local/data/style.component";
  import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.svelte";
  import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle.svelte";
  import type { SubmenuGridTitleProps } from "../data/types";

  let { title, items }: SubmenuGridTitleProps = $props();
</script>

<div class={StyleComponent.menu.submenu.gridTitle.root}>
  {#if title}
    <aside class={StyleComponent.menu.submenu.gridTitle.intro}>
      <MenuTitle {...title} variant="featured" />
    </aside>
  {/if}
  {#if items?.length}
    <div class={StyleComponent.menu.submenu.gridTitle.groups}>
      {#each items as item, index (index)}
        <div class={StyleComponent.menu.submenu.gridTitle.group}>
          <SubmenuTitle {...item} />
        </div>
      {/each}
    </div>
  {/if}
</div>
