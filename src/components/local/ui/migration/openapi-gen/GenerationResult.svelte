<script lang="ts">
  import type { OpenApiFrontendCodegenResult } from "../../../data/migration";

  let {
    errorMessage = "",
    result = null,
  }: {
    errorMessage?: string;
    result?: OpenApiFrontendCodegenResult | null;
  } = $props();
</script>

{#if errorMessage}
  <div class="rounded-lg border border-red-500/30 bg-red-500/10 p-3 text-red-500">
    <p class="font-semibold">Erro:</p>
    <p>{errorMessage}</p>
  </div>
{/if}

{#if result}
  <div class="rounded-xl border border-emerald-500/30 bg-emerald-500/10 p-5 space-y-3">
    <div class="flex items-center gap-2 text-emerald-500 font-semibold text-sm">
      <span>✓</span>
      <span>Frontend OpenAPI gerado com sucesso!</span>
    </div>
    <div class="grid grid-cols-3 gap-3 text-center">
      <div class="rounded-lg bg-blender-base/60 p-2.5 border border-emerald-500/20">
        <span class="block text-lg font-bold text-emerald-400">{result.endpointsProcessed}</span>
        <span class="text-[10px] text-zinc-400 uppercase tracking-wider">Entidades / Endpoints</span>
      </div>
      <div class="rounded-lg bg-blender-base/60 p-2.5 border border-emerald-500/20">
        <span class="block text-lg font-bold text-emerald-400">{result.generatedFrontendFiles}</span>
        <span class="text-[10px] text-zinc-400 uppercase tracking-wider">Ficheiros Gerados</span>
      </div>
      <div class="rounded-lg bg-blender-base/60 p-2.5 border border-emerald-500/20">
        <span class="block text-lg font-bold text-emerald-400">{result.copiedUiFiles}</span>
        <span class="text-[10px] text-zinc-400 uppercase tracking-wider">Componentes UI</span>
      </div>
    </div>
    <ul class="space-y-1 text-[11px] text-zinc-400 list-disc list-inside pt-2 border-t border-emerald-500/20">
      {#each result.summary as line}
        <li>{line}</li>
      {/each}
    </ul>
  </div>
{/if}
