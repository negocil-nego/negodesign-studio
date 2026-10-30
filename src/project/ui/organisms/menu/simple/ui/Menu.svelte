<script lang="ts">
  import Button from "../../../../atoms/button/variant/default/ui/Button.svelte";
  import Image from "../../../../atoms/image/ui/Image.svelte";
  import MenuText from "../../shared/menu/menu-grid/ui/MenuText.svelte";
  import type { MenuProps } from "../data/types";

  let { logo, menus, buttonsProps }: MenuProps = $props();

  let logoProps = $derived({
    ...logo,
    className: "h-8 w-8",
  });
</script>

<nav class="flex justify-between items-center">
  {#if logo}
    <Image {...logoProps} />
  {/if}

  {#if menus}
    <ul class="flex items-center justify-center gap-4">
      {#each menus as menu, i (i)}
        <li class="flex items-center gap-2">
          <MenuText {...menu} />
        </li>
      {/each}
    </ul>
  {/if}

  {#if buttonsProps}
    <div class="flex items-center gap-2">
      {#each buttonsProps as button}
        <Button {...button} />
      {/each}
    </div>
  {/if}
</nav>
