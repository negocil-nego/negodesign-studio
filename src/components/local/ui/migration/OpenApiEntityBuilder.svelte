<script lang="ts">
  import { goto } from "$app/navigation";
  import type { FormIt, FormProps } from "../../../../project/ui/molecules/form/data/types";
  import Form from "../../../../project/ui/molecules/form/ui/form.svelte";
  import ModalForm from "../../../../project/ui/organisms/modal/form/ui/ModalForm.svelte";
  import DataTable from "../../../../project/ui/organisms/data-table/ui/DataTable.svelte";
  import CarouselCardProfile from "../../../../project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.svelte";
  import GridCardProfile from "../../../../project/ui/organisms/section/grid/card/profile/ui/GridCardProfile.svelte";
  import GridCarouselCardProfile from "../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.svelte";
  import type { OpenApiEntity } from "../../data/migration";

  let { entity, onclose }: { entity: OpenApiEntity; onclose: () => void } = $props();
  type Target = "form" | "modal-form" | "data-table" | "carousel" | "grid" | "grid-carousel";
  const creationTargets: { id: Target; label: string; route: string }[] = [
    { id: "form", label: "Formulário", route: "/main/project/form#form" },
    { id: "modal-form", label: "Modal Form", route: "/main/project/modal#modal-form" },
  ];
  const presentationTargets: { id: Target; label: string; route: string }[] = [
    { id: "data-table", label: "DataTable", route: "/main/project#data-table" },
    { id: "carousel", label: "Carousel de cartões", route: "/main/project/carousel#carousel-card-profile" },
    { id: "grid", label: "Grelha de cartões", route: "/main/project/grid#grid-card-profile" },
    { id: "grid-carousel", label: "Grelha com carousel", route: "/main/project/grid-carousel#grid-carousel-card-profile" },
  ];
  const targets = $derived(entity.kind === "dto" || entity.kind === "request" ? creationTargets : presentationTargets);
  let target = $state<Target>("form");
  $effect(() => {
    target = entity.kind === "dto" || entity.kind === "request" ? "form" : "data-table";
  });
  let operationIndex = $state(0);
  let isModalOpen = $state(false);
  let submissionMessage = $state("");
  let submitting = $state(false);
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
    onSubmit: submitRequest,
  }));

  const tableProps = $derived({
    title: entity.name,
    columns: entity.fields.map((field) => ({ key: field.name, label: field.name })),
    rows: [{ id: "openapi-preview", cells: entity.fields.map((field) => field.type) }],
  });
  const cards = $derived(entity.fields.map((field) => ({
    titleProps: { title: field.name },
    descriptionProps: { description: `${field.type}${field.required ? " · obrigatório" : ""}` },
    imageProps: { src: "/img/ux-01.jpg", alt: field.name },
  })));

  async function submitRequest(values: Record<string, unknown> = {}) {
    submissionMessage = "";
    if (!activeOperation) {
      submissionMessage = "Este DTO não está associado a uma operação de escrita na especificação OpenAPI.";
      return;
    }
    const base = entity.serverUrl?.replace(/\/$/, "") ?? "";
    const url = `${base}${activeOperation.path}`;
    submitting = true;
    try {
      const response = await fetch(url, {
        method: activeOperation.method,
        headers: { "Content-Type": "application/json", Accept: "application/json" },
        body: ["GET", "DELETE"].includes(activeOperation.method) ? undefined : JSON.stringify(values),
      });
      const body = await response.text();
      submissionMessage = response.ok
        ? `${activeOperation.method} ${activeOperation.path} — sucesso (${response.status}).${body ? ` ${body.slice(0, 180)}` : ""}`
        : `${activeOperation.method} ${activeOperation.path} — erro ${response.status}: ${body.slice(0, 180)}`;
    } catch (error) {
      submissionMessage = error instanceof Error ? error.message : String(error);
    } finally {
      submitting = false;
    }
  }

  function openTargetRoute() {
    const selected = targets.find((item) => item.id === target);
    if (selected) void goto(selected.route);
  }
</script>

<div class="fixed inset-0 z-[120] flex items-center justify-center overflow-y-auto bg-black/60 p-4 backdrop-blur-sm" role="presentation" onclick={(event) => event.target === event.currentTarget && onclose()}>
  <div class="my-auto flex max-h-[92vh] w-full max-w-5xl flex-col overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-2xl" role="dialog" aria-modal="true" aria-labelledby="entity-builder-title" tabindex="-1">
    <header class="flex items-start justify-between gap-4 border-b border-blender-border p-5">
      <div><p class="text-[10px] font-semibold uppercase tracking-wider text-blue-400">{entity.kind === "dto" || entity.kind === "request" ? "Criação · DTO/Request" : "Apresentação · Response"}</p><h2 id="entity-builder-title" class="mt-1 text-lg font-semibold text-zinc-100">Componentizar {entity.name}</h2><p class="mt-1 text-xs text-zinc-400">{entity.fields.length} campos · {operations.length} operação(ões) associada(s)</p></div>
      <button type="button" class="text-zinc-400 hover:text-zinc-100" onclick={onclose} aria-label="Fechar">×</button>
    </header>
    <div class="min-h-0 flex-1 overflow-y-auto p-5">
      {#if targets.length}
        <div class="mb-5 flex flex-wrap gap-2" aria-label="Escolher componente">
          {#each targets as item (item.id)}
            <button type="button" aria-pressed={target === item.id} class={`rounded-lg border px-3 py-2 text-xs ${target === item.id ? "border-blue-500 bg-blue-500/10 text-blue-300" : "border-blender-border text-zinc-400 hover:bg-blender-hover"}`} onclick={() => target = item.id}>{item.label}</button>
          {/each}
        </div>
      {/if}

      {#if (target === "form" || target === "modal-form") && (entity.kind === "dto" || entity.kind === "request")}
        {#if operations.length}
          <label class="mb-4 block max-w-xl"><span class="mb-1 block text-[10px] font-semibold uppercase text-zinc-500">Operação de submissão</span><select class="w-full rounded-md border border-blender-border bg-blender-base px-3 py-2 text-xs text-zinc-200" bind:value={operationIndex}>{#each operations as operation, index (`${operation.method}-${operation.path}`)}<option value={index}>{operation.method} {operation.path}{operation.summary ? ` · ${operation.summary}` : ""}</option>{/each}</select></label>
        {:else}
          <p class="mb-4 rounded-lg border border-amber-500/30 bg-amber-500/5 p-3 text-xs text-amber-200">Sem operação de escrita associada a este schema. O formulário é pré-visualizado, mas não será enviado sem uma rota declarada no OpenAPI.</p>
        {/if}
        <div class="mx-auto max-w-3xl rounded-xl border border-blender-border bg-blender-base p-5">
          {#if target === "form"}
            <Form {...formProps} />
          {:else}
            <div class="space-y-4"><h3 class="font-semibold text-zinc-100">Pré-visualização do Modal Form</h3><p class="text-xs text-zinc-400">O formulário será submetido para a operação seleccionada.</p><button type="button" class="rounded-md bg-blue-600 px-4 py-2 text-xs font-semibold text-white" onclick={() => isModalOpen = true}>Abrir formulário</button></div>
            {#if isModalOpen}<ModalForm open={isModalOpen} title={entity.name} form={formProps} onClose={() => isModalOpen = false} />{/if}
          {/if}
          {#if submitting}<p class="mt-3 text-xs text-blue-300" role="status">A submeter…</p>{/if}
          {#if submissionMessage}<p class="mt-3 break-words text-xs text-zinc-300" role="status">{submissionMessage}</p>{/if}
        </div>
      {:else if target === "data-table"}
        <div class="rounded-lg bg-blender-base p-4"><DataTable variant="03" {...tableProps} /></div>
      {:else if target === "carousel"}
        <CarouselCardProfile sectionCarouselButtonsProps={{ titleProps: { text: entity.name }, descriptionProps: { text: "Response OpenAPI apresentada em carousel." } }} items={cards} />
      {:else if target === "grid"}
        <GridCardProfile sectionGridProps={{ titleProps: { text: entity.name }, descriptionProps: { text: "Response OpenAPI apresentada em grelha." } }} items={cards} />
      {:else if target === "grid-carousel"}
        <GridCarouselCardProfile sectionGridCarouselProps={{ titleProps: { text: entity.name }, descriptionProps: { text: "Response OpenAPI apresentada em grelha navegável." }, rows: 1, columns: 3 }} items={cards} />
      {/if}
    </div>
    <footer class="flex flex-wrap justify-between gap-3 border-t border-blender-border p-4">
      <div class="text-[10px] text-zinc-500">Rota da biblioteca: {targets.find((item) => item.id === target)?.route}</div>
      <div class="flex gap-2"><button type="button" class="rounded-md border border-blender-border px-3 py-2 text-xs text-zinc-300 hover:bg-blender-hover" onclick={onclose}>Fechar</button><button type="button" class="rounded-md bg-blue-600 px-3 py-2 text-xs font-semibold text-white hover:bg-blue-500" onclick={openTargetRoute}>Abrir componente na biblioteca</button></div>
    </footer>
  </div>
</div>
