<script lang="ts">
  import Radio from "../../../../../../project/ui/atoms/radio/ui/Radio.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/radio/ui/Radio.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/radio/ui/Radio.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/radio/ui/Radio.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/radio/ui/Radio.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  const demoOptions = [
    { value: "svelte", label: "Svelte" },
    { value: "react", label: "React" },
    { value: "vue", label: "Vue" },
  ];
</script>

<Terminal id="radio">
  {#snippet preview()}
    <Radio name="docs-radio" options={demoOptions} value="svelte" />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/atoms/radio/ui/Radio.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/atoms/radio/ui/Radio.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/atoms/radio/ui/Radio.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/atoms/radio/ui/Radio.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
