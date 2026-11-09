<script lang="ts">
  import { goto } from "$app/navigation";
  import { LocalStyleComponent } from "../../data/style.component";
  import {
    fetchOpenApi,
    importOpenApiFile,
    openApiEntities,
  } from "../../data/migration";

  let { onclose }: { onclose: () => void } = $props();

  /* ── state ── */
  let sourceMode = $state<"url" | "file">("url");
  let url = $state("");
  let loading = $state(false);
  let errorMessage = $state("");
  let success = $state(false);

  /* ── handlers ── */
  async function submitUrl(event: SubmitEvent) {
    event.preventDefault();
    if (!url.trim()) return;
    loading = true;
    errorMessage = "";
    try {
      const entities = await fetchOpenApi(url.trim());
      success = true;
      openApiEntities.set(entities);
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
      const entities = await importOpenApiFile(path);
      success = true;
      openApiEntities.set(entities);
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      loading = false;
    }
  }

  function openApiDocs() {
    onclose();
    void goto("/main/openapi");
  }

  /* ── entity summary ── */
  const entityCount = $derived($openApiEntities.length);
  const requestCount = $derived(
    $openApiEntities.filter((e) => e.kind === "dto" || e.kind === "request")
      .length,
  );
  const responseCount = $derived(
    $openApiEntities.filter((e) => e.kind === "response" || e.kind === "schema")
      .length,
  );
</script>

<div class="space-y-5">
  <!-- Header row -->
  <div class="flex items-center gap-2">
    <span class="text-lg" aria-hidden="true">◈</span>
    <div>
      <h3 class="text-sm font-semibold text-zinc-800 dark:text-zinc-100">
        ApiDocs · OpenAPI
      </h3>
      <p class="text-xs text-zinc-500 dark:text-zinc-400">
        Ligue-se a um servidor Swagger/OpenAPI ou importe um ficheiro JSON/YAML
        e componentize as entidades.
      </p>
    </div>
  </div>

  <!-- Source mode tabs -->
  <div
    class="flex gap-1 rounded-lg border border-blender-border bg-blender-base p-1 w-fit"
  >
    <button
      type="button"
      class="rounded-md px-3 py-1 text-xs font-medium transition-colors {sourceMode ===
      'url'
        ? 'bg-white dark:bg-blender-card text-zinc-800 dark:text-zinc-100 shadow-sm'
        : 'text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300'}"
      onclick={() => (sourceMode = "url")}
    >
      URL
    </button>
    <button
      type="button"
      class="rounded-md px-3 py-1 text-xs font-medium transition-colors {sourceMode ===
      'file'
        ? 'bg-white dark:bg-blender-card text-zinc-800 dark:text-zinc-100 shadow-sm'
        : 'text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300'}"
      onclick={() => (sourceMode = "file")}
    >
      Ficheiro JSON / YAML
    </button>
  </div>

  <!-- URL form -->
  {#if sourceMode === "url"}
    <form onsubmit={submitUrl} class="space-y-3">
      <label class="block">
        <span class={LocalStyleComponent.migration.label}
          >URL do endpoint OpenAPI</span
        >
        <input
          class="{LocalStyleComponent.migration.field} mt-1"
          type="url"
          bind:value={url}
          required
          disabled={loading}
          placeholder="http://localhost:8080/v3/api-docs"
          autocomplete="off"
        />
        <span class="mt-1 block text-[10px] text-zinc-500">
          Exemplo: <code class="font-mono"
            >http://localhost:8080/v3/api-docs</code
          >
          (formato JSON ou YAML servido pelo Swagger/SpringDoc)
        </span>
      </label>
      {#if errorMessage}
        <p class={LocalStyleComponent.migration.error} role="alert">
          {errorMessage}
        </p>
      {/if}
      <button
        type="submit"
        class={LocalStyleComponent.migration.primaryButton}
        disabled={loading || !url.trim()}
      >
        {loading ? "A extrair entidades…" : "Extrair entidades"}
      </button>
    </form>
  {:else}
    <!-- File picker -->
    <div class="space-y-3">
      <button
        type="button"
        class="{LocalStyleComponent.migration.ghostButton} w-full"
        onclick={chooseFile}
        disabled={loading}
      >
        {loading ? "A importar…" : "Seleccionar ficheiro JSON, YAML ou YML"}
      </button>
      {#if errorMessage}
        <p class={LocalStyleComponent.migration.error} role="alert">
          {errorMessage}
        </p>
      {/if}
    </div>
  {/if}

  <!-- Success / entity summary -->
  {#if success || entityCount > 0}
    <div
      class="rounded-lg border border-green-500/30 bg-green-500/5 p-4 space-y-3"
    >
      <div class="flex items-center gap-2">
        <span class="text-green-400 text-sm" aria-hidden="true">✓</span>
        <p class="text-xs font-semibold text-green-400">
          {entityCount} entidade(s) extraída(s) com sucesso
        </p>
      </div>

      <!-- Summary by kind -->
      <div class="flex flex-wrap gap-3">
        {#if requestCount > 0}
          <div
            class="rounded-md border border-blender-border bg-blender-card px-3 py-2 text-center min-w-20"
          >
            <p class="text-base font-bold text-zinc-100">{requestCount}</p>
            <p class="text-[10px] text-zinc-500 uppercase tracking-wider">
              DTO / Request
            </p>
            <p class="mt-1 text-[9px] text-zinc-600 leading-tight">
              Formulários · Modal Form
            </p>
          </div>
        {/if}
        {#if responseCount > 0}
          <div
            class="rounded-md border border-blender-border bg-blender-card px-3 py-2 text-center min-w-20"
          >
            <p class="text-base font-bold text-zinc-100">{responseCount}</p>
            <p class="text-[10px] text-zinc-500 uppercase tracking-wider">
              Response / Schema
            </p>
            <p class="mt-1 text-[9px] text-zinc-600 leading-tight">
              DataTable · Grid · Carousel
            </p>
          </div>
        {/if}
      </div>

      <!-- Guide -->
      <div
        class="rounded-md bg-blender-base/60 p-3 space-y-1.5 text-[10px] text-zinc-500 leading-relaxed"
      >
        <p
          class="font-semibold text-zinc-400 uppercase tracking-wider text-[9px]"
        >
          Como componentizar
        </p>
        <p>
          <span class="text-blue-400 font-medium">DTO / Request</span> →
          Formulário de criação ou edição (<code class="font-mono">Form</code>,
          <code class="font-mono">ModalForm</code>) com rota de submissão
          identificada automaticamente.
        </p>
        <p>
          <span class="text-purple-400 font-medium">Response / Schema</span> →
          Componente de apresentação (<code class="font-mono">DataTable</code>,
          <code class="font-mono">Grid</code>,
          <code class="font-mono">Carousel</code>,
          <code class="font-mono">GridCarousel</code>).
        </p>
      </div>

      <button
        type="button"
        class={LocalStyleComponent.migration.primaryButton}
        onclick={openApiDocs}
      >
        Abrir entidades e componentizar →
      </button>
    </div>
  {/if}
</div>
