<script lang="ts">
  import { onMount } from "svelte";
  import { listCodegenPaths, deleteCodegenPath, type CodegenPath } from "./paths";
  import { pickProjectFolder } from "./folder";
  import { LocalStyleComponent } from "../../../data/style.component";
  import type { BackendFramework, DatabaseEngine, SpringLanguage } from "../../../data/migration";
  import CreateBackendProjectDialog from "./CreateBackendProjectDialog.svelte";

  let {
    value = $bindable(""),
    target,
    placeholder,
    showCreate = false,
    backendFramework = "spring-boot",
    required = false,
    language = "java",
    ormStrategy = "spring-data-jpa",
    databaseEngine = "postgres",
  }: {
    value?: string;
    target: "frontend" | "backend";
    placeholder?: string;
    showCreate?: boolean;
    backendFramework?: BackendFramework;
    required?: boolean;
    language?: SpringLanguage;
    ormStrategy?: string;
    databaseEngine?: DatabaseEngine;
  } = $props();

  let history = $state<CodegenPath[]>([]);
  let showHistory = $state(false);
  let showCreateDialog = $state(false);
  let createDir = $state("");

  onMount(async () => {
    history = await listCodegenPaths(target);
  });

  async function refreshHistory() {
    history = await listCodegenPaths(target);
  }

  async function handlePick() {
    const selected = await pickProjectFolder(target);
    if (selected) {
      value = selected;
      showHistory = false;
    }
  }

  function selectHistory(p: CodegenPath) {
    value = p.path;
    showHistory = false;
  }

  async function removeHistory(e: Event, id: number) {
    e.stopPropagation();
    await deleteCodegenPath(id);
    await refreshHistory();
  }

  function openCreateDialog() {
    createDir = value || "";
    showCreateDialog = true;
  }

  function onProjectCreated(dir: string) {
    value = dir;
    showCreateDialog = false;
    refreshHistory();
  }
</script>

<div class="relative">
  <div class="flex gap-2">
    <div class="relative flex-1">
      <input
        id={`path-input-${target}`}
        class={LocalStyleComponent.migration.field}
        bind:value
        {placeholder}
        {required}
        onfocus={() => (showHistory = history.length > 0)}
        onblur={() => setTimeout(() => (showHistory = false), 200)}
      />
      {#if showHistory && history.length > 0}
        <div class="absolute z-30 mt-1 w-full rounded-lg border border-blender-border bg-blender-card shadow-xl">
          <p class="px-3 py-1.5 text-[10px] font-semibold uppercase tracking-wider text-zinc-500">Recentes</p>
          {#each history as p}
            <button
              type="button"
              class="flex w-full items-center justify-between gap-2 px-3 py-2 text-left text-xs text-zinc-300 hover:bg-blender-hover"
              onmousedown={() => selectHistory(p)}
            >
              <span class="truncate">{p.path}</span>
              {#if p.label}
                <span class="shrink-0 rounded bg-zinc-700 px-1.5 py-0.5 text-[9px] text-zinc-400">{p.label}</span>
              {/if}
              <span class="shrink-0 text-zinc-600 hover:text-red-400" onmousedown={(e) => removeHistory(e, p.id)}>✕</span>
            </button>
          {/each}
        </div>
      {/if}
    </div>
    <button
      type="button"
      class="shrink-0 rounded-lg border border-blender-border bg-blender-card px-3 py-2 text-xs font-medium text-zinc-200 transition hover:bg-blender-hover"
      onclick={handlePick}
    >Procurar...</button>
    {#if showCreate}
      <button
        type="button"
        class="shrink-0 rounded-lg bg-emerald-600 px-3 py-2 text-xs font-semibold text-white transition hover:bg-emerald-500"
        onclick={openCreateDialog}
      >+ Criar</button>
    {/if}
  </div>
</div>

{#if showCreateDialog}
  <CreateBackendProjectDialog
    targetDir={createDir || "."}
    framework={backendFramework}
    {language}
    {ormStrategy}
    {databaseEngine}
    onclose={() => (showCreateDialog = false)}
    oncreated={onProjectCreated}
  />
{/if}
