<script lang="ts">
    import Button from "../../../../../atoms/button/variant/default/ui/Button.svelte";
  import type { CardButtonProps } from "../data/types";

  let { ...restProps }: CardButtonProps = $props();
</script>

<Button {...restProps} />
