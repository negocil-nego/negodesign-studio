<script lang="ts">
  import {
    classifyOpenApiOperation,
    type OpenApiCrudRole,
    type OpenApiOperation,
  } from "../../../data/migration";

  let {
    operations,
    roles,
  }: {
    operations: OpenApiOperation[];
    roles: Record<OpenApiCrudRole, OpenApiOperation[]>;
  } = $props();

  const crudRoleLabels: Record<OpenApiCrudRole, string> = {
    list: "GET · Listar (/list, /all)",
    get: "GET · Detalhe (/{id})",
    create: "POST · Criar (/create, /add)",
    update: "PUT/PATCH · Actualizar (/update, /edit, /save)",
    delete: "DELETE · Remover (/delete, /remove)",
  };

  function methodBadgeClass(method: string): string {
    const m = method.toUpperCase();
    if (m === "GET")
      return "border-emerald-500/30 bg-emerald-500/10 text-emerald-400";
    if (m === "POST")
      return "border-blue-500/30 bg-blue-500/10 text-blue-400";
    if (m === "PUT" || m === "PATCH")
      return "border-amber-500/30 bg-amber-500/10 text-amber-400";
    if (m === "DELETE")
      return "border-rose-500/30 bg-rose-500/10 text-rose-400";
    return "border-purple-500/30 bg-purple-500/10 text-purple-400";
  }
</script>

<div
  class="flex flex-col gap-1.5 border-b border-blender-border/60 bg-blender-base/30 px-4 py-2.5"
>
  <div class="flex items-center justify-between text-[10px] text-zinc-400">
    <span class="font-semibold uppercase tracking-wider">
      Endpoints Identificados para AdminCrud ({operations.length})
    </span>
    <div class="flex gap-1.5">
      {#if roles.list.length}
        <span class="rounded bg-emerald-500/15 px-1.5 py-0.5 text-[9px] font-semibold text-emerald-400">GET/LIST</span>
      {/if}
      {#if roles.create.length}
        <span class="rounded bg-blue-500/15 px-1.5 py-0.5 text-[9px] font-semibold text-blue-400">POST/CREATE</span>
      {/if}
      {#if roles.update.length}
        <span class="rounded bg-amber-500/15 px-1.5 py-0.5 text-[9px] font-semibold text-amber-400">PUT·PATCH/UPDATE</span>
      {/if}
      {#if roles.delete.length}
        <span class="rounded bg-rose-500/15 px-1.5 py-0.5 text-[9px] font-semibold text-rose-400">DELETE</span>
      {/if}
    </div>
  </div>
  <div class="flex flex-wrap gap-1.5">
    {#each operations as operation (`${operation.method}-${operation.path}`)}
      {@const info = classifyOpenApiOperation(operation)}
      <span
        class="inline-flex items-center gap-1.5 rounded border px-2 py-1 font-mono text-[10px] {methodBadgeClass(operation.method)}"
        title={`${crudRoleLabels[info.crudRole]} (${info.patternStyle === "named" ? "Nomenclatura de Método" : "API RESTful"})`}
      >
        <span class="font-bold">{operation.method}</span>
        <span>{operation.path}</span>
        <span class="rounded bg-black/30 px-1 py-0.2 text-[8px] uppercase tracking-wider opacity-85">
          {info.crudRole} · {info.patternStyle === "named" ? " método" : "rest"}
        </span>
      </span>
    {/each}
  </div>
</div>
