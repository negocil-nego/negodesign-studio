<script lang="ts">
  import type { FrontendFramework, HttpClientKind } from "../../../data/migration";

  let {
    frontendFramework,
    httpClient = $bindable("native"),
    useTanstackQuery = $bindable(true),
  }: {
    frontendFramework: FrontendFramework;
    httpClient?: HttpClientKind;
    useTanstackQuery?: boolean;
  } = $props();

  const kinds: HttpClientKind[] = ["native", "axios"];

  const tanstackPackage = $derived(
    frontendFramework === "nuxt"
      ? "@tanstack/vue-query"
      : frontendFramework === "sveltekit"
        ? "@tanstack/svelte-query"
        : "@tanstack/react-query",
  );
</script>

<section class="space-y-3">
  <h3 class="font-semibold text-zinc-800 dark:text-zinc-100">
    6. Cliente de Requisição HTTP
  </h3>
  <div class="grid grid-cols-2 gap-3">
    {#each kinds as kind (kind)}
      <label
        class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {httpClient ===
        kind
          ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
          : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
      >
        <div class="flex items-center justify-between">
          <span class="font-semibold text-xs text-zinc-200">
            {#if kind === "native"}
              Nativo do Framework ({frontendFramework === "nuxt"
                ? "useFetch / $fetch"
                : "Fetch API nativa"})
            {:else}
              Axios
            {/if}
          </span>
          <input
            type="radio"
            name="httpClient"
            value={kind}
            bind:group={httpClient}
            class="text-blue-500"
          />
        </div>
        <span class="text-[10px] text-zinc-400">
          {#if kind === "native"}
            {frontendFramework === "nuxt"
              ? "Utiliza o useFetch() e $fetch() nativos do ecossistema Nuxt / Nitro"
              : "Utiliza a Fetch API padrão sem dependências externas adicionais"}
          {:else}
            Instância configurada com axios.create(), interceptors e métodos tipados
          {/if}
        </span>
      </label>
    {/each}
  </div>
</section>

<section class="space-y-2">
  <div class="flex items-center justify-between rounded-xl border border-blender-border bg-blender-base p-3">
    <div>
      <span class="block font-semibold text-xs text-zinc-200">
        7. Integrar TanStack Query ({tanstackPackage})
      </span>
      <span class="text-[10px] text-zinc-400">
        Gera hooks e composables com cache reactivo, invalidação automática em mutations e paginação
      </span>
    </div>
    <input
      type="checkbox"
      bind:checked={useTanstackQuery}
      class="h-5 w-5 rounded border-blender-border text-blue-500 focus:ring-0"
    />
  </div>
</section>
