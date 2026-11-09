<script lang="ts">
  import { goto } from "$app/navigation";
  import { LocalStyleComponent } from "../../data/style.component";
  import DatabaseConnectDialog from "./DatabaseConnectDialog.svelte";
  import ApiDocsSection from "./ApiDocsSection.svelte";

  let { onclose }: { onclose: () => void } = $props();
  let activeDialog = $state<"database" | "apidocs" | null>(null);

  function finish(path: string) {
    activeDialog = null;
    onclose();
    void goto(path);
  }
</script>

<div>
  <h3 class={LocalStyleComponent.migration.sectionTitle}>
    Migração de projecto
  </h3>
  <div class="grid gap-3 sm:grid-cols-2">
    <button
      type="button"
      class={LocalStyleComponent.migration.optionCard}
      onclick={() => (activeDialog = "database")}
    >
      <span class={LocalStyleComponent.migration.optionIcon} aria-hidden="true"
        >⛁</span
      >
      <span class={LocalStyleComponent.migration.optionTitle}
        >Banco de dados</span
      >
      <span class={LocalStyleComponent.migration.optionHint}>
        Ligue-se a um MySQL ou PostgreSQL e importe a estrutura das tabelas.
      </span>
    </button>
    <button
      type="button"
      class={LocalStyleComponent.migration.optionCard}
      onclick={() => (activeDialog = "apidocs")}
    >
      <span class={LocalStyleComponent.migration.optionIcon} aria-hidden="true"
        >◈</span
      >
      <span class={LocalStyleComponent.migration.optionTitle}>ApiDocs</span>
      <span class={LocalStyleComponent.migration.optionHint}>
        Ligue-se ao OpenAPI/Swagger por URL ou ficheiro JSON/YAML e componentize
        os DTOs e responses.
      </span>
    </button>
  </div>
</div>

{#if activeDialog === "database"}
  <DatabaseConnectDialog
    oncomplete={() => finish("/main/database")}
    oncancel={() => (activeDialog = null)}
  />
{:else if activeDialog === "apidocs"}
  <div
    class={LocalStyleComponent.migration.overlay}
    role="presentation"
    onclick={(e) => e.target === e.currentTarget && (activeDialog = null)}
    onkeydown={(e) => e.key === "Escape" && (activeDialog = null)}
  >
    <div class="{LocalStyleComponent.migration.dialog} max-w-lg">
      <ApiDocsSection onclose={() => finish("/main/openapi")} />
    </div>
  </div>
{/if}
