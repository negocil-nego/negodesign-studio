<script lang="ts">
  import { onMount } from "svelte";
  import { LocalStyleComponent } from "../../data/style.component";
  import {
    connectDatabase,
    connectSavedDatabase,
    listSavedDatabaseConnections,
    type DatabaseConnection,
    type DatabaseDriver,
    type SavedDatabaseConnection,
  } from "../../data/migration";

  let {
    oncomplete,
    oncancel,
  }: { oncomplete: () => void; oncancel: () => void } = $props();

  const drivers: { id: DatabaseDriver; label: string; port: number }[] = [
    { id: "mysql", label: "MySQL", port: 3306 },
    { id: "postgres", label: "PostgreSQL", port: 5432 },
  ];

  let driver = $state<DatabaseDriver>("postgres");
  let host = $state("localhost");
  let port = $state(5432);
  let database = $state("");
  let user = $state("");
  let password = $state("");
  let connecting = $state(false);
  let errorMessage = $state("");
  let recentConnections = $state<SavedDatabaseConnection[]>([]);

  onMount(async () => {
    try {
      recentConnections = await listSavedDatabaseConnections();
    } catch (error) {
      console.error("Não foi possível carregar as ligações recentes:", error);
    }
  });

  function selectDriver(id: DatabaseDriver) {
    const target = drivers.find((item) => item.id === id);
    if (!target || target.id === driver) return;
    driver = target.id;
    port = target.port;
  }

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    await connectAndOpen({
      driver,
      host,
      port: Number(port),
      database,
      user,
      password,
    });
  }

  async function connectAndOpen(connection: DatabaseConnection) {
    connecting = true;
    errorMessage = "";
    try {
      await connectDatabase(connection);
      recentConnections = await listSavedDatabaseConnections();
      oncomplete();
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      connecting = false;
    }
  }

  async function useRecent(connection: SavedDatabaseConnection) {
    connecting = true;
    errorMessage = "";
    try {
      await connectSavedDatabase(connection.id);
      recentConnections = await listSavedDatabaseConnections();
      oncomplete();
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      connecting = false;
    }
  }
</script>

<div
  class={LocalStyleComponent.migration.overlay}
  role="presentation"
  onclick={(event) =>
    event.target === event.currentTarget && !connecting && oncancel()}
  onkeydown={(event) => event.key === "Escape" && !connecting && oncancel()}
>
  <div
    class="{LocalStyleComponent.migration
      .dialog} max-h-[90vh] max-w-md overflow-y-auto"
    role="dialog"
    aria-modal="true"
    aria-labelledby="db-dialog-title"
  >
    <form onsubmit={submit}>
      <div class="flex items-start justify-between gap-4">
        <div>
          <h2
            id="db-dialog-title"
            class="text-base font-semibold text-zinc-100"
          >
            Ligar base de dados
          </h2>
          <p class="mt-1 text-xs leading-5 text-zinc-400">
            Escolha o motor e indique as credenciais de ligação.
          </p>
        </div>
        <button
          type="button"
          class="rounded px-2 py-1 text-zinc-400 hover:bg-blender-hover hover:text-zinc-100"
          aria-label="Fechar"
          onclick={oncancel}
          disabled={connecting}>×</button
        >
      </div>

      <fieldset class="mt-5 flex gap-2" disabled={connecting}>
        {#each drivers as item (item.id)}
          <button
            type="button"
            class="{LocalStyleComponent.migration.driverTab} {driver === item.id
              ? LocalStyleComponent.migration.driverTabActive
              : ''}"
            aria-pressed={driver === item.id}
            onclick={() => selectDriver(item.id)}>{item.label}</button
          >
        {/each}
      </fieldset>

      <div class="mt-4 grid gap-3 sm:grid-cols-2">
        <label class="sm:col-span-2">
          <span class={LocalStyleComponent.migration.label}>Host</span>
          <input
            class={LocalStyleComponent.migration.field}
            bind:value={host}
            required
            disabled={connecting}
            placeholder="localhost"
          />
        </label>
        <label>
          <span class={LocalStyleComponent.migration.label}>Porta</span>
          <input
            class={LocalStyleComponent.migration.field}
            type="number"
            bind:value={port}
            required
            disabled={connecting}
          />
        </label>
        <label>
          <span class={LocalStyleComponent.migration.label}>Base de dados</span>
          <input
            class={LocalStyleComponent.migration.field}
            bind:value={database}
            required
            disabled={connecting}
            placeholder="nome_da_base"
          />
        </label>
        <label>
          <span class={LocalStyleComponent.migration.label}>Utilizador</span>
          <input
            class={LocalStyleComponent.migration.field}
            bind:value={user}
            required
            disabled={connecting}
            autocomplete="username"
          />
        </label>
        <label>
          <span class={LocalStyleComponent.migration.label}>Palavra-passe</span>
          <input
            class={LocalStyleComponent.migration.field}
            type="password"
            bind:value={password}
            disabled={connecting}
            autocomplete="current-password"
          />
        </label>
      </div>

      {#if recentConnections.length}
        <section class="mt-5 space-y-2" aria-label="Ligações recentes">
          <h3 class={LocalStyleComponent.migration.label}>Ligações recentes</h3>
          <div class="max-h-36 space-y-2 overflow-y-auto">
            {#each recentConnections as connection (connection.id)}
              <button
                type="button"
                class="flex w-full items-center justify-between gap-3 rounded-lg border border-blender-border bg-blender-base px-3 py-2 text-left transition hover:border-blue-500/60 hover:bg-blender-hover disabled:opacity-50"
                onclick={() => useRecent(connection)}
                disabled={connecting}
              >
                <span class="min-w-0">
                  <span class="block truncate text-xs font-medium text-zinc-200"
                    >{connection.database}</span
                  >
                  <span class="block truncate text-[10px] text-zinc-500"
                    >{connection.driver === "postgres" ? "PostgreSQL" : "MySQL"}
                    · {connection.host}:{connection.port} · {connection.user}</span
                  >
                </span>
                <span class="shrink-0 text-[10px] font-semibold text-blue-400"
                  >Ligar</span
                >
              </button>
            {/each}
          </div>
        </section>
      {/if}

      {#if errorMessage}<p
          class={LocalStyleComponent.migration.error}
          role="alert"
        >
          {errorMessage}
        </p>{/if}

      <div class="mt-5 flex justify-end gap-2">
        <button
          type="button"
          class={LocalStyleComponent.migration.ghostButton}
          onclick={oncancel}
          disabled={connecting}
        >
          Cancelar
        </button>
        <button
          type="submit"
          class={LocalStyleComponent.migration.primaryButton}
          disabled={connecting}
        >
          {connecting ? "A ligar…" : "Ligar e importar"}
        </button>
      </div>
    </form>
  </div>
</div>
