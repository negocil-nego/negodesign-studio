<script lang="ts">
  import type { Snippet } from "svelte";

  type Language = "svelte" | "react" | "vue" | "flutter";
  type TerminalProps = {
    preview?: Snippet;
    svelte?: Snippet;
    react?: Snippet;
    vue?: Snippet;
    flutter?: Snippet;
  };

  let { preview, svelte, react, vue, flutter }: TerminalProps = $props();
  let activeTab = $state<"preview" | Language>("preview");

  const languageLabels: Record<Language, string> = {
    svelte: "Svelte",
    react: "React",
    vue: "Vue",
    flutter: "Flutter",
  };
  const languageLogos: Record<Language, string> = {
    svelte: "/langs/svelte.svg",
    react: "/langs/react.svg",
    vue: "/langs/vue-js.svg",
    flutter: "/langs/flutter.svg",
  };
</script>

<section
  class="overflow-hidden rounded-lg border border-blender-border bg-white shadow-sm"
>
  <div class="flex items-center gap-2 border-b border-white/5 px-3 py-2">
    <span class="h-2.5 w-2.5 rounded-full bg-amber-400"></span>
    <span class="h-2.5 w-2.5 rounded-full bg-blue-400"></span>
    <span class="h-2.5 w-2.5 rounded-full bg-red-400"></span>
    <span class="ml-2 truncate text-[10px] text-slate-400">
      {activeTab === "preview" ? "Preview" : languageLabels[activeTab]}
    </span>
    {#if activeTab !== "preview"}
      <img
        src={languageLogos[activeTab]}
        alt=""
        class="ml-auto h-4 w-4 object-contain"
      />
    {/if}
  </div>

  <div class="min-h-25 p-4">
    {#if activeTab === "preview"}
      {#if preview}
        {@render preview()}
      {/if}
    {:else if activeTab === "svelte"}
      {#if svelte}
        {@render svelte()}
      {:else}
        <p class="text-xs text-amber-300">Snippet Svelte indisponível.</p>
      {/if}
    {:else if activeTab === "react"}
      {#if react}
        {@render react()}
      {:else}
        <p class="text-xs text-amber-300">Snippet React indisponível.</p>
      {/if}
    {:else if activeTab === "vue"}
      {#if vue}
        {@render vue()}
      {:else}
        <p class="text-xs text-amber-300">Snippet Vue indisponível.</p>
      {/if}
    {:else}
      {#if flutter}
        {@render flutter()}
      {:else}
        <p class="text-xs text-amber-300">Snippet Flutter indisponível.</p>
      {/if}
    {/if}
  </div>

  <nav
    class="flex flex-wrap gap-1 border-t border-white/5 p-2"
    aria-label="Preview e código-fonte"
  >
    <button
      type="button"
      aria-pressed={activeTab === "preview"}
      onclick={() => (activeTab = "preview")}
      class="rounded px-2 py-1 text-[10px] {activeTab === 'preview'
        ? 'bg-amber-700 text-white'
        : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'}">Preview</button
    >
    <button
      type="button"
      aria-pressed={activeTab === "svelte"}
      onclick={() => (activeTab = "svelte")}
      class="inline-flex items-center gap-1.5 rounded px-2 py-1 text-[10px] {activeTab ===
      'svelte'
        ? 'bg-amber-700 text-white'
        : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'}"
      ><img
        src={languageLogos.svelte}
        alt=""
        class="h-4 w-4 object-contain"
      />Svelte</button
    >
    <button
      type="button"
      aria-pressed={activeTab === "react"}
      onclick={() => (activeTab = "react")}
      class="inline-flex items-center gap-1.5 rounded px-2 py-1 text-[10px] {activeTab ===
      'react'
        ? 'bg-amber-700 text-white'
        : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'}"
      ><img
        src={languageLogos.react}
        alt=""
        class="h-4 w-4 object-contain"
      />React</button
    >
    <button
      type="button"
      aria-pressed={activeTab === "vue"}
      onclick={() => (activeTab = "vue")}
      class="inline-flex items-center gap-1.5 rounded px-2 py-1 text-[10px] {activeTab ===
      'vue'
        ? 'bg-amber-700 text-white'
        : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'}"
      ><img
        src={languageLogos.vue}
        alt=""
        class="h-4 w-4 object-contain"
      />Vue</button
    >
    <button
      type="button"
      aria-pressed={activeTab === "flutter"}
      onclick={() => (activeTab = "flutter")}
      class="inline-flex items-center gap-1.5 rounded px-2 py-1 text-[10px] {activeTab ===
      'flutter'
        ? 'bg-amber-700 text-white'
        : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'}"
      ><img
        src={languageLogos.flutter}
        alt=""
        class="h-4 w-4 object-contain"
      />Flutter</button
    >
  </nav>
</section>
