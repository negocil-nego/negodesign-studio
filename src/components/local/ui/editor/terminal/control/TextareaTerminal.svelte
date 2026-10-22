<script lang="ts">
  import Textarea from "../../../../../../project/ui/atoms/textarea/ui/Textarea.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/textarea/ui/Textarea.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/textarea/ui/Textarea.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/textarea/ui/Textarea.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/textarea/ui/Textarea.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="textarea">
  {#snippet preview()}
    <Textarea
      id="docs-textarea"
      placeholder="Escreva aqui o seu texto…"
      rows={4}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/atoms/textarea/ui/Textarea.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/atoms/textarea/ui/Textarea.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/atoms/textarea/ui/Textarea.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/atoms/textarea/ui/Textarea.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
