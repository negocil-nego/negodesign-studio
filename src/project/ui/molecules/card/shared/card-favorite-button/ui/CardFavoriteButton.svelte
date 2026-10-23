<script lang="ts">
    import IconButton from "../../../../../atoms/button/shared/icon/ui/IconButton.svelte";
  import type { CardFavoriteButtonProps } from "../data/types";

  let { onClick }: CardFavoriteButtonProps = $props();
</script>

<IconButton icon={{ name: "Favorite" }} {onClick} />
