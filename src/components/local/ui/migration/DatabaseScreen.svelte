<script lang="ts">
  import { StyleComponent } from "../../data/style.component";
  import type { DatabaseTable, DatabaseColumn } from "../../data/migration";
  import DataTable from "../../../../project/ui/organisms/data-table/ui/DataTable.svelte";
  import Form from "../../../../project/ui/molecules/form/ui/form.svelte";
  import ModalForm from "../../../../project/ui/organisms/modal/form/ui/ModalForm.svelte";

  let { 
    table, 
    selectedComponentId,
    mappings = {},
    onMappingChange
  }: { 
    table: DatabaseTable; 
    selectedComponentId: string;
    mappings: Record<string, string>;
    onMappingChange: (key: string, value: string) => void;
  } = $props();

  let isModalOpen = $state(false);

  const isForm = $derived(selectedComponentId === "form" || selectedComponentId === "modal-form");
  const isTable = $derived(selectedComponentId === "data-table");

  function getTableProps() {
    return {
      title: table.name,
      columns: table.columns
        .filter((col: DatabaseColumn) => mappings[col.name] !== "ignore")
        .map((col: DatabaseColumn) => ({ key: col.name, label: col.name })),
      rows: [
        { id: "1", cells: table.columns.map(() => "Exemplo") },
        { id: "2", cells: table.columns.map(() => "Dados") }
      ]
    };
  }

  function getFormProps() {
    return {
      fields: table.columns
        .filter((col: DatabaseColumn) => mappings[col.name] && mappings[col.name] !== "ignore")
        .map((col: DatabaseColumn) => ({
          id: col.name,
          component: mappings[col.name] as any,
          label: col.name,
          placeholder: `Introduza ${col.name}...`
        }))
    };
  }
</script>

<div class="flex h-full min-h-0">
  <aside class="w-80 border-r border-blender-border p-6 overflow-y-auto space-y-6">
    <div class="space-y-4">
      <span class={StyleComponent.migration.label}>Mapeamento de Colunas</span>
      <p class="text-[10px] text-zinc-500 leading-relaxed">
        Escolha como cada coluna da tabela <strong>{table.name}</strong> deve ser representada no componente.
      </p>
      
      {#each table.columns as col}
        <label class="block">
          <span class="text-[10px] text-zinc-500 uppercase font-medium">{col.name} ({col.dataType})</span>
          <select 
            class={StyleComponent.migration.field} 
            value={mappings[col.name] || ""}
            onchange={(e) => onMappingChange(col.name, e.currentTarget.value)}
          >
            <option value="ignore">Ignorar</option>
            {#if isTable}
              <option value="column">Coluna da Tabela</option>
            {:else if isForm}
              <option value="input-text">Texto</option>
              <option value="input-number">Número</option>
              <option value="input-date">Data</option>
              <option value="input-email">Email</option>
              <option value="textarea">Área de Texto</option>
              <option value="toggle">Interruptor</option>
            {/if}
          </select>
        </label>
      {/each}
    </div>
  </aside>

  <main class="flex-1 bg-blender-base/30 p-8 flex flex-col items-center min-h-0 overflow-y-auto">
    <div class="w-full max-w-4xl space-y-6">
      <div class="text-center space-y-1">
        <p class="text-[10px] uppercase tracking-widest text-zinc-500 font-semibold">Pré-visualização Interactiva</p>
        <p class="text-xs text-zinc-400">O componente reflecte as alterações de mapeamento em tempo real.</p>
      </div>

      <div class="flex justify-center p-8 bg-blender-card rounded-xl border border-blender-border shadow-2xl overflow-x-auto min-h-[300px]">
        {#if selectedComponentId === 'data-table'}
          <DataTable {...getTableProps()} />
        {:else if selectedComponentId === 'form'}
          <Form {...getFormProps()} />
        {:else if selectedComponentId === 'modal-form'}
          <div class="text-center space-y-4">
            <p class="text-xs text-zinc-500 italic">O formulário será apresentado dentro de um diálogo.</p>
            <button 
              class={StyleComponent.migration.primaryButton}
              onclick={() => isModalOpen = true}
            >
              Abrir Modal Exemplo
            </button>
            <div class="opacity-50 pointer-events-none scale-90 origin-top">
              <Form {...getFormProps()} />
            </div>

            {#if isModalOpen}
              <ModalForm 
                open={isModalOpen} 
                onClose={() => isModalOpen = false} 
                title={`Formulário: ${table.name}`}
                form={getFormProps()}
              />
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </main>
</div>
