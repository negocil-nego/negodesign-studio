<script lang="ts">
  import CardDescription from "../../../../../../project/ui/molecules/card/shared/card-description/ui/CardDescription.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-description/ui/CardDescription.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-description/ui/CardDescription.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-description/ui/CardDescription.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-description/ui/CardDescription.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-description">
  {#snippet preview()}
    <div class="max-w-xs">
      <CardDescription description="Descrição breve do conteúdo do cartão." />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-description/ui/CardDescription.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-description/ui/CardDescription.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-description/ui/CardDescription.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-description/ui/CardDescription.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
