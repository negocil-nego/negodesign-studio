<script lang="ts">
  import Button from "../../../../atoms/button/variant/default/ui/Button.svelte";
  import Icon from "../../../../atoms/icon/ui/Icon.svelte";
  import Image from "../../../../atoms/image/ui/Image.svelte";
  import Text from "../../../../atoms/text/ui/Text.svelte";
  import type { MenuSimpleProps } from "../data/types";

  let { logo, menus, buttonsProps }: MenuSimpleProps = $props();
</script>

<nav class="flex justify-between items-center">
  {#if logo}
    <Image {...logo} />
  {/if}

  {#if menus}
    <ul class="flex items-center justify-center gap-4">
      {#each menus as menu, i (i)}
        <li class="flex items-center gap-2">
          {#if menu.icon}
            <Icon {...menu.icon} />
          {/if}
          {#if menu.textProps}
            <Text {...menu.textProps} />
          {/if}
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
