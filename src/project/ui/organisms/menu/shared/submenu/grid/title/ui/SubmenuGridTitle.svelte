<script lang="ts">
  import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.svelte";
  import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle.svelte";
  import type { SubmenuGridTitleProps } from "../data/types";

  let { title, items }: SubmenuGridTitleProps = $props();
</script>

<div class="@container grid min-w-0 grid-cols-1 divide-y divide-blender-border @lg:grid-cols-[minmax(0,0.9fr)_minmax(0,2.1fr)] @lg:divide-x @lg:divide-y-0">
  {#if title}
    <aside class="flex min-w-0 flex-col items-start gap-4 p-6 @md:p-8 @lg:pr-10">
      <MenuTitle {...title} variant="featured" />
    </aside>
  {/if}
  {#if items?.length}
    <div class="grid min-w-0 grid-cols-1 gap-6 p-6 @xl:grid-cols-2 @xl:gap-5 @md:p-8">
      {#each items as item, index (index)}
        <div class="min-w-0">
          <SubmenuTitle {...item} />
        </div>
      {/each}
    </div>
  {/if}
</div>
