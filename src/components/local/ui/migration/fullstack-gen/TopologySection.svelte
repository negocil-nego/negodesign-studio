<script lang="ts">
  import type { ArchitectureMode } from "../../../data/migration";

  let {
    architectureMode = $bindable<ArchitectureMode>("split"),
  }: { architectureMode?: ArchitectureMode } = $props();
</script>

<!-- 1. Topologia da Aplicação -->
<section class="space-y-3">
  <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-400">1. Topologia da Aplicação</h3>
  <div class="grid gap-3 sm:grid-cols-2">
    <button
      type="button"
      class={`flex flex-col items-start rounded-xl border p-4 text-left transition ${
        architectureMode === "split"
          ? "border-blue-500 bg-blue-500/10 text-zinc-100"
          : "border-blender-border bg-blender-base text-zinc-400 hover:border-zinc-600"
      }`}
      onclick={() => (architectureMode = "split")}
    >
      <span class="text-sm font-semibold text-zinc-100">Dividir em Duas Aplicações (Frontend + Backend)</span>
      <span class="mt-1 text-xs text-zinc-400">Gera o backend (Spring Boot, NestJS, Laravel ou ASP.NET Core) no padrão `feature` (controller → service → repository) e integra com o projecto frontend.</span>
    </button>

    <button
      type="button"
      class={`flex flex-col items-start rounded-xl border p-4 text-left transition ${
        architectureMode === "monolith"
          ? "border-blue-500 bg-blue-500/10 text-zinc-100"
          : "border-blender-border bg-blender-base text-zinc-400 hover:border-zinc-600"
      }`}
      onclick={() => (architectureMode = "monolith")}
    >
      <span class="text-sm font-semibold text-zinc-100">Tudo numa Única Aplicação (Fullstack Frontend + API Direta)</span>
      <span class="mt-1 text-xs text-zinc-400">Utiliza o próprio SvelteKit, Nuxt ou Next.js como backend criando rotas de API com ligação direta a PostgreSQL, MySQL ou SQLite.</span>
    </button>
  </div>
</section>
