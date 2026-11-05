<script lang="ts">
  import { StyleComponent } from "../../data/style.component";
  import { fetchOpenApi } from "../../data/migration";

  let {
    oncomplete,
    oncancel,
  }: { oncomplete: () => void; oncancel: () => void } = $props();

  let url = $state("");
  let loading = $state(false);
  let errorMessage = $state("");

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    loading = true;
    errorMessage = "";
    try {
      await fetchOpenApi(url.trim());
      oncomplete();
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      loading = false;
    }
  }
</script>

<div
  class={StyleComponent.migration.overlay}
  role="presentation"
  onclick={(event) => event.target === event.currentTarget && !loading && oncancel()}
  onkeydown={(event) => event.key === "Escape" && !loading && oncancel()}
>
  <div
    class="{StyleComponent.migration.dialog} max-w-md"
    role="dialog"
    aria-modal="true"
    aria-labelledby="openapi-dialog-title"
  >
    <form onsubmit={submit}>
    <div class="flex items-start justify-between gap-4">
      <div>
        <h2 id="openapi-dialog-title" class="text-base font-semibold text-zinc-100">Importar OpenApi</h2>
        <p class="mt-1 text-xs leading-5 text-zinc-400">
          Indique a rota da documentação Swagger/OpenAPI (JSON).
        </p>
      </div>
      <button
        type="button"
        class="rounded px-2 py-1 text-zinc-400 hover:bg-blender-hover hover:text-zinc-100"
        aria-label="Fechar"
        onclick={oncancel}
        disabled={loading}>×</button
      >
    </div>

    <label class="mt-5 block">
      <span class={StyleComponent.migration.label}>Rota da documentação</span>
      <input
        class={StyleComponent.migration.field}
        type="url"
        bind:value={url}
        required
        disabled={loading}
        placeholder="https://api.exemplo.com/v3/api-docs"
      />
    </label>

    {#if errorMessage}<p class={StyleComponent.migration.error} role="alert">{errorMessage}</p>{/if}

    <div class="mt-5 flex justify-end gap-2">
      <button type="button" class={StyleComponent.migration.ghostButton} onclick={oncancel} disabled={loading}>
        Cancelar
      </button>
      <button type="submit" class={StyleComponent.migration.primaryButton} disabled={loading || !url.trim()}>
        {loading ? "A extrair…" : "Extrair entidades"}
      </button>
    </div>
    </form>
  </div>
</div>
