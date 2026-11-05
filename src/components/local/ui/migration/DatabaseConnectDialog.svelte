<script lang="ts">
  import { StyleComponent } from "../../data/style.component";
  import { connectDatabase, type DatabaseDriver } from "../../data/migration";

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

  function selectDriver(id: DatabaseDriver) {
    const target = drivers.find((item) => item.id === id);
    if (!target || target.id === driver) return;
    driver = target.id;
    port = target.port;
  }

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    connecting = true;
    errorMessage = "";
    try {
      await connectDatabase({ driver, host, port: Number(port), database, user, password });
      oncomplete();
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      connecting = false;
    }
  }
</script>

<div
  class={StyleComponent.migration.overlay}
  role="presentation"
  onclick={(event) => event.target === event.currentTarget && !connecting && oncancel()}
  onkeydown={(event) => event.key === "Escape" && !connecting && oncancel()}
>
  <div
    class="{StyleComponent.migration.dialog} max-w-md"
    role="dialog"
    aria-modal="true"
    aria-labelledby="db-dialog-title"
  >
    <form onsubmit={submit}>
    <div class="flex items-start justify-between gap-4">
      <div>
        <h2 id="db-dialog-title" class="text-base font-semibold text-zinc-100">Ligar base de dados</h2>
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
          class="{StyleComponent.migration.driverTab} {driver === item.id
            ? StyleComponent.migration.driverTabActive
            : ''}"
          aria-pressed={driver === item.id}
          onclick={() => selectDriver(item.id)}>{item.label}</button
        >
      {/each}
    </fieldset>

    <div class="mt-4 grid gap-3 sm:grid-cols-2">
      <label class="sm:col-span-2">
        <span class={StyleComponent.migration.label}>Host</span>
        <input class={StyleComponent.migration.field} bind:value={host} required disabled={connecting} placeholder="localhost" />
      </label>
      <label>
        <span class={StyleComponent.migration.label}>Porta</span>
        <input class={StyleComponent.migration.field} type="number" bind:value={port} required disabled={connecting} />
      </label>
      <label>
        <span class={StyleComponent.migration.label}>Base de dados</span>
        <input class={StyleComponent.migration.field} bind:value={database} required disabled={connecting} placeholder="nome_da_base" />
      </label>
      <label>
        <span class={StyleComponent.migration.label}>Utilizador</span>
        <input class={StyleComponent.migration.field} bind:value={user} required disabled={connecting} autocomplete="username" />
      </label>
      <label>
        <span class={StyleComponent.migration.label}>Palavra-passe</span>
        <input class={StyleComponent.migration.field} type="password" bind:value={password} disabled={connecting} autocomplete="current-password" />
      </label>
    </div>

    {#if errorMessage}<p class={StyleComponent.migration.error} role="alert">{errorMessage}</p>{/if}

    <div class="mt-5 flex justify-end gap-2">
      <button type="button" class={StyleComponent.migration.ghostButton} onclick={oncancel} disabled={connecting}>
        Cancelar
      </button>
      <button type="submit" class={StyleComponent.migration.primaryButton} disabled={connecting}>
        {connecting ? "A ligar…" : "Ligar e importar"}
      </button>
    </div>
    </form>
  </div>
</div>
