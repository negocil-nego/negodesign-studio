<script lang="ts">
  import type { FrontendFramework } from "../../../data/migration";

  let { frontendFramework = $bindable() }: { frontendFramework?: FrontendFramework } =
    $props();

  const options: { value: FrontendFramework; title: string; description: string }[] = [
    {
      value: "sveltekit",
      title: "SvelteKit",
      description: "Svelte 5 Runes ($state, $derived), rotas de admin em src/routes/(admin)",
    },
    {
      value: "nuxt",
      title: "Nuxt (Vue 3)",
      description: "Vue 3 script setup, useFetch nativo / composables em pages/admin",
    },
    {
      value: "nextjs",
      title: "Next.js (React 19)",
      description: 'App Router, componentes "use client" em app/admin/[entity]',
    },
  ];
</script>

<section class="space-y-3">
  <h3 class="font-semibold text-zinc-800 dark:text-zinc-100">
    2. Framework Frontend de Destino
  </h3>
  <div class="grid grid-cols-3 gap-3">
    {#each options as opt (opt.value)}
      <label
        class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {frontendFramework ===
        opt.value
          ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
          : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
      >
        <div class="flex items-center justify-between">
          <span class="font-semibold text-xs text-zinc-200">{opt.title}</span>
          <input
            type="radio"
            name="frontendFramework"
            value={opt.value}
            bind:group={frontendFramework}
            class="text-blue-500"
          />
        </div>
        <span class="text-[10px] text-zinc-400">{opt.description}</span>
      </label>
    {/each}
  </div>
</section>
