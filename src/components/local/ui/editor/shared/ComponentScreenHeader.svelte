<script lang="ts">
  import { activeProject } from "../../../data/projects";

  let { count, status = "" }: { count: number; status?: string } = $props();
</script>

<header class="space-y-4">
  <div class="flex items-end justify-between gap-4">
    <div>
      <p class="text-xs font-semibold uppercase tracking-[0.16em] text-blue-500">Project UI</p>
      <h1 class="mt-1 text-2xl font-semibold text-zinc-100">Componentes</h1>
      <p class="mt-1 text-sm text-zinc-400">Pré-visualização dos componentes existentes neste projecto.</p>
    </div>
    <span class="shrink-0 rounded-full border border-blender-border bg-blender-card px-3 py-1 text-xs font-medium text-zinc-400">{count} componentes</span>
  </div>

  <section class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-blender-border bg-blender-card px-4 py-3">
    <div class="min-w-0">
      <p class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">Linguagem do projecto activo</p>
      {#if $activeProject}
        <p class="mt-1 truncate text-sm font-semibold text-zinc-100">{$activeProject.language}<span class="font-normal text-zinc-400"> · {$activeProject.name}</span></p>
      {:else}
        <p class="mt-1 text-sm text-zinc-400">Escolha um projecto no Inspector para activar a cópia de componentes.</p>
      {/if}
    </div>
    {#if $activeProject}<span class="rounded-full border border-blue-500/30 bg-blue-500/10 px-3 py-1 text-xs font-medium text-blue-400">{$activeProject.language}</span>{/if}
  </section>
  {#if status}<p class="text-xs text-zinc-400" role="status">{status}</p>{/if}
</header>
