<script lang="ts">
  import type { Snippet } from "svelte";
  import { activeProject } from "../../../data/projects";
  import { copyComponentToActiveProject } from "../../../data/component-copy";

  type Language = "svelte" | "react" | "vue" | "flutter";
  type TerminalProps = {
    id: string;
    preview?: Snippet;
    svelte?: Snippet;
    react?: Snippet;
    vue?: Snippet;
    flutter?: Snippet;
  };

  let { id, preview, svelte, react, vue, flutter }: TerminalProps = $props();
  let activeTab = $state<"preview" | Language>("preview");
  let copyMessage = $state("");
  let copying = $state(false);

  const componentInfo: Record<
    string,
    { name: string; description: string; componentId: string }
  > = {
    "input-text": {
      name: "InputText",
      description: "Campo de texto para inserir uma linha de conteúdo, com suporte a rótulo e propriedades nativas do input.",
      componentId: "input-text",
    },
    "input-email": {
      name: "InputEmail",
      description: "Campo de email com validação nativa do browser, indicação de formato e propriedades configuráveis.",
      componentId: "input-email",
    },
    "input-phone": {
      name: "InputPhone",
      description: "Campo de telefone preparado para números e atributos nativos de entrada telefónica.",
      componentId: "input-phone",
    },
    "input-phone-flag": {
      name: "InputPhoneFlag",
      description: "Campo telefónico com selector de país e indicativo internacional integrado.",
      componentId: "input-phone-flag",
    },
    "input-password": {
      name: "InputPassword",
      description: "Campo protegido para palavras-passe, com propriedades nativas de validação e preenchimento.",
      componentId: "input-password",
    },
    "input-date": {
      name: "InputDate",
      description: "Campo de data que utiliza o selector de calendário fornecido pelo browser.",
      componentId: "input-date",
    },
    "input-time": {
      name: "InputTime",
      description: "Campo de hora com entrada e selector nativos do browser.",
      componentId: "input-time",
    },
    "input-number": {
      name: "InputNumber",
      description: "Campo numérico para inserir valores com as opções de validação do elemento input.",
      componentId: "input-number",
    },
    "input-age": {
      name: "InputAge",
      description: "Campo numérico especializado em idade, com rótulo e limites definidos pelo consumidor.",
      componentId: "input-age",
    },
  };

  const info = componentInfo[id];

  async function copyToProject() {
    if (!info) return;
    copying = true;
    copyMessage = "";
    try {
      const result = await copyComponentToActiveProject(info.componentId);
      copyMessage = `${result.copiedFiles} ficheiro(s) e ${result.exportedComponents} componente(s) disponíveis em ${result.project.name}/negodesign.`;
    } catch (error) {
      copyMessage = error instanceof Error ? error.message : String(error);
    } finally {
      copying = false;
    }
  }

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
  {id}
  class="overflow-hidden rounded-lg border border-blender-border bg-blender-card text-zinc-300 shadow-sm"
>
  {#if info}
    <div class="flex flex-wrap items-start justify-between gap-3 border-b border-blender-border px-4 py-3">
      <div class="min-w-0">
        <h2 class="text-sm font-semibold text-zinc-100">{info.name}</h2>
        <p class="mt-1 max-w-2xl text-xs leading-5 text-zinc-400">{info.description}</p>
        {#if copyMessage}<p class="mt-1 text-[11px] text-blue-400" role="status">{copyMessage}</p>{/if}
      </div>
      <button
        type="button"
        class="shrink-0 rounded bg-blue-600 px-3 py-1.5 text-[11px] font-medium text-white transition-colors hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-40"
        onclick={copyToProject}
        disabled={!$activeProject || $activeProject.language === "Unknown" || copying}
        title={!$activeProject
          ? "Seleccione um projecto activo"
          : $activeProject.language === "Unknown"
            ? "A linguagem do projecto não foi identificada"
            : `Copiar para ${$activeProject.name}`}
      >{copying ? "A copiar…" : "Copiar para projecto"}</button>
    </div>
  {/if}
  <div class="flex items-center gap-2 border-b border-blender-border px-3 py-2">
    <span class="h-2.5 w-2.5 rounded-full bg-amber-400"></span>
    <span class="h-2.5 w-2.5 rounded-full bg-blue-400"></span>
    <span class="h-2.5 w-2.5 rounded-full bg-red-400"></span>
    <span class="ml-2 truncate text-[10px] text-zinc-500">
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
    class="flex flex-wrap gap-1 border-t border-blender-border p-2"
    aria-label="Preview e código-fonte"
  >
    <button
      type="button"
      aria-pressed={activeTab === "preview"}
      onclick={() => (activeTab = "preview")}
      class="rounded px-2 py-1 text-[10px] {activeTab === 'preview'
        ? 'bg-amber-700 text-white'
        : 'text-zinc-500 hover:bg-blender-hover hover:text-zinc-100'}">Preview</button
    >
    <button
      type="button"
      aria-pressed={activeTab === "svelte"}
      onclick={() => (activeTab = "svelte")}
      class="inline-flex items-center gap-1.5 rounded px-2 py-1 text-[10px] {activeTab ===
      'svelte'
        ? 'bg-amber-700 text-white'
        : 'text-zinc-500 hover:bg-blender-hover hover:text-zinc-100'}"
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
        : 'text-zinc-500 hover:bg-blender-hover hover:text-zinc-100'}"
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
        : 'text-zinc-500 hover:bg-blender-hover hover:text-zinc-100'}"
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
        : 'text-zinc-500 hover:bg-blender-hover hover:text-zinc-100'}"
      ><img
        src={languageLogos.flutter}
        alt=""
        class="h-4 w-4 object-contain"
      />Flutter</button
    >
  </nav>
</section>
