<script lang="ts">
  import CardFavoriteButton from "../../../../../../project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-favorite-button">
  {#snippet preview()}
    <CardFavoriteButton />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
