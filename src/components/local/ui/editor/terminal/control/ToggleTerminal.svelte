<script lang="ts">
  import Toggle from "../../../../../../project/ui/atoms/toggle/ui/Toggle.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/toggle/ui/Toggle.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/toggle/ui/Toggle.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/toggle/ui/Toggle.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/toggle/ui/Toggle.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="toggle">
  {#snippet preview()}
    <div class="flex flex-col gap-3">
      <Toggle id="docs-toggle-off" label="Desactivado" />
      <Toggle id="docs-toggle-on" checked={true} label="Activado" />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/atoms/toggle/ui/Toggle.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/atoms/toggle/ui/Toggle.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/atoms/toggle/ui/Toggle.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/atoms/toggle/ui/Toggle.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
