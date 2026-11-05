<script lang="ts">
  import { StyleComponent } from "../../data/style.component";
  import { fetchOpenApi, importOpenApiFile } from "../../data/migration";

  let {
    oncomplete,
    oncancel,
  }: { oncomplete: () => void; oncancel: () => void } = $props();

  let url = $state("");
  let loading = $state(false);
  let errorMessage = $state("");
  let sourceMode = $state<"url" | "file">("url");

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

  async function chooseFile() {
    loading = true;
    errorMessage = "";
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const path = await open({
        multiple: false,
        title: "Seleccionar ficheiro OpenAPI",
        filters: [{ name: "OpenAPI", extensions: ["json", "yaml", "yml"] }],
      });
      if (!path || Array.isArray(path)) return;
      await importOpenApiFile(path);
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
          Importe OpenAPI por URL ou seleccione um ficheiro JSON/YAML.
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

    <div class="mt-5 flex gap-2">
      <button type="button" class="{StyleComponent.migration.driverTab} {sourceMode === 'url' ? StyleComponent.migration.driverTabActive : ''}" onclick={() => sourceMode = "url"}>URL</button>
      <button type="button" class="{StyleComponent.migration.driverTab} {sourceMode === 'file' ? StyleComponent.migration.driverTabActive : ''}" onclick={() => sourceMode = "file"}>Ficheiro JSON/YAML</button>
    </div>
    {#if sourceMode === "url"}
      <label class="mt-4 block">
        <span class={StyleComponent.migration.label}>URL OpenAPI</span>
        <input class={StyleComponent.migration.field} type="url" bind:value={url} required disabled={loading} placeholder="http://localhost:8080/v3/api-docs" />
      </label>
    {:else}
      <button type="button" class="{StyleComponent.migration.ghostButton} mt-4 w-full" onclick={chooseFile} disabled={loading}>
        {loading ? "A importar…" : "Seleccionar ficheiro JSON, YAML ou YML"}
      </button>
    {/if}

    {#if errorMessage}<p class={StyleComponent.migration.error} role="alert">{errorMessage}</p>{/if}

    <div class="mt-5 flex justify-end gap-2">
      <button type="button" class={StyleComponent.migration.ghostButton} onclick={oncancel} disabled={loading}>
        Cancelar
      </button>
      {#if sourceMode === "url"}<button type="submit" class={StyleComponent.migration.primaryButton} disabled={loading || !url.trim()}>
        {loading ? "A extrair…" : "Extrair entidades"}
      </button>{/if}
    </div>
    </form>
  </div>
</div>
