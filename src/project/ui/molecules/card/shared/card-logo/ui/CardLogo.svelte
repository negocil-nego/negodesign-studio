<script lang="ts">
    import Avatar from "../../../../../atoms/avatar/ui/Avatar.svelte";
  import type { CardLogoProps } from "../data/types";

  let { ...restProps }: CardLogoProps = $props();
</script>

<Avatar {...restProps} />
