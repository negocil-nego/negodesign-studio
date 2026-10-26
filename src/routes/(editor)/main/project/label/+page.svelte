<script lang="ts">
  import EditorPageShell from "../../../../../components/local/ui/editor/EditorPageShell.svelte";
  import AtomicComponentScreen from "../../../../../components/local/ui/editor/screen/AtomicComponentScreen.svelte";
  import { componentCatalog } from "../../../../../components/local/data/components";
  import { activeProject } from "../../../../../components/local/data/projects";
  import { copyComponentToActiveProject } from "../../../../../components/local/data/component-copy";

  const labelComponents = componentCatalog.filter(c => c.id === "label");
  
  let copyingId = $state<string | null>(null);

  async function handleCopy(component: any) {
    if (!component.dependence) return;
    copyingId = component.id;
    try {
      await copyComponentToActiveProject(component.id);
    } catch (error) {
      console.error(error);
    } finally {
      copyingId = null;
    }
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
    {copyingId} 
    onCopy={handleCopy} 
  />
</EditorPageShell>
