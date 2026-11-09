<script lang="ts">
  import Button from "../../../../atoms/button/variant/default/ui/Button.svelte";
  import Icon from "../../../../atoms/icon/ui/Icon.svelte";
  import Image from "../../../../atoms/image/ui/Image.svelte";
  import SubmenuGridTitle from "../../shared/submenu/grid/title/ui/SubmenuGridTitle.svelte";
  import SubmenuSidebarGrid from "../../shared/submenu/grid/sidebar/ui/SubmenuSidebarGrid.svelte";
  import type { MenuProps, MenuTypeProps } from "../data/types";
  import type { SubmenuGridTitleProps } from "../../shared/submenu/grid/title/data/types";
  import type { SubmenuSidebarGridProps } from "../../shared/submenu/grid/sidebar/data/types";
  import { LocalStyleComponent } from "../../../../../../components/local/data/style.component";

  let {
    style = "default",
    logo,
    logoText,
    logoSubtitle,
    menus = [],
    buttonsProps = [],
    defaultOpenSubmenu = null,
  }: MenuProps = $props();
  // svelte-ignore state_referenced_locally
  let openSubmenu = $state<number | null>(defaultOpenSubmenu);
  let activeMenu = $derived(
    openSubmenu === null ? undefined : menus[openSubmenu],
  );
  let isGhost = $derived(style === "ghost" || style === "ghost-inverse");
  let isInverseGhost = $derived(style === "ghost-inverse");

  function isSidebarSubmenu(
    menu: MenuTypeProps,
  ): menu is SubmenuSidebarGridProps {
    return "type" in menu && menu.type === "sidebar-grid";
  }

  function isGridSubmenu(menu: MenuTypeProps): menu is SubmenuGridTitleProps {
    return (
      !isSidebarSubmenu(menu) &&
      ("items" in menu || "title" in menu || "label" in menu)
    );
  }

  function isSubmenu(
    menu: MenuTypeProps,
  ): menu is SubmenuGridTitleProps | SubmenuSidebarGridProps {
    return isSidebarSubmenu(menu) || isGridSubmenu(menu);
  }

  function getSubmenuLabel(
    menu: SubmenuGridTitleProps | SubmenuSidebarGridProps,
  ) {
    return isSidebarSubmenu(menu)
      ? (menu.title?.title?.text ?? "Explorar")
      : (menu.label ?? menu.title?.title?.text ?? "Explorar");
  }

  function toggleSubmenu(index: number) {
    openSubmenu = openSubmenu === index ? null : index;
  }

  let logoProps = $derived({
    ...logo,
    className:
      `${logo?.className ?? ""} ${LocalStyleComponent.menu.navigation.logo}`.trim(),
  });
</script>

<nav
  class={isGhost
    ? LocalStyleComponent.menu.navigation.ghostRoot
    : LocalStyleComponent.menu.navigation.root}
  aria-label="Menu principal"
>
  <div
    class={isGhost
      ? LocalStyleComponent.menu.navigation.ghostBar
      : LocalStyleComponent.menu.navigation.bar}
  >
    <div class={LocalStyleComponent.menu.navigation.logoContainer}>
      {#if logo}<Image {...logoProps} />{/if}
      {#if logoText}
        <div
          class={isInverseGhost
            ? LocalStyleComponent.menu.navigation.ghostInverseBrand
            : LocalStyleComponent.menu.navigation.ghostBrand}
        >
          <span class={LocalStyleComponent.menu.navigation.ghostBrandName}
            >{logoText}</span
          >
          {#if logoSubtitle}
            <span
              class={isInverseGhost
                ? LocalStyleComponent.menu.navigation.ghostInverseBrandSubtitle
                : LocalStyleComponent.menu.navigation.ghostBrandSubtitle}
              >{logoSubtitle}</span
            >
          {/if}
        </div>
      {/if}
    </div>

    <ul
      class={isGhost
        ? LocalStyleComponent.menu.navigation.ghostList
        : LocalStyleComponent.menu.navigation.list}
    >
      {#each menus as menu, index (index)}
        <li class={LocalStyleComponent.menu.navigation.item}>
          {#if isSubmenu(menu)}
            <button
              type="button"
              class={isInverseGhost
                ? LocalStyleComponent.menu.navigation.ghostInverseSubmenuTrigger
                : style === "ghost"
                  ? LocalStyleComponent.menu.navigation.ghostSubmenuTrigger
                  : LocalStyleComponent.menu.navigation.submenuTrigger}
              aria-expanded={openSubmenu === index}
              aria-haspopup="true"
              onkeydown={(event) => {
                if (event.key === "Escape") openSubmenu = null;
              }}
              onclick={() => toggleSubmenu(index)}
            >
              <span class={LocalStyleComponent.menu.navigation.submenuLabel}
                >{getSubmenuLabel(menu)}</span
              >
              <Icon
                name={openSubmenu === index ? "ArrowUp" : "ArrowDown"}
                className={LocalStyleComponent.menu.navigation.submenuIcon}
              />
            </button>
          {:else if menu.href}
            <a
              href={menu.href}
              class={isInverseGhost
                ? LocalStyleComponent.menu.navigation.ghostInverseLink
                : style === "ghost"
                  ? LocalStyleComponent.menu.navigation.ghostLink
                  : LocalStyleComponent.menu.navigation.link}
            >
              {#if menu.icon}
                <Icon
                  name={menu.icon.name}
                  isLoading={menu.icon.isLoading}
                  className={`${LocalStyleComponent.menu.navigation.linkIcon} ${menu.icon.className ?? ""}`}
                />
              {/if}
              {#if menu.textProps}
                <span class={LocalStyleComponent.menu.navigation.linkLabel}
                  >{menu.textProps.text}</span
                >
              {/if}
            </a>
          {:else}
            <button
              type="button"
              class={isInverseGhost
                ? LocalStyleComponent.menu.navigation.ghostInverseLink
                : style === "ghost"
                  ? LocalStyleComponent.menu.navigation.ghostLink
                  : LocalStyleComponent.menu.navigation.link}
            >
              {#if menu.icon}
                <Icon
                  name={menu.icon.name}
                  isLoading={menu.icon.isLoading}
                  className={`${LocalStyleComponent.menu.navigation.linkIcon} ${menu.icon.className ?? ""}`}
                />
              {/if}
              {#if menu.textProps}
                <span class={LocalStyleComponent.menu.navigation.linkLabel}
                  >{menu.textProps.text}</span
                >
              {/if}
            </button>
          {/if}
        </li>
      {/each}
    </ul>

    <div
      class={isGhost
        ? LocalStyleComponent.menu.navigation.ghostActions
        : LocalStyleComponent.menu.navigation.actions}
    >
      {#each buttonsProps as button, index (index)}
        <Button
          {...button}
          variant={isGhost ? "outline" : button.variant}
          className={isInverseGhost
            ? LocalStyleComponent.menu.navigation.ghostInverseButton
            : style === "ghost"
              ? LocalStyleComponent.menu.navigation.ghostButton
              : button.className}
        />
      {/each}
    </div>
  </div>

  {#if activeMenu && isSubmenu(activeMenu)}
    <div class={LocalStyleComponent.menu.navigation.panel}>
      {#if isSidebarSubmenu(activeMenu)}
        <SubmenuSidebarGrid {...activeMenu} />
      {:else}
        <SubmenuGridTitle {...activeMenu} />
      {/if}
    </div>
  {/if}
</nav>
