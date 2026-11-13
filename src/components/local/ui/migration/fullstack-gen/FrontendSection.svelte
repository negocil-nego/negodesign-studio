<script lang="ts">
  import { LocalStyleComponent } from "../../../data/style.component";
  import type { ApiCallPattern, ArchitectureMode, FrontendFramework, HttpClientKind } from "../../../data/migration";
  import { nativeHttpClientLabel, tanstackPackageLabel } from "./options";
  import ProjectPathField from "./ProjectPathField.svelte";

  let {
    architectureMode = "split",
    frontendFramework = $bindable<FrontendFramework>("sveltekit"),
    frontendPath = $bindable(""),
    apiCallPattern = $bindable<ApiCallPattern>("bff"),
    httpClient = $bindable<HttpClientKind>("native"),
    useTanstackQuery = $bindable(true),
  }: { architectureMode?: ArchitectureMode; frontendFramework?: FrontendFramework; frontendPath?: string; apiCallPattern?: ApiCallPattern; httpClient?: HttpClientKind; useTanstackQuery?: boolean } = $props();

  const tanstackLabel = $derived(tanstackPackageLabel(frontendFramework));
  const nativeLabel = $derived(nativeHttpClientLabel(frontendFramework));

  const frontendLogos: Record<FrontendFramework, string> = {
    sveltekit: "/langs/svelte.svg",
    nuxt: "/langs/nuxt-js.svg",
    nextjs: "/langs/nextjs-light.svg",
  };
</script>

<!-- 3. Aplicação Frontend -->
<section class="space-y-4 rounded-xl border border-blender-border bg-blender-base/50 p-4">
  <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">3. Configuração do Projecto Frontend (`AdminCrud`)</h3>

  <div class="grid gap-4 sm:grid-cols-3">
    {#each [
      { id: "sveltekit", label: "SvelteKit", desc: "Rotas +page.svelte & AdminCrud.svelte" },
      { id: "nuxt", label: "Nuxt", desc: "Páginas Vue 3, useFetch & AdminCrud.vue" },
      { id: "nextjs", label: "Next.js", desc: "App Router React & AdminCrud.tsx" },
    ] as fw (fw.id)}
      <button type="button" class={`flex items-center gap-3 rounded-lg border p-3 text-left transition ${frontendFramework === fw.id ? "border-blue-500 bg-blue-500/15 text-zinc-100" : "border-blender-border bg-blender-card text-zinc-400 hover:border-zinc-600"}`} onclick={() => (frontendFramework = fw.id as FrontendFramework)}>
        <img src={frontendLogos[fw.id as FrontendFramework]} alt={fw.label} class="size-7 shrink-0 object-contain" />
        <div class="min-w-0">
          <span class="block truncate text-xs font-semibold text-zinc-100">{fw.label}</span>
          <span class="mt-0.5 block text-[11px] text-zinc-400">{fw.desc}</span>
        </div>
      </button>
    {/each}
  </div>

  <div class="grid gap-4 sm:grid-cols-2">
    <div class="block sm:col-span-2">
      <span class={LocalStyleComponent.migration.label}>Caminho da Aplicação Frontend</span>
      <div class="mt-1">
        <ProjectPathField
          bind:value={frontendPath}
          target="frontend"
          placeholder="/caminho/para/projecto-frontend"
          showCreate
          {frontendFramework}
          {apiCallPattern}
          {httpClient}
          {useTanstackQuery}
          required
        />
      </div>
    </div>

    {#if architectureMode === "split"}
      <div>
        <span class={LocalStyleComponent.migration.label}>Padrão de Comunicação com o Backend</span>
        <div class="mt-1 grid grid-cols-2 gap-2">
          <button type="button" class={`rounded-lg border px-3 py-2 text-xs font-medium transition ${apiCallPattern === "bff" ? "border-blue-500 bg-blue-500/15 text-blue-300" : "border-blender-border bg-blender-card text-zinc-400"}`} onclick={() => (apiCallPattern = "bff")}>Padrão BFF (Server Routes)</button>
          <button type="button" class={`rounded-lg border px-3 py-2 text-xs font-medium transition ${apiCallPattern === "direct" ? "border-blue-500 bg-blue-500/15 text-blue-300" : "border-blender-border bg-blender-card text-zinc-400"}`} onclick={() => (apiCallPattern = "direct")}>Chamar Backend Diretamente</button>
        </div>
      </div>
    {/if}

    <div>
      <span class={LocalStyleComponent.migration.label}>Cliente HTTP de Requisição</span>
      <div class="mt-1 grid grid-cols-2 gap-2">
        <button type="button" class={`rounded-lg border px-3 py-2 text-xs font-medium transition ${httpClient === "native" ? "border-blue-500 bg-blue-500/15 text-blue-300" : "border-blender-border bg-blender-card text-zinc-400"}`} onclick={() => (httpClient = "native")}>{nativeLabel}</button>
        <button type="button" class={`rounded-lg border px-3 py-2 text-xs font-medium transition ${httpClient === "axios" ? "border-blue-500 bg-blue-500/15 text-blue-300" : "border-blender-border bg-blender-card text-zinc-400"}`} onclick={() => (httpClient = "axios")}>Axios</button>
      </div>
    </div>

    <div class="flex items-center justify-between rounded-lg border border-blender-border bg-blender-card px-4 py-2.5 sm:col-span-2">
      <div>
        <span class="block text-xs font-semibold text-zinc-200">Integrar TanStack Query ({tanstackLabel})</span>
        <span class="block text-[11px] text-zinc-400">Gera hooks reactivos de query e mutação (findAll, insert, update, delete) com invalidação de cache.</span>
      </div>
      <input type="checkbox" bind:checked={useTanstackQuery} class="size-4 rounded border-blender-border text-blue-600" />
    </div>
  </div>
</section>
