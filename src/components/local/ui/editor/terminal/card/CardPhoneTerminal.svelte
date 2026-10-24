<script lang="ts">
  import CardPhone from "../../../../../../project/ui/molecules/card/shared/card-phone/ui/CardPhone.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-phone/ui/CardPhone.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-phone/ui/CardPhone.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-phone/ui/CardPhone.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-phone/ui/CardPhone.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-phone">
  {#snippet preview()}
    <CardPhone phone="+244 900 000 000" />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-phone/ui/CardPhone.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-phone/ui/CardPhone.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-phone/ui/CardPhone.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-phone/ui/CardPhone.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
