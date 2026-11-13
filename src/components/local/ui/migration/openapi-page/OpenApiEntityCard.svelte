<script lang="ts">
  import { entityKindLabels, summarizeEntityCrudPattern, type OpenApiEntity } from "../../../data/migration";
  import { LocalStyleComponent } from "../../../data/style.component";
  import OpenApiEndpointBadges from "./OpenApiEndpointBadges.svelte";

  let {
    entity,
    isChecked,
    onToggle,
    onComponentize,
    onGenerate,
  }: {
    entity: OpenApiEntity;
    isChecked: boolean;
    checkedCount?: number;
    onToggle: () => void;
    onComponentize: () => void;
    onGenerate: () => void;
  } = $props();

  const isCreation = $derived(entity.kind === "dto" || entity.kind === "request");
  const crudSummary = $derived(summarizeEntityCrudPattern(entity));
</script>

<article class={`${LocalStyleComponent.migration.card} ${isChecked ? "ring-1 ring-emerald-500/60" : ""}`}>
  <header class="flex items-center justify-between gap-3 border-b border-blender-border px-4 py-3">
    <label class="flex items-center gap-2.5 min-w-0 cursor-pointer flex-1">
      <input type="checkbox" checked={isChecked} onchange={onToggle} class="size-4 rounded border-blender-border text-emerald-600" />
      <span class="shrink-0 rounded px-1.5 py-0.5 text-[9px] font-semibold uppercase tracking-wider {isCreation ? 'bg-blue-500/10 text-blue-400' : 'bg-purple-500/10 text-purple-400'}">
        {isCreation ? "DTO/Request" : "Response"}
      </span>
      <h2 class="truncate font-mono text-sm font-semibold text-zinc-800 dark:text-zinc-100" title={entity.name}>{entity.name}</h2>
    </label>
    <div class="flex items-center gap-1.5 shrink-0">
      <span class="rounded border border-emerald-500/30 bg-emerald-500/10 px-2 py-0.5 text-[9px] font-medium text-emerald-300" title="Padrão de endpoints identificado para AdminCrud">{crudSummary.styleLabel}</span>
      <span class={LocalStyleComponent.migration.badge}>{entityKindLabels[entity.kind]}</span>
    </div>
  </header>
  {#if entity.operations?.length}
    <OpenApiEndpointBadges operations={entity.operations} roles={crudSummary.roles} />
  {/if}
  {#if entity.fields.length}
    <div class="overflow-x-auto">
      <table class="w-full text-left text-xs">
        <thead class={LocalStyleComponent.migration.cellHead}>
          <tr>
            <th class="px-4 py-2 font-semibold">Campo</th>
            <th class="px-4 py-2 font-semibold">Tipo</th>
            <th class="px-4 py-2 font-semibold">Obrigatório</th>
          </tr>
        </thead>
        <tbody>
          {#each entity.fields as field (field.name)}
            <tr class="border-t border-blender-border/60">
              <td class="px-4 py-2 font-mono text-zinc-700 dark:text-zinc-200">{field.name}</td>
              <td class="px-4 py-2 text-zinc-500">{field.type}</td>
              <td class="px-4 py-2 text-zinc-500">{field.required ? "Sim" : "Não"}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <p class="px-4 py-3 text-xs text-zinc-500">Sem campos definidos.</p>
  {/if}
  <footer class="flex items-center justify-between gap-3 border-t border-blender-border/60 px-4 py-3">
    <p class="text-[10px] text-zinc-500">
      {#if isCreation}
        Componentes: <span class="text-blue-400">Form · ModalForm</span>
      {:else}
        Componentes: <span class="text-purple-400">DataTable · Grid · Carousel · GridCarousel</span>
      {/if}
    </p>
    <div class="flex items-center gap-2">
      <button type="button" class="rounded-md border border-emerald-500/30 bg-emerald-500/10 px-2.5 py-1.5 text-[11px] font-semibold text-emerald-400 hover:bg-emerald-500/20 transition-colors" onclick={onGenerate}>◈ Gerar AdminCrud</button>
      <button type="button" class="rounded-md bg-blue-600 px-3 py-1.5 text-[11px] font-semibold text-white hover:bg-blue-500 transition-colors" onclick={onComponentize}>Componentizar {isCreation ? "formulário" : "apresentação"}</button>
    </div>
  </footer>
</article>
