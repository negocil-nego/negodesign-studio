<script lang="ts">
  import type { TextProps } from "../data/types";

  let { text }: TextProps = $props();
</script>

<div data-name="text">{text}</div>
