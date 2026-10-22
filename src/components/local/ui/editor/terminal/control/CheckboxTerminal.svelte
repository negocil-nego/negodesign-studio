<script lang="ts">
  import Checkbox from "../../../../../../project/ui/atoms/checkbox/ui/Checkbox.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/checkbox/ui/Checkbox.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/checkbox/ui/Checkbox.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/checkbox/ui/Checkbox.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/checkbox/ui/Checkbox.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="checkbox">
  {#snippet preview()}
    <div class="flex flex-col gap-3">
      <Checkbox id="docs-checkbox-unchecked" label="Não seleccionado" />
      <Checkbox id="docs-checkbox-checked" checked={true} label="Seleccionado" />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/atoms/checkbox/ui/Checkbox.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/atoms/checkbox/ui/Checkbox.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/atoms/checkbox/ui/Checkbox.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/atoms/checkbox/ui/Checkbox.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
