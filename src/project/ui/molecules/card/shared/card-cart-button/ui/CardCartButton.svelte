<script lang="ts">
  import IconButton from "../../../../../atoms/button/shared/icon/ui/IconButton.svelte";
  import type { CardCartButtonProps } from "../data/types";

  let { ...restProps }: CardCartButtonProps = $props();
</script>

<IconButton icon={{ name: "Cart" }} {...restProps} />
