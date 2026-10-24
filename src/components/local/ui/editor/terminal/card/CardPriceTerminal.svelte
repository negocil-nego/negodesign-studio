<script lang="ts">
  import CardPrice from "../../../../../../project/ui/molecules/card/shared/card-price/ui/CardPrice.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-price/ui/CardPrice.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-price/ui/CardPrice.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-price/ui/CardPrice.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-price/ui/CardPrice.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-price">
  {#snippet preview()}
    <CardPrice number={129.9} />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-price/ui/CardPrice.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-price/ui/CardPrice.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-price/ui/CardPrice.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-price/ui/CardPrice.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
