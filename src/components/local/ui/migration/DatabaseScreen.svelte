<script lang="ts">
  import { LocalStyleComponent } from "../../data/style.component";
  import type { DatabaseTable, DatabaseColumn } from "../../data/migration";
  import DataTable from "../../../../project/ui/organisms/data-table/ui/DataTable.svelte";
  import Form from "../../../../project/ui/molecules/form/ui/form.svelte";
  import ModalForm from "../../../../project/ui/organisms/modal/form/ui/ModalForm.svelte";
  import type {
    FormIt,
    FormProps,
  } from "../../../../project/ui/molecules/form/data/types";

  let {
    table,
    selectedComponentId,
    mappings = {},
    onMappingChange,
  }: {
    table: DatabaseTable;
    selectedComponentId: string;
    mappings: Record<string, string>;
    onMappingChange: (key: string, value: string) => void;
  } = $props();

  let isModalOpen = $state(false);

  const isForm = $derived(
    selectedComponentId === "form" || selectedComponentId === "modal-form",
  );
  const isTable = $derived(selectedComponentId === "data-table");

  function getTableProps() {
    return {
      title: table.name,
      columns: table.columns
        .filter((col: DatabaseColumn) => mappings[col.name] !== "ignore")
        .map((col: DatabaseColumn) => ({ key: col.name, label: col.name })),
      rows: [
        { id: "1", cells: table.columns.map(() => "Exemplo") },
        { id: "2", cells: table.columns.map(() => "Dados") },
      ],
    };
  }

  function getFormProps(): FormProps {
    if (!table || !table.columns) return { fields: [] };

    const fields = table.columns
      .filter((col: DatabaseColumn) => {
        const mapping = mappings[col.name];
        return mapping && mapping !== "ignore" && mapping !== "column";
      })
      .map((col: DatabaseColumn) => {
        const component = mappings[col.name];
        const inputTypeByComponent: Record<string, string> = {
          "input-text": "text",
          "input-email": "email",
          "input-password": "password",
          "input-number": "number",
          "input-date": "date",
          "input-phone": "tel",
          "input-age": "number",
        };
        const common = {
          id: col.name,
          name: col.name,
          component: component as any,
          label: col.name,
          required: !col.nullable,
          placeholder: `Introduza ${col.name}...`,
        };

        if (component?.startsWith("input-")) {
          return {
            ...common,
            labelProps: { label: col.name, for: col.name },
            inputProps: {
              id: col.name,
              name: col.name,
              type: (inputTypeByComponent[component] ?? "text") as NonNullable<
                FormIt["inputProps"]
              >["type"],
              required: !col.nullable,
              placeholder: `Introduza ${col.name}...`,
            },
          };
        }

        return {
          ...common,
          ...(component === "textarea"
            ? { labelProps: { label: col.name, for: col.name } }
            : {}),
          ...(component === "radio"
            ? {
                options: [
                  { label: "Sim", value: "true" },
                  { label: "Não", value: "false" },
                ],
              }
            : {}),
        };
      });

    return { style: "grid" as const, fields };
  }
</script>

<div class="flex h-full min-h-0">
  <aside
    class="w-80 border-r border-blender-border p-6 overflow-y-auto space-y-6"
  >
    <div class="space-y-4">
      <span class={LocalStyleComponent.migration.label}
        >Mapeamento de Colunas</span
      >
      <p class="text-[10px] text-zinc-500 leading-relaxed">
        Escolha como cada coluna da tabela <strong>{table.name}</strong> deve ser
        representada no componente.
      </p>

      {#each table.columns as col}
        <label class="block">
          <span class="text-[10px] text-zinc-500 uppercase font-medium"
            >{col.name} ({col.dataType})</span
          >
          <select
            class={LocalStyleComponent.migration.field}
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
              <option value="input-password">Palavra-passe</option>
              <option value="input-phone">Telefone</option>
              <option value="input-age">Idade</option>
              <option value="textarea">Área de Texto</option>
              <option value="toggle">Interruptor</option>
              <option value="checkbox">Caixa de Verificação</option>
              <option value="radio">Botão de Rádio</option>
            {/if}
          </select>
        </label>
      {/each}
    </div>
  </aside>

  <main
    class="flex-1 bg-blender-base/30 p-8 flex flex-col items-center min-h-0 overflow-y-auto"
  >
    <div class="w-full max-w-4xl space-y-6">
      <div class="text-center space-y-1">
        <p
          class="text-[10px] uppercase tracking-widest text-zinc-500 font-semibold"
        >
          Pré-visualização Interactiva
        </p>
        <p class="text-xs text-zinc-400">
          O componente reflecte as alterações de mapeamento em tempo real.
        </p>
      </div>

      <div
        class="flex justify-center p-8 bg-blender-card rounded-xl border border-blender-border shadow-2xl overflow-x-auto min-h-[400px]"
      >
        {#if selectedComponentId === "data-table"}
          <DataTable {...getTableProps()} />
        {:else if selectedComponentId === "form"}
          <div class="w-full max-w-2xl">
            <Form {...getFormProps()} />
          </div>
        {:else if selectedComponentId === "modal-form"}
          <div class="flex w-full flex-col items-center gap-4">
            <div
              class="w-full max-w-2xl rounded-xl border border-blender-border bg-blender-base p-6 shadow-sm"
            >
              <div class="mb-5 border-b border-blender-border pb-3">
                <h2 class="text-base font-semibold text-zinc-100">
                  Formulário: {table.name}
                </h2>
                <p class="mt-1 text-xs text-zinc-400">
                  Pré-visualização do formulário que será apresentado no modal.
                </p>
              </div>
              <Form {...getFormProps()} />
              <div class="mt-5 flex justify-end">
                <button
                  class={LocalStyleComponent.migration.primaryButton}
                  onclick={() => (isModalOpen = true)}
                >
                  Abrir no modal
                </button>
              </div>
            </div>
            <button
              class={LocalStyleComponent.migration.ghostButton}
              onclick={() => (isModalOpen = true)}
            >
              Abrir modal interactivo
            </button>
            {#if isModalOpen}
              <ModalForm
                open={isModalOpen}
                onClose={() => (isModalOpen = false)}
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
