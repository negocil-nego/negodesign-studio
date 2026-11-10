<script lang="ts">
  import Button from "../../../../atoms/button/variant/default/ui/Button.svelte";
  import Icon from "../../../../atoms/icon/ui/Icon.svelte";
  import Image from "../../../../atoms/image/ui/Image.svelte";
  import SubmenuGridTitle from "../../shared/submenu/grid/title/ui/SubmenuGridTitle.svelte";
  import SubmenuSidebarGrid from "../../shared/submenu/grid/sidebar/ui/SubmenuSidebarGrid.svelte";
  import {
    getSubmenuLabel,
    isSidebarSubmenu,
    isSubmenu,
    resolveMenuClasses,
  } from "../../../../../hooks/menu.hook";
  import type { MenuProps } from "../data/types";

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
  const activeMenu = $derived(openSubmenu === null ? undefined : menus[openSubmenu]);
  const cls = $derived(resolveMenuClasses(style));
  const logoProps = $derived({
    ...logo,
    className: `${logo?.className ?? ""} ${cls.nav.logo}`.trim(),
  });
</script>

<nav class={cls.root} aria-label="Main menu">
  <div class={cls.bar}>
    <div class={cls.nav.logoContainer}>
      {#if logo}<Image {...logoProps} />{/if}
      {#if logoText}
        <div class={cls.brand}>
          <span class={cls.nav.ghostBrandName}>{logoText}</span>
          {#if logoSubtitle}<span class={cls.brandSubtitle}>{logoSubtitle}</span>{/if}
        </div>
      {/if}
    </div>

    <ul class={cls.list}>
      {#each menus as menu, index (index)}
        <li class={cls.nav.item}>
          {#if isSubmenu(menu)}
            <button type="button" class={cls.submenuTrigger} aria-expanded={openSubmenu === index} aria-haspopup="true"
              onkeydown={(e) => e.key === "Escape" && (openSubmenu = null)}
              onclick={() => (openSubmenu = openSubmenu === index ? null : index)}>
              <span class={cls.nav.submenuLabel}>{getSubmenuLabel(menu)}</span>
              <Icon name={openSubmenu === index ? "ArrowUp" : "ArrowDown"} className={cls.nav.submenuIcon} />
            </button>
          {:else if menu.href}
            <a href={menu.href} class={cls.link}>
              {#if menu.icon}<Icon name={menu.icon.name} isLoading={menu.icon.isLoading} className={`${cls.nav.linkIcon} ${menu.icon.className ?? ""}`} />{/if}
              {#if menu.textProps}<span class={cls.nav.linkLabel}>{menu.textProps.text}</span>{/if}
            </a>
          {:else}
            <button type="button" class={cls.link}>
              {#if menu.icon}<Icon name={menu.icon.name} isLoading={menu.icon.isLoading} className={`${cls.nav.linkIcon} ${menu.icon.className ?? ""}`} />{/if}
              {#if menu.textProps}<span class={cls.nav.linkLabel}>{menu.textProps.text}</span>{/if}
            </button>
          {/if}
        </li>
      {/each}
    </ul>

    <div class={cls.actions}>
      {#each buttonsProps as button, index (index)}
        <Button {...button} variant={cls.isGhost ? "outline" : button.variant} className={cls.buttonClass(button.className)} />
      {/each}
    </div>
  </div>

  {#if activeMenu && isSubmenu(activeMenu)}
    <div class={cls.nav.panel}>
      {#if isSidebarSubmenu(activeMenu)}
        <SubmenuSidebarGrid {...activeMenu} />
      {:else}
        <SubmenuGridTitle {...activeMenu} />
      {/if}
    </div>
  {/if}
</nav>
