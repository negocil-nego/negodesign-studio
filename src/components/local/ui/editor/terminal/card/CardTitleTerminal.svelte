<script lang="ts">
  import CardTitle from "../../../../../../project/ui/molecules/card/shared/card-title/ui/CardTitle.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-title/ui/CardTitle.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-title/ui/CardTitle.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-title/ui/CardTitle.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-title/ui/CardTitle.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-title">
  {#snippet preview()}
    <CardTitle title="Título do cartão" />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-title/ui/CardTitle.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-title/ui/CardTitle.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-title/ui/CardTitle.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-title/ui/CardTitle.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
