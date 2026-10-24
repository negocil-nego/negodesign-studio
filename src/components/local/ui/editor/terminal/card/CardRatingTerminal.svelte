<script lang="ts">
  import CardRating from "../../../../../../project/ui/molecules/card/shared/card-rating/ui/CardRating.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-rating/ui/CardRating.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-rating/ui/CardRating.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-rating/ui/CardRating.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-rating/ui/CardRating.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-rating">
  {#snippet preview()}
    <div class="flex flex-col gap-3">
      <CardRating rating={4} />
      <CardRating rating={2} />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-rating/ui/CardRating.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-rating/ui/CardRating.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-rating/ui/CardRating.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-rating/ui/CardRating.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
