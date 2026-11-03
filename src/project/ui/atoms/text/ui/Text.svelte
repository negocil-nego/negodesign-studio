<script lang="ts">
  import type { TextProps } from "../data/types";

  let { text, className }: TextProps = $props();
</script>

<div data-name="text" class={className}>{text}</div>
