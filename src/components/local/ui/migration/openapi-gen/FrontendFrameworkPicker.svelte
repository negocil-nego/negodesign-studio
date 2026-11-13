<script lang="ts">
  import type { FrontendFramework } from "../../../data/migration";

  let { frontendFramework = $bindable() }: { frontendFramework?: FrontendFramework } =
    $props();

  const options: { value: FrontendFramework; title: string; description: string; logo: string }[] = [
    {
      value: "sveltekit",
      title: "SvelteKit",
      description: "Svelte 5 Runes ($state, $derived), rotas de admin em src/routes/(admin)",
      logo: "/langs/svelte.svg",
    },
    {
      value: "nuxt",
      title: "Nuxt (Vue 3)",
      description: "Vue 3 script setup, useFetch nativo / composables em pages/admin",
      logo: "/langs/nuxt-js.svg",
    },
    {
      value: "nextjs",
      title: "Next.js (React 19)",
      description: 'App Router, componentes "use client" em app/admin/[entity]',
      logo: "/langs/nextjs-light.svg",
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
        class="flex cursor-pointer flex-col gap-2 rounded-xl border p-3 transition {frontendFramework ===
        opt.value
          ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
          : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
      >
        <div class="flex items-center gap-2">
          <img src={opt.logo} alt={opt.title} class="size-6 shrink-0 object-contain" />
          <span class="font-semibold text-xs text-zinc-200">{opt.title}</span>
          <input
            type="radio"
            name="frontendFramework"
            value={opt.value}
            bind:group={frontendFramework}
            class="ml-auto text-blue-500"
          />
        </div>
        <span class="text-[10px] text-zinc-400">{opt.description}</span>
      </label>
    {/each}
  </div>
</section>
