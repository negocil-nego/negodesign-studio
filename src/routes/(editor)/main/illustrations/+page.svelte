<script lang="ts">
  import Icon from "../../../../project/ui/atoms/icon/ui/Icon.svelte";

  let downloading = $state(false);
  let statusMessage = $state<string | null>(null);

  async function handleDownloadGallery() {
    downloading = true;
    statusMessage = null;
    try {
      // Simula / inicia o pedido de transferência do pacote padrão da galeria
      await new Promise((resolve) => setTimeout(resolve, 600));
      statusMessage =
        "O pacote padrão da galeria de ilustrações foi preparado para transferência.";
    } finally {
      downloading = false;
    }
  }
</script>

<svelte:head>
  <title>Galeria de ilustrações · Negodesign</title>
  <meta
    name="description"
    content="Transfira o pacote padrão da galeria de ilustrações."
  />
</svelte:head>

<div
  class="flex min-h-[calc(100vh-7rem)] w-full items-center justify-center px-6 py-12"
>
  <div
    class="flex w-full max-w-md flex-col items-center rounded-2xl border border-blender-border bg-blender-card p-8 text-center shadow-sm"
  >
    <div
      class="mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-blue-500/10 text-blue-500"
    >
      <Icon name="download-04" />
    </div>

    <h1
      class="text-xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100"
    >
      Galeria de ilustrações
    </h1>

    <p class="mt-2 text-sm leading-relaxed text-zinc-500 dark:text-zinc-400">
      Os recursos visuais da galeria não vêm incluídos por omissão para manter a
      aplicação leve. Transfira a pasta padrão da galeria para utilizar as
      ilustrações localmente.
    </p>

    <button
      type="button"
      disabled={downloading}
      onclick={handleDownloadGallery}
      class="mt-6 inline-flex items-center justify-center gap-2 rounded-xl bg-blue-600 px-5 py-2.5 text-sm font-medium text-white shadow-sm transition hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-60"
    >
      <Icon name="download-04" />
      <span
        >{downloading
          ? "A transferir..."
          : "Baixar pasta padrão da galeria"}</span
      >
    </button>

    {#if statusMessage}
      <p class="mt-4 text-xs font-medium text-emerald-500">
        {statusMessage}
      </p>
    {/if}
  </div>
</div>
