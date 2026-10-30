<script lang="ts">
  import Button from "../../../../atoms/button/variant/default/ui/Button.svelte";
  import Icon from "../../../../atoms/icon/ui/Icon.svelte";
  import Image from "../../../../atoms/image/ui/Image.svelte";
  import SubmenuGridTitle from "../../shared/submenu/grid/title/ui/SubmenuGridTitle.svelte";
  import type { MenuProps, MenuTypeProps } from "../data/types";
  import type { SubmenuGridTitleProps } from "../../shared/submenu/grid/title/data/types";

  let { logo, menus = [], buttonsProps = [] }: MenuProps = $props();
  let openSubmenu = $state<number | null>(null);
  let activeMenu = $derived(openSubmenu === null ? undefined : menus[openSubmenu]);

  function isSubmenu(menu: MenuTypeProps): menu is SubmenuGridTitleProps {
    return "items" in menu || "title" in menu || "label" in menu;
  }

  function toggleSubmenu(index: number) {
    openSubmenu = openSubmenu === index ? null : index;
  }

  let logoProps = $derived({
    ...logo,
    className: `${logo?.className ?? "h-8 w-8"} shrink-0 object-contain`,
  });
</script>

<nav class="relative w-full @container" aria-label="Menu principal">
  <div class="grid min-h-16 grid-cols-[auto_minmax(0,1fr)] items-center gap-3 border-b border-blender-border bg-blender-card px-4 py-3 @xl:grid-cols-[auto_minmax(0,1fr)_auto] @xl:gap-5 @xl:px-8">
    <div class="flex min-w-0 items-center">
      {#if logo}<Image {...logoProps} />{/if}
    </div>

    <ul class="col-span-full row-start-2 flex min-w-0 items-center justify-center gap-1 border-t border-blender-border/60 pt-2 @xl:col-span-1 @xl:row-start-auto @xl:gap-3 @xl:border-0 @xl:pt-0">
      {#each menus as menu, index (index)}
        <li class="min-w-0">
          {#if isSubmenu(menu)}
            <button
              type="button"
              class="inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
              aria-expanded={openSubmenu === index}
              aria-haspopup="true"
              onkeydown={(event) => {
                if (event.key === "Escape") openSubmenu = null;
              }}
              onclick={() => toggleSubmenu(index)}
            >
              <span class="truncate">{menu.label ?? menu.title?.title?.text ?? "Explorar"}</span>
              <Icon
                name={openSubmenu === index ? "ArrowUp" : "ArrowDown"}
                className="h-3.5 w-3.5 shrink-0 opacity-70"
              />
            </button>
          {:else if menu.href}
            <a
              href={menu.href}
              class="inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
            >
              {#if menu.icon}<Icon {...menu.icon} />{/if}
              {#if menu.textProps}<span>{menu.textProps.text}</span>{/if}
            </a>
          {:else}
            <button
              type="button"
              class="inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
            >
              {#if menu.icon}<Icon {...menu.icon} />{/if}
              {#if menu.textProps}<span>{menu.textProps.text}</span>{/if}
            </button>
          {/if}
        </li>
      {/each}
    </ul>

    <div class="col-start-2 row-start-1 flex items-center justify-end gap-2 [&>button]:!w-auto">
      {#each buttonsProps as button, index (index)}
        <Button {...button} />
      {/each}
    </div>
  </div>

  {#if activeMenu && isSubmenu(activeMenu)}
    <div class="absolute inset-x-0 top-full z-40 overflow-hidden rounded-b-xl border-x border-b border-blender-border bg-blender-card shadow-2xl">
      <SubmenuGridTitle {...activeMenu} />
    </div>
  {/if}
</nav>
