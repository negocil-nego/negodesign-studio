<script lang="ts">
  import { StyleComponent } from "../../data/style.component";
  import { type DatabaseTable } from "../../data/migration";
  import { componentCatalog, type ComponentDefinition } from "../../data/components";
  import DatabaseScreen from "./DatabaseScreen.svelte";
  import Icon from "../../../../project/ui/atoms/icon/ui/Icon.svelte";

  let { table, onclose }: { table: DatabaseTable; onclose: () => void } = $props();
  let step = $state<"selection" | "mapping">("selection");
  let selectedComponent = $state<ComponentDefinition | null>(null);
  let mappings = $state<Record<string, string>>({});

  const targetComponents = $derived([
    componentCatalog.find(c => c.id === "form"),
    componentCatalog.find(c => c.id === "modal-form"),
    componentCatalog.find(c => c.id === "data-table"),
  ].filter((c): c is ComponentDefinition => !!c));

  function inferMappings(compId: string) {
    const newMappings: Record<string, string> = {};
    const isTable = compId === "data-table";
    
    if (!table || !table.columns) return newMappings;

    table.columns.forEach(col => {
      const name = (col.name || "").toLowerCase();
      const type = (col.dataType || "").toLowerCase();
      
      if (isTable) {
        newMappings[col.name] = "column";
        return;
      }

      // Ignorar campos de auditoria comuns em formulários
      if (["id", "created_at", "updated_at", "deleted_at", "created_by", "updated_by"].includes(name)) {
        newMappings[col.name] = "ignore";
        return;
      }

      if (name.includes("email")) newMappings[col.name] = "input-email";
      else if (name.includes("pass")) newMappings[col.name] = "input-password";
      else if (name.includes("desc") || name.includes("bio") || name.includes("comment")) newMappings[col.name] = "textarea";
      else if (type.includes("bool")) newMappings[col.name] = "toggle";
      else if (type.includes("date") || type.includes("time")) newMappings[col.name] = "input-date";
      else if (type.includes("int") || type.includes("num") || type.includes("decimal") || type.includes("float") || type.includes("double")) newMappings[col.name] = "input-number";
      else newMappings[col.name] = "input-text";
    });
    
    return newMappings;
  }

  function selectComponent(comp: ComponentDefinition) {
    if (!comp) return;
    try {
      // Garantir que criamos um novo objecto de mappings para disparar reactividade
      const newMappings = inferMappings(comp.id);
      
      // Actualizar estados
      selectedComponent = comp;
      mappings = newMappings;
      step = "mapping";
    } catch (e) {
      console.error("Error transitioning to mapping step:", e);
    }
  }

  function handleMappingChange(key: string, value: string) {
    if (!mappings) mappings = {};
    mappings[key] = value;
  }
</script>

<div class={StyleComponent.migration.overlay} role="presentation" onclick={(e) => e.target === e.currentTarget && onclose()}>
  <div class="flex h-[85vh] w-full max-w-6xl flex-col overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-2xl" role="dialog">
    <header class="flex items-center justify-between border-b border-blender-border px-6 py-4">
      <div>
        <h2 class="text-lg font-semibold text-zinc-100">
          {step === 'selection' ? `Seleccionar Organismo para ${table.name}` : `Mapear: ${selectedComponent?.name} para ${table.name}`}
        </h2>
        <p class="text-xs text-zinc-400">
          {step === 'selection' ? 'Escolha o tipo de interface que deseja construir.' : 'Associe as colunas da tabela às propriedades do componente.'}
        </p>
      </div>
      <button type="button" class="text-zinc-400 hover:text-zinc-100" onclick={onclose}>✕</button>
    </header>

    {#if step === 'selection'}
      <div class="flex-1 flex items-center justify-center p-6">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 w-full max-w-4xl">
          {#each targetComponents as item}
            <button
              class="flex flex-col items-center text-center gap-4 rounded-xl border border-blender-border p-8 transition hover:border-blue-500 hover:bg-blue-500/5 group"
              onclick={() => selectComponent(item)}
            >
              <div class="p-4 rounded-full bg-zinc-800 text-zinc-400 group-hover:bg-blue-500/20 group-hover:text-blue-400 transition">
                <Icon name={item.icon} className="h-8 w-8" />
              </div>
              <div>
                <span class="block text-base font-semibold text-zinc-200 group-hover:text-blue-300">{item.label || item.name}</span>
                <span class="mt-2 block text-xs text-zinc-500 leading-relaxed">{item.description}</span>
              </div>
            </button>
          {/each}
        </div>
      </div>
    {:else if selectedComponent}
      <div class="flex-1 min-h-0 flex flex-col">
        <div class="bg-blender-base/20 px-6 py-2 border-b border-blender-border flex items-center justify-between">
          <button class="text-[10px] font-bold uppercase text-blue-500 hover:text-blue-400" onclick={() => step = 'selection'}>
            ← Alterar Organismo
          </button>
          <span class="text-[10px] text-zinc-500 uppercase font-mono">{selectedComponent.name}</span>
        </div>
        <div class="flex-1 min-h-0">
          <DatabaseScreen 
            {table} 
            selectedComponentId={selectedComponent.id} 
            {mappings}
            onMappingChange={handleMappingChange}
          />
        </div>
      </div>
    {/if}

    <footer class="flex justify-end gap-3 border-t border-blender-border px-6 py-4">
      <button class={StyleComponent.migration.ghostButton} onclick={onclose}>Cancelar</button>
      <button class={StyleComponent.migration.primaryButton} disabled={step === 'selection'}>Gerar Componente</button>
    </footer>
  </div>
</div>
