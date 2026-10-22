<script lang="ts">
  import { getIconClass, IconNamed } from "../../../../../hooks/icon.svelte";
  import type { IconProps } from "../data/types";

  let { name }: IconProps = $props();

  const iconClass = $derived(getIconClass(IconNamed[name]));
</script>

<i class={`${iconClass} text-base opacity-75`} aria-hidden="true"></i>
