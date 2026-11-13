<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import type { ApiCallPattern } from "../../../data/migration";

  let {
    frontendPath = $bindable(""),
    apiCallPattern = $bindable("bff"),
    backendBaseUrl = $bindable(""),
    onerror,
  }: {
    frontendPath?: string;
    apiCallPattern?: ApiCallPattern;
    backendBaseUrl?: string;
    onerror: (message: string) => void;
  } = $props();

  const patterns: ApiCallPattern[] = ["bff", "direct"];
  const patternInfo: Record<ApiCallPattern, { title: string; description: string }> = {
    bff: {
      title: "BFF (Backend-for-Frontend)",
      description:
        "Cria rotas de proxy no próprio frontend (+server.ts, server/api, route.ts) protegendo chaves e centralizando chamadas",
    },
    direct: {
      title: "Chamada Direta ao Backend",
      description: "O cliente no browser chama a URL base do backend de forma direta via CORS",
    },
  };

  async function chooseFrontendDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Seleccionar pasta da aplicação frontend",
      });
      if (selected && typeof selected === "string") {
        frontendPath = selected;
      }
    } catch (err) {
      onerror(err instanceof Error ? err.message : String(err));
    }
  }
</script>

<section class="space-y-2">
  <label for="frontendPath" class="block font-semibold text-zinc-800 dark:text-zinc-100">
    3. Pasta do Projecto Frontend
  </label>
  <div class="flex gap-2">
    <input
      id="frontendPath"
      type="text"
      bind:value={frontendPath}
      placeholder="/caminho/para/projecto-frontend"
      class="flex-1 rounded-lg border border-blender-border bg-blender-base px-3 py-2 text-xs text-zinc-200 placeholder-zinc-500 focus:border-blue-500 focus:outline-none"
    />
    <button type="button" onclick={chooseFrontendDirectory} class="rounded-lg border border-blender-border bg-blender-base px-3 py-2 text-xs font-medium text-zinc-300 hover:bg-blender-base/80 transition">Procurar...</button>
  </div>
</section>

<section class="space-y-3">
  <h3 class="font-semibold text-zinc-800 dark:text-zinc-100">
    4. Padrão de Comunicação (BFF vs Direct)
  </h3>
  <div class="grid grid-cols-2 gap-3">
    {#each patterns as pattern (pattern)}
      <label
        class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {apiCallPattern ===
        pattern
          ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
          : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
      >
        <div class="flex items-center justify-between">
          <span class="font-semibold text-xs text-zinc-200">{patternInfo[pattern].title}</span>
          <input type="radio" name="apiCallPattern" value={pattern} bind:group={apiCallPattern} class="text-blue-500" />
        </div>
        <span class="text-[10px] text-zinc-400">{patternInfo[pattern].description}</span>
      </label>
    {/each}
  </div>
</section>

<section class="space-y-2">
  <label for="backendBaseUrl" class="block font-semibold text-zinc-800 dark:text-zinc-100">
    5. URL Base do Backend
  </label>
  <input
    id="backendBaseUrl"
    type="text"
    bind:value={backendBaseUrl}
    placeholder="http://localhost:8080 ou https://api.exemplo.com"
    class="w-full rounded-lg border border-blender-border bg-blender-base px-3 py-2 text-xs text-zinc-200 placeholder-zinc-500 focus:border-blue-500 focus:outline-none"
  />
</section>
