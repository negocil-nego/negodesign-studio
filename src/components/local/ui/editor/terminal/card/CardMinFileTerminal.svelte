<script lang="ts">
  import CardMinFile from "../../../../../../project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  import type { CardMinFileProps } from "../../../../../../project/ui/molecules/card/shared/card-min-file/data/types";

  const minFiles: CardMinFileProps[] = [
    { imageProps: { src: "/img/ux-01.jpg", alt: "Miniatura 1" }, active: true },
    { imageProps: { src: "/img/programmer-01.jpg", alt: "Miniatura 2" } },
    { imageProps: { src: "/img/programmer-02.jpg", alt: "Miniatura 3" } },
  ];
</script>

<Terminal id="card-min-file">
  {#snippet preview()}
    <div class="flex gap-2">
      {#each minFiles as minFile}
        <CardMinFile {...minFile} />
      {/each}
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
