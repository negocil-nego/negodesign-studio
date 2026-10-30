<script lang="ts">
  import ModalFormUsage from "./ModalFormUsage.svelte";
  import svelteSource from "./ModalFormUsage.svelte?raw";
  import reactSource from "./ModalFormUsage.tsx?raw";
  import vueSource from "./ModalFormUsage.vue?raw";
  import flutterSource from "./ModalFormUsage.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="modal-form" name="ModalForm" description="Modal que apresenta um formulário configurado através das propriedades do componente Form.">
  {#snippet preview()}<ModalFormUsage />{/snippet}
  {#snippet svelte()}
    <SourceCode path="src/components/local/ui/editor/terminal/modal/ModalFormUsage.svelte" source={svelteSource} />
  {/snippet}
  {#snippet react()}<SourceCode path="src/components/local/ui/editor/terminal/modal/ModalFormUsage.tsx" source={reactSource} />{/snippet}
  {#snippet vue()}<SourceCode path="src/components/local/ui/editor/terminal/modal/ModalFormUsage.vue" source={vueSource} />{/snippet}
  {#snippet flutter()}<SourceCode path="src/components/local/ui/editor/terminal/modal/ModalFormUsage.dart" source={flutterSource} />{/snippet}
</Terminal>
