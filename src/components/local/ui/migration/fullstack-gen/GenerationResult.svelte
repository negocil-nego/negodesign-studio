<script lang="ts">
  import { LocalStyleComponent } from "../../../data/style.component";
  import type { FullstackCodegenResult } from "../../../data/migration";

  let {
    errorMessage = "",
    result = null,
  }: {
    errorMessage?: string;
    result?: FullstackCodegenResult | null;
  } = $props();
</script>

{#if errorMessage}
  <p class={LocalStyleComponent.migration.error} role="alert">{errorMessage}</p>
{/if}

{#if result}
  <div class="rounded-xl border border-emerald-500/40 bg-emerald-500/10 p-4 text-xs text-emerald-200 space-y-2">
    <p class="font-semibold text-emerald-300">Geração concluída com sucesso ({result.tablesProcessed} tabela(s) processada(s))!</p>
    <ul class="list-disc pl-5 space-y-1">
      {#each result.summary as line}
        <li>{line}</li>
      {/each}
    </ul>
  </div>
{/if}
