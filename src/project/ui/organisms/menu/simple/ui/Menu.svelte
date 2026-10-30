<script lang="ts">
  import Button from "../../../../atoms/button/variant/default/ui/Button.svelte";
  import Icon from "../../../../atoms/icon/ui/Icon.svelte";
  import Image from "../../../../atoms/image/ui/Image.svelte";
  import SubmenuGridTitle from "../../shared/submenu/grid/title/ui/SubmenuGridTitle.svelte";
  import SubmenuSidebarGrid from "../../shared/submenu/grid/sidebar/ui/SubmenuSidebarGrid.svelte";
  import type { MenuProps, MenuTypeProps } from "../data/types";
  import type { SubmenuGridTitleProps } from "../../shared/submenu/grid/title/data/types";
  import type { SubmenuSidebarGridProps } from "../../shared/submenu/grid/sidebar/data/types";
  import { StyleComponent } from "../../../../../../components/local/data/style.component";

  let {
    logo,
    menus = [],
    buttonsProps = [],
    defaultOpenSubmenu = null,
  }: MenuProps = $props();
  // svelte-ignore state_referenced_locally
  let openSubmenu = $state<number | null>(defaultOpenSubmenu);
  let activeMenu = $derived(
    openSubmenu === null ? undefined : menus[openSubmenu],
  );

  function isSidebarSubmenu(menu: MenuTypeProps): menu is SubmenuSidebarGridProps {
    return "type" in menu && menu.type === "sidebar-grid";
  }

  function isGridSubmenu(menu: MenuTypeProps): menu is SubmenuGridTitleProps {
    return !isSidebarSubmenu(menu) && ("items" in menu || "title" in menu || "label" in menu);
  }

  function isSubmenu(menu: MenuTypeProps): menu is SubmenuGridTitleProps | SubmenuSidebarGridProps {
    return isSidebarSubmenu(menu) || isGridSubmenu(menu);
  }

  function getSubmenuLabel(menu: SubmenuGridTitleProps | SubmenuSidebarGridProps) {
    return isSidebarSubmenu(menu)
      ? menu.title?.title?.text ?? "Explorar"
      : menu.label ?? menu.title?.title?.text ?? "Explorar";
  }

  function toggleSubmenu(index: number) {
    openSubmenu = openSubmenu === index ? null : index;
  }

  let logoProps = $derived({
    ...logo,
    className:
      `${logo?.className ?? ""} ${StyleComponent.menu.navigation.logo}`.trim(),
  });
</script>

<nav class={StyleComponent.menu.navigation.root} aria-label="Menu principal">
  <div class={StyleComponent.menu.navigation.bar}>
    <div class={StyleComponent.menu.navigation.logoContainer}>
      {#if logo}<Image {...logoProps} />{/if}
    </div>

    <ul class={StyleComponent.menu.navigation.list}>
      {#each menus as menu, index (index)}
        <li class={StyleComponent.menu.navigation.item}>
          {#if isSubmenu(menu)}
            <button
              type="button"
              class={StyleComponent.menu.navigation.submenuTrigger}
              aria-expanded={openSubmenu === index}
              aria-haspopup="true"
              onkeydown={(event) => {
                if (event.key === "Escape") openSubmenu = null;
              }}
              onclick={() => toggleSubmenu(index)}
            >
              <span class={StyleComponent.menu.navigation.submenuLabel}
                >{getSubmenuLabel(menu)}</span
              >
              <Icon
                name={openSubmenu === index ? "ArrowUp" : "ArrowDown"}
                className={StyleComponent.menu.navigation.submenuIcon}
              />
            </button>
          {:else if menu.href}
            <a href={menu.href} class={StyleComponent.menu.navigation.link}>
              {#if menu.icon}
                <Icon
                  name={menu.icon.name}
                  isLoading={menu.icon.isLoading}
                  className={`${StyleComponent.menu.navigation.linkIcon} ${menu.icon.className ?? ""}`}
                />
              {/if}
              {#if menu.textProps}
                <span class={StyleComponent.menu.navigation.linkLabel}
                  >{menu.textProps.text}</span
                >
              {/if}
            </a>
          {:else}
            <button type="button" class={StyleComponent.menu.navigation.link}>
              {#if menu.icon}
                <Icon
                  name={menu.icon.name}
                  isLoading={menu.icon.isLoading}
                  className={`${StyleComponent.menu.navigation.linkIcon} ${menu.icon.className ?? ""}`}
                />
              {/if}
              {#if menu.textProps}
                <span class={StyleComponent.menu.navigation.linkLabel}
                  >{menu.textProps.text}</span
                >
              {/if}
            </button>
          {/if}
        </li>
      {/each}
    </ul>

    <div class={StyleComponent.menu.navigation.actions}>
      {#each buttonsProps as button, index (index)}
        <Button {...button} />
      {/each}
    </div>
  </div>

  {#if activeMenu && isSubmenu(activeMenu)}
    <div class={StyleComponent.menu.navigation.panel}>
      {#if isSidebarSubmenu(activeMenu)}
        <SubmenuSidebarGrid {...activeMenu} />
      {:else}
        <SubmenuGridTitle {...activeMenu} />
      {/if}
    </div>
  {/if}
</nav>
