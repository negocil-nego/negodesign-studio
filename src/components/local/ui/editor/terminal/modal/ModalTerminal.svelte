<script lang="ts">
  import ModalUsage from "./ModalUsage.svelte";
  import svelteSource from "./ModalUsage.svelte?raw";
  import reactSource from "./ModalUsage.tsx?raw";
  import vueSource from "./ModalUsage.vue?raw";
  import flutterSource from "./ModalUsage.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="modal-core" name="Modal" description="Diálogo acessível para apresentar conteúdo sobre a página actual.">
  {#snippet preview()}<ModalUsage />{/snippet}
  {#snippet svelte()}
    <SourceCode path="src/components/local/ui/editor/terminal/modal/ModalUsage.svelte" source={svelteSource} />
  {/snippet}
  {#snippet react()}<SourceCode path="src/components/local/ui/editor/terminal/modal/ModalUsage.tsx" source={reactSource} />{/snippet}
  {#snippet vue()}<SourceCode path="src/components/local/ui/editor/terminal/modal/ModalUsage.vue" source={vueSource} />{/snippet}
  {#snippet flutter()}<SourceCode path="src/components/local/ui/editor/terminal/modal/ModalUsage.dart" source={flutterSource} />{/snippet}
</Terminal>
