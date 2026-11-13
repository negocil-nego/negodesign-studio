<script lang="ts">
  import type { FormIt, FormProps } from "../../../../../project/ui/molecules/form/data/types";
  import Form from "../../../../../project/ui/molecules/form/ui/form.svelte";
  import ModalForm from "../../../../../project/ui/organisms/modal/form/ui/ModalForm.svelte";
  import type { OpenApiEntity } from "../../../data/migration";

  let {
    entity,
    target,
    operationIndex = $bindable(0),
    onsubmit,
    submitting,
    submissionMessage,
  }: {
    entity: OpenApiEntity;
    target: "form" | "modal-form";
    operationIndex?: number;
    onsubmit: (values?: Record<string, unknown>) => void;
    submitting: boolean;
    submissionMessage: string;
  } = $props();

  let isModalOpen = $state(false);

  const operations = $derived(entity.operations ?? []);
  const activeOperation = $derived(operations[operationIndex]);

  function fieldComponent(field: OpenApiEntity["fields"][number]): string {
    const type = field.type.toLowerCase();
    if (field.name.toLowerCase().includes("email") || type.includes("email")) return "input-email";
    if (field.name.toLowerCase().includes("password")) return "input-password";
    if (type.includes("date") || type.includes("time")) return "input-date";
    if (type.includes("int") || type.includes("number") || type.includes("decimal") || type.includes("float")) return "input-number";
    if (type === "boolean") return "checkbox";
    if (type.includes("string")) return "input-text";
    return "textarea";
  }

  const formProps = $derived.by((): FormProps => ({
    style: "grid",
    submitLabel: activeOperation ? `${activeOperation.method} · ${activeOperation.summary ?? activeOperation.path}` : "Submeter",
    fields: entity.fields.map((field) => {
      const component = fieldComponent(field);
      const inputType = component === "input-email" ? "email" : component === "input-password" ? "password" : component === "input-number" ? "number" : component === "input-date" ? "date" : "text";
      const common: FormIt = {
        id: field.name,
        name: field.name,
        component: component as FormIt["component"],
        label: field.name,
        required: field.required,
        placeholder: field.description ?? `Introduza ${field.name}`,
      };
      if (component.startsWith("input-")) {
        return {
          ...common,
          labelProps: { label: field.name, for: field.name },
          inputProps: { id: field.name, name: field.name, type: inputType as NonNullable<FormIt["inputProps"]>["type"], required: field.required, placeholder: field.description ?? `Introduza ${field.name}` },
        };
      }
      return { ...common, ...(component === "radio" ? { options: [] } : {}) };
    }),
    onSubmit: onsubmit,
  }));
</script>

{#if operations.length}
  <label class="mb-4 block max-w-xl">
    <span class="mb-1 block text-[10px] font-semibold uppercase text-zinc-500">Operação de submissão</span>
    <select
      class="w-full rounded-md border border-blender-border bg-blender-base px-3 py-2 text-xs text-zinc-200"
      bind:value={operationIndex}
    >
      {#each operations as operation, index (`${operation.method}-${operation.path}`)}
        <option value={index}>{operation.method} {operation.path}{operation.summary ? ` · ${operation.summary}` : ""}</option>
      {/each}
    </select>
  </label>
{:else}
  <p class="mb-4 rounded-lg border border-amber-500/30 bg-amber-500/5 p-3 text-xs text-amber-200">Sem operação de escrita associada a este schema. O formulário é pré-visualizado, mas não será enviado sem uma rota declarada no OpenAPI.</p>
{/if}

<div class="mx-auto max-w-3xl rounded-xl border border-blender-border bg-blender-base p-5">
  {#if target === "form"}
    <Form {...formProps} />
  {:else}
    <div class="space-y-4">
      <h3 class="font-semibold text-zinc-100">Pré-visualização do Modal Form</h3>
      <p class="text-xs text-zinc-400">O formulário será submetido para a operação seleccionada.</p>
      <button type="button" class="rounded-md bg-blue-600 px-4 py-2 text-xs font-semibold text-white" onclick={() => (isModalOpen = true)}>Abrir formulário</button>
    </div>
    {#if isModalOpen}
      <ModalForm open={isModalOpen} title={entity.name} form={formProps} onClose={() => (isModalOpen = false)} />
    {/if}
  {/if}
  {#if submitting}<p class="mt-3 text-xs text-blue-300" role="status">A submeter…</p>{/if}
  {#if submissionMessage}<p class="mt-3 break-words text-xs text-zinc-300" role="status">{submissionMessage}</p>{/if}
</div>
