<script lang="ts">
  import { goto } from "$app/navigation";
  import { StyleComponent } from "../../data/style.component";
  import MigrationSection from "./MigrationSection.svelte";
  import ApiDocsSection from "./ApiDocsSection.svelte";

  let { open = false, onclose }: { open?: boolean; onclose: () => void } = $props();

  /** Painel activo no dialog: "start" | "apidocs" */
  let activePanel = $state<"start" | "apidocs">("start");

  function go(path: string) {
    onclose();
    void goto(path);
  }

  function resetPanel() {
    activePanel = "start";
  }
</script>

{#if open}
  <div
    class={StyleComponent.migration.overlay}
    role="presentation"
    onclick={(event) => event.target === event.currentTarget && onclose()}
    onkeydown={(event) => event.key === "Escape" && onclose()}
  >
    <div
      class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-xl border border-blender-border bg-white p-6 text-slate-900 shadow-2xl dark:bg-blender-card dark:text-zinc-100"
      role="dialog"
      aria-modal="true"
      aria-labelledby="project-options-title"
    >
      <!-- Header -->
      <div class="mb-6 flex items-start justify-between gap-4">
        <div class="flex items-center gap-2">
          {#if activePanel !== "start"}
            <button
              type="button"
              class="rounded p-1 text-zinc-400 hover:bg-blender-hover hover:text-zinc-100 transition-colors"
              aria-label="Voltar"
              onclick={resetPanel}
            >
              ←
            </button>
          {/if}
          <div>
            <p class="text-xs font-semibold uppercase tracking-wider text-blue-500">Projecto</p>
            <h2 id="project-options-title" class="mt-1 text-xl font-semibold">
              {#if activePanel === "apidocs"}
                ApiDocs · OpenAPI
              {:else}
                Como quer começar?
              {/if}
            </h2>
            {#if activePanel === "start"}
              <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
                Escolha uma opção para criar ou migrar o seu projecto.
              </p>
            {/if}
          </div>
        </div>
        <button
          type="button"
          class="rounded p-2 text-zinc-500 hover:bg-blender-hover"
          aria-label="Fechar"
          onclick={onclose}>✕</button
        >
      </div>

      <!-- Panel: start -->
      {#if activePanel === "start"}
        <div class="mb-6">
          <h3 class={StyleComponent.migration.sectionTitle}>Frontend</h3>
          <div class="grid gap-3 sm:grid-cols-2">
            <button type="button" class={StyleComponent.migration.optionCard} onclick={() => go("/main/template")}>
              <span class={StyleComponent.migration.optionIcon} aria-hidden="true">▦</span>
              <span class={StyleComponent.migration.optionTitle}>Layouts</span>
              <span class={StyleComponent.migration.optionHint}>Conheça os layouts prontos da plataforma.</span>
            </button>
            <button type="button" class={StyleComponent.migration.optionCard} onclick={() => go("/main/project")}>
              <span class={StyleComponent.migration.optionIcon} aria-hidden="true">◫</span>
              <span class={StyleComponent.migration.optionTitle}>Componentes</span>
              <span class={StyleComponent.migration.optionHint}>Explore os componentes disponíveis.</span>
            </button>
          </div>
        </div>

        <!-- ApiDocs option card -->
        <div class="mb-6">
          <h3 class={StyleComponent.migration.sectionTitle}>Backend · API</h3>
          <div class="grid gap-3 sm:grid-cols-2">
            <button
              type="button"
              class={StyleComponent.migration.optionCard}
              onclick={() => (activePanel = "apidocs")}
            >
              <span class={StyleComponent.migration.optionIcon} aria-hidden="true">◈</span>
              <span class={StyleComponent.migration.optionTitle}>ApiDocs</span>
              <span class={StyleComponent.migration.optionHint}>
                Ligue-se ao OpenAPI/Swagger por URL ou ficheiro JSON/YAML e componentize os DTOs e responses.
              </span>
            </button>
          </div>
        </div>

        <MigrationSection {onclose} />

      <!-- Panel: apidocs -->
      {:else if activePanel === "apidocs"}
        <ApiDocsSection onclose={() => { resetPanel(); onclose(); }} />
      {/if}
    </div>
  </div>
{/if}
