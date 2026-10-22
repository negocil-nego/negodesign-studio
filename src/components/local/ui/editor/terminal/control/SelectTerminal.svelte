<script lang="ts">
  import Select from "../../../../../../project/ui/atoms/select/ui/Select.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/select/ui/Select.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/select/ui/Select.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/select/ui/Select.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/select/ui/Select.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  const demoOptions = [
    { value: "ao", label: "Angola" },
    { value: "pt", label: "Portugal" },
    { value: "br", label: "Brasil" },
  ];
</script>

<Terminal id="select">
  {#snippet preview()}
    <Select
      id="docs-select"
      options={demoOptions}
      placeholder="Seleccione um país…"
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/atoms/select/ui/Select.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/atoms/select/ui/Select.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/atoms/select/ui/Select.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/atoms/select/ui/Select.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
