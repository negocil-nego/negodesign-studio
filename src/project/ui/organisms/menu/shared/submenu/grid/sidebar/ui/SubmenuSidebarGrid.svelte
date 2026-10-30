<script lang="ts">
  import { StyleComponent } from "../../../../../../../../../components/local/data/style.component";
  import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.svelte";
  import MenuText from "../../../../menu/menu-grid/ui/MenuText.svelte";
  import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle.svelte";
  import type { SubmenuSidebarGridProps } from "../data/types";

  let {
    title,
    items = [],
    defaultSelected = 0,
  }: SubmenuSidebarGridProps = $props();

  // svelte-ignore state_referenced_locally
  let selectedIndex = $state(
    Math.min(defaultSelected, Math.max(items.length - 1, 0)),
  );
  let activeItem = $derived(items[selectedIndex]);

  function selectItem(index: number) {
    selectedIndex = index;
  }
</script>

<div class={StyleComponent.menu.submenu.sidebarGrid.root}>
  <aside class={StyleComponent.menu.submenu.sidebarGrid.sidebar}>
    <p class={StyleComponent.menu.submenu.sidebarGrid.sidebarHeader}>Navegação</p>
    <nav
      class={StyleComponent.menu.submenu.sidebarGrid.nav}
      aria-label="Secções do submenu"
    >
      {#each items as item, index (index)}
        <button
          type="button"
          class="{StyleComponent.menu.submenu.sidebarGrid.navItem} {selectedIndex === index
            ? StyleComponent.menu.submenu.sidebarGrid.navItemActive
            : ''}"
          aria-current={selectedIndex === index ? "page" : undefined}
          onclick={() => selectItem(index)}
        >
          {#if item.title}
            <MenuText {...item.title} />
          {:else}
            <span>Secção {index + 1}</span>
          {/if}
        </button>
      {/each}
    </nav>
  </aside>

  <section class={StyleComponent.menu.submenu.sidebarGrid.content}>
    <div class={StyleComponent.menu.submenu.sidebarGrid.contentInner}>
      {#if title}
        <header class={StyleComponent.menu.submenu.sidebarGrid.contentHeader}>
          <MenuTitle {...title} variant="featured" />
        </header>
      {/if}

      {#if activeItem}
        <div class={StyleComponent.menu.submenu.sidebarGrid.groupContent}>
          <SubmenuTitle {...activeItem} />
        </div>
      {:else}
        <div class={StyleComponent.menu.submenu.sidebarGrid.empty}>
          Sem secções para apresentar.
        </div>
      {/if}
    </div>
  </section>
</div>
