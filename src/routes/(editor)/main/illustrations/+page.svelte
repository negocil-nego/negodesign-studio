<script lang="ts">
  import illustrations from "../../../../project/data/illustration-gallery.json";

  type Illustration = { category: string; name: string; src: string };
  const items = illustrations as Illustration[];
  const categories = [...new Set(items.map((item) => item.category))];

  let selectedCategory = $state("Todas");
  let search = $state("");

  const filteredItems = $derived.by(() => {
    const query = search.trim().toLowerCase();
    return items.filter((item) => {
      const matchesCategory =
        selectedCategory === "Todas" || item.category === selectedCategory;
      const matchesSearch =
        !query || `${item.name} ${item.category}`.toLowerCase().includes(query);
      return matchesCategory && matchesSearch;
    });
  });

  function displayName(value: string) {
    return value
      .replace(/[-_]+/g, " ")
      .replace(/\s+/g, " ")
      .trim()
      .replace(/\b\w/g, (letter) => letter.toUpperCase());
  }
</script>

<svelte:head>
  <title>Galeria de ilustrações · Negodesign</title>
  <meta
    name="description"
    content="Explore as ilustrações organizadas por coleção."
  />
</svelte:head>

<div class="mx-auto flex w-full max-w-7xl flex-col gap-6 px-5 py-8 sm:px-8">
  <header class="flex flex-wrap items-end justify-between gap-4">
    <div>
      <p
        class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-500"
      >
        Biblioteca visual
      </p>
      <h1
        class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100"
      >
        Galeria de ilustrações
      </h1>
      <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
        {items.length} imagens em {categories.length} coleções.
      </p>
    </div>
    <label class="w-full sm:w-72">
      <span class="sr-only">Pesquisar ilustrações</span>
      <input
        bind:value={search}
        type="search"
        placeholder="Pesquisar imagens..."
        class="w-full rounded-lg border border-blender-border bg-blender-card px-3.5 py-2.5 text-sm text-zinc-800 outline-none placeholder:text-zinc-400 focus:border-blue-500 dark:text-zinc-100"
      />
    </label>
  </header>

  <nav class="flex flex-wrap gap-2" aria-label="Filtrar por coleção">
    {#each ["Todas", ...categories] as category (category)}
      <button
        type="button"
        aria-pressed={selectedCategory === category}
        onclick={() => (selectedCategory = category)}
        class={`rounded-full border px-3 py-1.5 text-xs font-medium transition ${
          selectedCategory === category
            ? "border-blue-500/50 bg-blue-500/10 text-blue-600 dark:text-blue-300"
            : "border-blender-border bg-blender-card text-zinc-600 hover:border-blue-500/50 dark:text-zinc-300"
        }`}
      >
        {displayName(category)}
        <span class="ml-1.5 opacity-60">
          {category === "Todas"
            ? items.length
            : items.filter((item) => item.category === category).length}
        </span>
      </button>
    {/each}
  </nav>

  {#if filteredItems.length}
    <div
      class="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4 xl:grid-cols-6"
    >
      {#each filteredItems as item (item.src)}
        <article
          class="group overflow-hidden rounded-xl border border-blender-border bg-blender-card transition hover:-translate-y-0.5 hover:border-blue-500/50 hover:shadow-lg"
        >
          <div
            class="flex aspect-square items-center justify-center bg-white p-4 dark:bg-zinc-100"
          >
            <img
              src={item.src}
              alt={displayName(item.name)}
              loading="lazy"
              decoding="async"
              class="h-full w-full object-contain transition-transform duration-200 group-hover:scale-105"
            />
          </div>
          <div class="min-w-0 px-3 py-2.5">
            <h2
              class="truncate text-xs font-medium text-zinc-700 dark:text-zinc-200"
              title={displayName(item.name)}
            >
              {displayName(item.name)}
            </h2>
            <p
              class="mt-1 truncate text-[10px] text-zinc-500"
              title={displayName(item.category)}
            >
              {displayName(item.category)}
            </p>
          </div>
        </article>
      {/each}
    </div>
  {:else}
    <div
      class="rounded-xl border border-dashed border-blender-border px-6 py-16 text-center text-sm text-zinc-500"
    >
      Nenhuma imagem encontrada para esta pesquisa.
    </div>
  {/if}
</div>
