<script lang="ts">
  import CardTags from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  import type { CardTag } from "../../../../../../project/ui/molecules/card/shared/card-tags/data/types";

  const tags: CardTag[] = [
    { icon: "Star", title: "Novo", description: "Acabado de chegar" },
    { icon: "Location", title: "Luanda" },
  ];
</script>

<Terminal id="card-tags">
  {#snippet preview()}
    <CardTags {tags} />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-tags/ui/CardTags.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-tags/ui/CardTags.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-tags/ui/CardTags.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-tags/ui/CardTags.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
