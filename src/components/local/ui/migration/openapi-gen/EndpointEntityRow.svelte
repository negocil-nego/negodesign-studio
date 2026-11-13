<script lang="ts">
  import {
    classifyOpenApiOperation,
    summarizeEntityCrudPattern,
    type OpenApiEntity,
    type OpenApiOperation,
  } from "../../../data/migration";
  import { defaultOperationsForEntity, endpointKey, methodColor } from "./endpoints";

  let {
    entity,
    isSelected,
    selectedEndpointKeys,
    ontoggleentity,
    ontoggleendpoint,
  }: {
    entity: OpenApiEntity;
    isSelected: boolean;
    selectedEndpointKeys: string[];
    ontoggleentity: (name: string) => void;
    ontoggleendpoint: (entityName: string, op: OpenApiOperation) => void;
  } = $props();

  const ops = $derived(defaultOperationsForEntity(entity));
  const crudInfo = $derived(summarizeEntityCrudPattern(entity));
</script>

<div class="p-3 hover:bg-blender-base/70 transition space-y-2">
  <div class="flex items-center justify-between gap-2">
    <label class="flex items-center gap-2.5 min-w-0 cursor-pointer flex-1">
      <input
        type="checkbox"
        checked={isSelected}
        class="rounded border-blender-border text-blue-500 focus:ring-0"
        onchange={() => ontoggleentity(entity.name)}
      />
      <span class="font-mono font-semibold text-zinc-200 truncate">{entity.name}</span>
      <span class="rounded px-1.5 py-0.5 text-[9px] font-semibold uppercase bg-zinc-800 text-zinc-400 border border-zinc-700">
        {entity.kind}
      </span>
      <span class="rounded border border-emerald-500/30 bg-emerald-500/10 px-1.5 py-0.5 text-[9px] text-emerald-300">
        {crudInfo.styleLabel}
      </span>
    </label>
    <span class="shrink-0 text-[10px] text-zinc-400">
      {entity.fields.length} campos · {ops.length} endpoint(s)
    </span>
  </div>

  <div class="grid gap-1.5 pl-6 sm:grid-cols-2">
    {#each ops as op (`${op.method}-${op.path}`)}
      {@const key = endpointKey(entity.name, op.method, op.path)}
      {@const opChecked = selectedEndpointKeys.includes(key)}
      {@const classified = classifyOpenApiOperation(op)}
      <label
        class={`flex items-center justify-between gap-2 rounded border px-2 py-1.5 text-[10px] cursor-pointer transition ${
          opChecked
            ? "border-blue-500/40 bg-blue-500/10 text-zinc-200"
            : "border-blender-border/60 bg-blender-card/50 text-zinc-500"
        }`}
      >
        <div class="flex items-center gap-1.5 min-w-0">
          <input
            type="checkbox"
            checked={opChecked}
            onchange={() => ontoggleendpoint(entity.name, op)}
            class="size-3.5 rounded border-blender-border text-blue-500"
          />
          <span class="rounded border px-1 py-0.2 text-[9px] font-mono font-bold {methodColor(op.method)}">
            {op.method}
          </span>
          <span class="truncate font-mono" title={op.path}>{op.path}</span>
        </div>
        <span class="shrink-0 rounded bg-black/30 px-1.5 py-0.5 text-[8px] font-semibold uppercase tracking-wider text-zinc-300">
          {classified.crudRole} · {classified.patternStyle === "named" ? "método" : "rest"}
        </span>
      </label>
    {/each}
  </div>
</div>
