<script lang="ts">
  import ModalFormUsage from "./ModalFormUsage.svelte";
  import svelteSource from "./ModalFormUsage.svelte?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="modal-form" name="ModalForm" description="Modal que apresenta um formulário configurado através das propriedades do componente Form.">
  {#snippet preview()}<ModalFormUsage />{/snippet}
  {#snippet svelte()}
    <SourceCode path="src/project/ui/organisms/modal/form/ui/ModalForm.svelte (exemplo de utilização)" source={svelteSource} />
  {/snippet}
</Terminal>
