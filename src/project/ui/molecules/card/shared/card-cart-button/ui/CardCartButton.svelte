<script lang="ts">
  import IconButton from "../../../../../atoms/button/ui/IconButton.svelte";
  import type { CardCartButtonProps } from "../data/types";

  let { onClick }: CardCartButtonProps = $props();
</script>

<IconButton icon={{ name: "Cart" }} {onClick} />
