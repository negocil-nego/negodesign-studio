<script lang="ts">
  import CardCartButton from "../../../../../../project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-cart-button">
  {#snippet preview()}
    <CardCartButton />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
