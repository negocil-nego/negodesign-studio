<script lang="ts">
  import { goto } from "$app/navigation";
  import { StyleComponent } from "../../data/style.component";
  import DatabaseConnectDialog from "./DatabaseConnectDialog.svelte";
  import OpenApiConnectDialog from "./OpenApiConnectDialog.svelte";

  let { onclose }: { onclose: () => void } = $props();
  let activeDialog = $state<"database" | "openapi" | null>(null);

  function finish(path: string) {
    activeDialog = null;
    onclose();
    void goto(path);
  }
</script>

<div>
  <h3 class={StyleComponent.migration.sectionTitle}>Migração de projecto</h3>
  <div class="grid gap-3 sm:grid-cols-2">
    <button
      type="button"
      class={StyleComponent.migration.optionCard}
      onclick={() => (activeDialog = "database")}
    >
      <span class={StyleComponent.migration.optionIcon} aria-hidden="true">⛁</span>
      <span class={StyleComponent.migration.optionTitle}>Banco de dados</span>
      <span class={StyleComponent.migration.optionHint}>
        Ligue-se a um MySQL ou PostgreSQL e importe a estrutura das tabelas.
      </span>
    </button>
    <button
      type="button"
      class={StyleComponent.migration.optionCard}
      onclick={() => (activeDialog = "openapi")}
    >
      <span class={StyleComponent.migration.optionIcon} aria-hidden="true">◈</span>
      <span class={StyleComponent.migration.optionTitle}>OpenApi</span>
      <span class={StyleComponent.migration.optionHint}>
        Indique a rota Swagger/OpenAPI e extraia as entidades (response, dto).
      </span>
    </button>
  </div>
</div>

{#if activeDialog === "database"}
  <DatabaseConnectDialog
    oncomplete={() => finish("/main/database")}
    oncancel={() => (activeDialog = null)}
  />
{:else if activeDialog === "openapi"}
  <OpenApiConnectDialog
    oncomplete={() => finish("/main/openapi")}
    oncancel={() => (activeDialog = null)}
  />
{/if}
