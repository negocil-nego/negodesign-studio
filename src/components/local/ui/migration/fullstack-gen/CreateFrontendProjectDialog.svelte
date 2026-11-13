<script lang="ts">
  import {
    createSveltekitProject,
    createNuxtProject,
    createNextjsProject,
    type FrontendProjectResult,
  } from "./paths";
  import { LocalStyleComponent } from "../../../data/style.component";
  import type {
    ApiCallPattern,
    FrontendFramework,
    HttpClientKind,
  } from "../../../data/migration";

  let {
    targetDir,
    framework,
    apiCallPattern = "bff",
    httpClient = "native",
    useTanstackQuery = true,
    onclose,
    oncreated,
  }: {
    targetDir: string;
    framework: FrontendFramework;
    apiCallPattern?: ApiCallPattern;
    httpClient?: HttpClientKind;
    useTanstackQuery?: boolean;
    onclose: () => void;
    oncreated: (projectDir: string) => void;
  } = $props();

  const isSvelte = $derived(framework === "sveltekit");
  const isNuxt = $derived(framework === "nuxt");
  const isNext = $derived(framework === "nextjs");

  const fwLabel = $derived(
    isSvelte ? "SvelteKit" : isNuxt ? "Nuxt" : "Next.js",
  );
  const fwDesc = $derived(
    isSvelte ? "Vite + Svelte 5" : isNuxt ? "Nuxt 3 + Vue 3" : "Next.js 15 + React 19",
  );

  let name = $state("negodesign-app");
  let description = $state("Aplicação frontend gerada pelo Negodesign");
  let includeTailwind = $state(true);
  // svelte-ignore state_referenced_locally
  let includeTanstack = $state(Boolean(useTanstackQuery));
  let creating = $state(false);
  let error = $state<string | null>(null);
  let result = $state<FrontendProjectResult | null>(null);

  const patternLabel = $derived(
    apiCallPattern === "bff" ? "Padrão BFF (Server Routes)" : "Chamar Backend Diretamente",
  );
  const httpLabel = $derived(
    httpClient === "axios" ? "Axios" : "Fetch API (Nativo Web / SSR)",
  );
  const tanstackPkg = $derived(
    isSvelte
      ? "@tanstack/svelte-query"
      : isNuxt
        ? "@tanstack/vue-query"
        : "@tanstack/react-query",
  );

  const deps = $derived([
    {
      name: isSvelte
        ? "@sveltejs/kit + svelte 5"
        : isNuxt
          ? "nuxt 3 + vue 3"
          : "next 15 + react 19",
      desc: `Estrutura de páginas e rotas com ${fwLabel}.`,
      required: true,
    },
    {
      name: "Tailwind CSS v4",
      desc: "Estilização utilitária e tema NegoDesign.",
      required: true,
    },
    {
      name: "TypeScript",
      desc: "Tipagem estática e suporte para DTOs e entidades.",
      required: true,
    },
    ...(httpClient === "axios"
      ? [
          {
            name: "axios",
            desc: "Cliente HTTP baseado em Promises.",
            required: true,
          },
        ]
      : []),
    {
      name: tanstackPkg,
      desc: "Gestão de estado assíncrono e cache reactiva para AdminCrud.",
      required: false,
    },
  ]);

  async function handleCreate() {
    error = null;
    creating = true;
    try {
      const input = {
        targetDir,
        name: name.trim(),
        description: description.trim(),
        framework,
        apiCallPattern,
        httpClient,
        useTanstackQuery: includeTanstack,
        includeTailwind,
      };

      if (isSvelte) {
        result = await createSveltekitProject(input);
      } else if (isNuxt) {
        result = await createNuxtProject(input);
      } else {
        result = await createNextjsProject(input);
      }
      oncreated(result.projectDir);
    } catch (e) {
      error = String(e);
    } finally {
      creating = false;
    }
  }

  const canCreate = $derived(name.trim().length > 0);
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
  role="presentation"
  onclick={onclose}
  onkeydown={(e) => e.key === "Escape" && onclose()}
>
  <div
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-2xl border border-blender-border bg-blender-card p-6 shadow-2xl"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    <div class="flex items-start justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-zinc-100">Criar Projecto {fwLabel}</h2>
        <p class="mt-1 text-sm text-zinc-400">Vai ser criado um projecto {fwDesc} pronto para receber os módulos AdminCrud.</p>
      </div>
      <button class="text-zinc-400 hover:text-zinc-200" onclick={onclose}>✕</button>
    </div>

    {#if result}
      <div class="mt-5 space-y-4">
        <div class="rounded-lg border border-emerald-500/40 bg-emerald-500/10 p-4">
          <p class="font-semibold text-emerald-300">Projecto {result.framework} criado com sucesso!</p>
          <p class="mt-1 text-sm text-zinc-300">{result.projectDir}</p>
          <p class="mt-2 text-xs text-zinc-400">{result.generatedFiles} ficheiros gerados · {result.buildTool}</p>
        </div>
        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Dependências incluídas</h3>
          <ul class="mt-2 space-y-1 text-sm text-zinc-300">
            {#each result.dependencies as dep}
              <li class="font-mono text-xs">• {dep}</li>
            {/each}
          </ul>
        </div>
        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Próximos passos</h3>
          <ol class="mt-2 list-decimal space-y-1 pl-5 text-sm text-zinc-300">
            {#each result.nextSteps as step}
              <li class="font-mono text-xs">{step}</li>
            {/each}
          </ol>
        </div>
        <div class="flex justify-end gap-2">
          <button class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white hover:bg-blue-500" onclick={onclose}>
            Usar este caminho
          </button>
        </div>
      </div>
    {:else}
      <div class="mt-5 space-y-5">
        <div class="grid gap-4 sm:grid-cols-2">
          <label class="block">
            <span class={LocalStyleComponent.migration.label}>Nome do projecto</span>
            <input class={LocalStyleComponent.migration.field} bind:value={name} placeholder="negodesign-app" />
          </label>
          <label class="block">
            <span class={LocalStyleComponent.migration.label}>Descrição</span>
            <input class={LocalStyleComponent.migration.field} bind:value={description} placeholder="Aplicação frontend" />
          </label>
        </div>

        <div class="rounded-lg border border-blender-border bg-blender-base/50 p-4">
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Configuração detectada</h3>
          <div class="mt-3 grid gap-2 text-sm">
            <div class="flex justify-between">
              <span class="text-zinc-400">Framework</span>
              <span class="font-medium text-zinc-200">{fwLabel} ({fwDesc})</span>
            </div>
            <div class="flex justify-between">
              <span class="text-zinc-400">Comunicação</span>
              <span class="font-medium text-zinc-200">{patternLabel}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-zinc-400">Cliente HTTP</span>
              <span class="font-medium text-zinc-200">{httpLabel}</span>
            </div>
          </div>
        </div>

        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Dependências que serão incluídas</h3>
          <div class="mt-3 space-y-2">
            {#each deps as dep}
              <label class="flex items-start gap-3 rounded-lg border border-blender-border bg-blender-base/50 p-3">
                {#if dep.required}
                  <input type="checkbox" checked disabled class="mt-0.5 accent-blue-500" />
                {:else}
                  <input type="checkbox" bind:checked={includeTanstack} class="mt-0.5 accent-blue-500" />
                {/if}
                <div class="min-w-0 flex-1">
                  <div class="flex items-center gap-2">
                    <span class="font-mono text-xs font-semibold text-zinc-200">{dep.name}</span>
                    {#if dep.required}
                      <span class="rounded bg-blue-500/20 px-1.5 py-0.5 text-[9px] font-semibold text-blue-300">OBRIGATÓRIO</span>
                    {/if}
                  </div>
                  <p class="mt-1 text-xs text-zinc-400">{dep.desc}</p>
                </div>
              </label>
            {/each}
          </div>
        </div>

        {#if error}
          <div class="rounded-lg border border-red-500/40 bg-red-500/10 p-3 text-sm text-red-300">{error}</div>
        {/if}

        <div class="flex justify-end gap-2">
          <button class="rounded-lg border border-blender-border px-4 py-2 text-sm font-medium text-zinc-300 hover:bg-blender-hover" onclick={onclose}>Cancelar</button>
          <button
            class="rounded-lg bg-emerald-600 px-4 py-2 text-sm font-semibold text-white hover:bg-emerald-500 disabled:opacity-50"
            disabled={creating || !canCreate}
            onclick={handleCreate}
          >
            {creating ? "A criar..." : "Criar projecto"}
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>

