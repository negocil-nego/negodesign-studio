<script lang="ts">
  import CardImage from "../../../../../../project/ui/molecules/card/shared/card-image/ui/CardImage.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-image/ui/CardImage.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-image/ui/CardImage.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-image/ui/CardImage.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-image/ui/CardImage.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-image">
  {#snippet preview()}
    <div class="w-40 overflow-hidden rounded-lg">
      <CardImage src="/img/ux-01.jpg" alt="Imagem de cartão" />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-image/ui/CardImage.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-image/ui/CardImage.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-image/ui/CardImage.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-image/ui/CardImage.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
