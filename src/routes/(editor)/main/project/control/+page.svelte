<script lang="ts">
  import EditorPageShell from "../../../../../components/local/ui/editor/EditorPageShell.svelte";
  import AtomicComponentScreen from "../../../../../components/local/ui/editor/screen/AtomicComponentScreen.svelte";
  import ComponentCopyDialog from "../../../../../components/local/ui/editor/shared/ComponentCopyDialog.svelte";
  import { componentCatalog, type ComponentDefinition } from "../../../../../components/local/data/components";

  const controlComponents = componentCatalog.filter((c) => c.group === "Atom · Control");
  let componentToCopy = $state<ComponentDefinition | null>(null);
  let statusMessage = $state("");

  function handleCopy(component: ComponentDefinition) {
    if (!component.dependence) return;
    componentToCopy = component;
  }
</script>

<EditorPageShell>
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">
      Átomos / Control
    </p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Controlos de selecção e Bootswatch</h2>
    <p class="mt-2 mb-6 text-xs leading-5 text-zinc-400">
      Pré-visualização interactiva e código-fonte dos componentes de controlo, navegação e contentores Bootswatch.
    </p>
    {#if statusMessage}
      <p class="mb-4 text-xs text-blue-400">{statusMessage}</p>
    {/if}
  </div>

  <AtomicComponentScreen
    components={controlComponents}
    viewMode="terminal"
    copyingId={null}
    onCopy={handleCopy}
  />
</EditorPageShell>

{#if componentToCopy}
  <ComponentCopyDialog
    component={componentToCopy}
    onclose={() => (componentToCopy = null)}
    onmessage={(msg) => (statusMessage = msg)}
  />
{/if}
