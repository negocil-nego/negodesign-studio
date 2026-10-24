<script lang="ts">
    import IconButton from "../../../../../atoms/button/shared/icon/ui/IconButton.svelte";
  import type { CardFavoriteButtonProps } from "../data/types";

  let { ...restProps }: CardFavoriteButtonProps = $props();
</script>

<IconButton icon={{ name: "Favorite" }} {...restProps} />
