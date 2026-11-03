<script lang="ts">
  import CardTags from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  import type { CardTag } from "../../../../../../project/ui/molecules/card/shared/card-tags/data/types";

  const buttonTags: CardTag[] = [
    { icon: "User", title: "Follow" },
    { icon: "Message", title: "Message" },
    { icon: "Email", title: "Email" },
  ];
  const statTags: CardTag[] = [
    { icon: "User", title: "154" },
    { icon: "CheckmarkSquare01", title: "34" },
  ];
</script>

<Terminal id="card-tags">
  {#snippet preview()}
    <div class="space-y-4">
      <div>
        <p class="mb-2 text-[10px] font-semibold uppercase tracking-wider text-zinc-500">Botões</p>
        <CardTags tags={buttonTags} variant="buttons" />
      </div>
      <div>
        <p class="mb-2 text-[10px] font-semibold uppercase tracking-wider text-zinc-500">Estatísticas</p>
        <CardTags tags={statTags} variant="stats" />
      </div>
    </div>
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
