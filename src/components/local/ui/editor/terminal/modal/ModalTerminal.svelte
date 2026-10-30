<script lang="ts">
  import ModalUsage from "./ModalUsage.svelte";
  import svelteSource from "./ModalUsage.svelte?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="modal-core" name="Modal" description="Diálogo acessível para apresentar conteúdo sobre a página actual.">
  {#snippet preview()}<ModalUsage />{/snippet}
  {#snippet svelte()}
    <SourceCode path="src/project/ui/organisms/modal/core/ui/Modal.svelte (exemplo de utilização)" source={svelteSource} />
  {/snippet}
</Terminal>
