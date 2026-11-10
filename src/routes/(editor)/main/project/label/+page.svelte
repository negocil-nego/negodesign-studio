<script lang="ts">
  import EditorPageShell from "../../../../../components/local/ui/editor/EditorPageShell.svelte";
  import AtomicComponentScreen from "../../../../../components/local/ui/editor/screen/AtomicComponentScreen.svelte";
  import ComponentCopyDialog from "../../../../../components/local/ui/editor/shared/ComponentCopyDialog.svelte";
  import { componentCatalog, type ComponentDefinition } from "../../../../../components/local/data/components";

  const labelComponents = componentCatalog.filter((c) => c.id === "label");
  let componentToCopy = $state<ComponentDefinition | null>(null);

  function handleCopy(component: ComponentDefinition) {
    if (!component.dependence) return;
    componentToCopy = component;
  }
</script>

<EditorPageShell>
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">
      Átomos / Label
    </p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Label</h2>
    <p class="mt-2 mb-6 max-w-3xl text-xs leading-5 text-zinc-400">
      Texto associado a um controlo de formulário. A propriedade <code
        class="text-blue-300">for</code
      >
      deve corresponder ao <code class="text-blue-300">id</code> do campo para que
      o utilizador possa focar o controlo ao seleccionar o rótulo.
    </p>
  </div>

  <AtomicComponentScreen
    components={labelComponents}
    viewMode="terminal"
    copyingId={null}
    onCopy={handleCopy}
  />
</EditorPageShell>

{#if componentToCopy}
  <ComponentCopyDialog
    component={componentToCopy}
    onclose={() => (componentToCopy = null)}
    onmessage={() => {}}
  />
{/if}
