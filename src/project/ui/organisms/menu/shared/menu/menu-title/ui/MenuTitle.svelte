<script lang="ts">
  import Text from "../../../../../atoms/text/ui/Text.svelte";
  import type { MenuTitleProps } from "../data/types";

  let { title, link, description }: MenuTitleProps = $props();
</script>

<div class="flex flex-col gap-3">
  {#if title}
    <Text {...title} />
  {/if}

  {#if description}
    <Text {...description} />
  {/if}

  {#if link}
    <Text {...link} />
  {/if}
</div>
