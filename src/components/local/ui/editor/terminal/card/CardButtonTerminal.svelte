<script lang="ts">
  import CardButton from "../../../../../../project/ui/molecules/card/shared/card-button/ui/CardButton.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-button/ui/CardButton.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-button/ui/CardButton.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-button/ui/CardButton.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-button/ui/CardButton.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-button">
  {#snippet preview()}
    <div class="flex flex-col gap-3">
      <CardButton text="Acção" />
      <CardButton variant="outline" text="Acção" />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-button/ui/CardButton.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-button/ui/CardButton.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-button/ui/CardButton.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-button/ui/CardButton.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
