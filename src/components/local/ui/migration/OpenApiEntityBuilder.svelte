<script lang="ts">
  import { goto } from "$app/navigation";
  import type { OpenApiEntity } from "../../data/migration";
  import EntityFormPreview from "./openapi-page/EntityFormPreview.svelte";
  import EntityTablePreview from "./openapi-page/EntityTablePreview.svelte";
  import EntityCardPreviews from "./openapi-page/EntityCardPreviews.svelte";

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
  let submissionMessage = $state("");
  let submitting = $state(false);
  const operations = $derived(entity.operations ?? []);
  const activeOperation = $derived(operations[operationIndex]);

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
        <EntityFormPreview {entity} target={target} bind:operationIndex {submitting} {submissionMessage} onsubmit={submitRequest} />
      {:else if target === "data-table"}
        <EntityTablePreview {entity} />
      {:else if target === "carousel" || target === "grid" || target === "grid-carousel"}
        <EntityCardPreviews {entity} {target} />
      {/if}
    </div>
    <footer class="flex flex-wrap justify-between gap-3 border-t border-blender-border p-4">
      <div class="text-[10px] text-zinc-500">Rota da biblioteca: {targets.find((item) => item.id === target)?.route}</div>
      <div class="flex gap-2"><button type="button" class="rounded-md border border-blender-border px-3 py-2 text-xs text-zinc-300 hover:bg-blender-hover" onclick={onclose}>Fechar</button><button type="button" class="rounded-md bg-blue-600 px-3 py-2 text-xs font-semibold text-white hover:bg-blue-500" onclick={openTargetRoute}>Abrir componente na biblioteca</button></div>
    </footer>
  </div>
</div>
