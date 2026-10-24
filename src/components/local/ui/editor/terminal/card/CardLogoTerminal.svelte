<script lang="ts">
  import CardLogo from "../../../../../../project/ui/molecules/card/shared/card-logo/ui/CardLogo.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-logo/ui/CardLogo.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-logo/ui/CardLogo.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-logo/ui/CardLogo.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-logo/ui/CardLogo.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-logo">
  {#snippet preview()}
    <CardLogo src="/img/programmer-01.jpg" alt="Logótipo" />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-logo/ui/CardLogo.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-logo/ui/CardLogo.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-logo/ui/CardLogo.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-logo/ui/CardLogo.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
