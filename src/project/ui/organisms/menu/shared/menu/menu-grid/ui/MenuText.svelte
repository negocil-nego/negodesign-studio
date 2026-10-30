<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import Text from "../../../../../atoms/text/ui/Text.svelte";
  import type { MenuTextProps } from "../data/types";

  let { icon, textProps }: MenuTextProps = $props();
</script>

{#if icon}
  <Icon {...icon} />
{/if}
{#if textProps}
  <Text {...textProps} />
{/if}
