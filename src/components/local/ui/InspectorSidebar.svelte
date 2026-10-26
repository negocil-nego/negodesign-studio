<script lang="ts">
  import { onMount } from "svelte";
  import { IconNamed, getIconClass } from "../../../hooks/icon.svelte";
  import { StyleComponent } from "../data/style.component";
  import {
    activeProject as activeProjectStore,
    listProjects,
    saveProject,
    setActiveProject,
    type ProjectRecord,
    type ComponentExport,
    activeProjectComponents,
    refreshActiveProjectComponents,
  } from "../data/projects";

  type ComponentTreeNode = {
    name: string;
    children: ComponentTreeNode[];
    component?: ComponentExport;
  };

  let projects = $state<ProjectRecord[]>([]);
  let activeProject = $state<ProjectRecord | null>(null);
  let busy = $state(false);
  let treeLoading = $state(false);
  let message = $state("");
  let treeMessage = $state("");
  let componentTree = $derived(buildComponentTree($activeProjectComponents));

  onMount(() => {
    void loadProjects();
  });

  function buildComponentTree(components: ComponentExport[]): ComponentTreeNode {
    const root: ComponentTreeNode = { name: "negodesign", children: [] };

    for (const component of components) {
      const segments = component.entry.replaceAll("\\", "/").split("/").filter(Boolean);
      let current = root;

      segments.forEach((name, index) => {
        let child = current.children.find((node) => node.name === name);
        if (!child) {
          child = { name, children: [] };
          current.children.push(child);
        }
        if (index === segments.length - 1) child.component = component;
        current = child;
      });
    }

    function sortNodes(node: ComponentTreeNode) {
      node.children.sort((left, right) => {
        const leftIsFolder = left.children.length > 0;
        const rightIsFolder = right.children.length > 0;
        return Number(rightIsFolder) - Number(leftIsFolder) || left.name.localeCompare(right.name);
      });
      node.children.forEach(sortNodes);
    }

    sortNodes(root);
    return root;
  }

  async function refreshComponentTree() {
    if (!activeProject) {
      activeProjectComponents.set([]);
      treeMessage = "";
      return;
    }

    treeLoading = true;
    treeMessage = "";
    try {
      await refreshActiveProjectComponents();
    } catch (error) {
      activeProjectComponents.set([]);
      treeMessage = error instanceof Error ? error.message : String(error);
    } finally {
      treeLoading = false;
    }
  }

  async function loadProjects() {
    try {
      projects = await listProjects();
      activeProject = projects.find((project) => project.active) ?? null;
      activeProjectStore.set(activeProject);
      await refreshComponentTree();
      message = "";
    } catch (error) {
      message = error instanceof Error ? error.message : String(error);
    }
  }

  async function chooseProject() {
    busy = true;
    message = "";

    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const selectedPath = await open({
        directory: true,
        multiple: false,
        title: "Abrir pasta do projecto",
      });

      if (!selectedPath || Array.isArray(selectedPath)) return;

      activeProject = await saveProject(selectedPath);
      activeProjectStore.set(activeProject);
      activeProjectComponents.set([]);
      await loadProjects();
    } catch (error) {
      message = error instanceof Error ? error.message : String(error);
    } finally {
      busy = false;
    }
  }

  async function activateProject(project: ProjectRecord) {
    try {
      activeProject = await setActiveProject(project.id);
      activeProjectStore.set(activeProject);
      activeProjectComponents.set([]);
      message = "";
      projects = await listProjects();
      await refreshComponentTree();
    } catch (error) {
      message = error instanceof Error ? error.message : String(error);
    }
  }
</script>

{#snippet treeNode(node: ComponentTreeNode, depth: number)}
  <li>
    {#if node.children.length}
      <details open>
        <summary
          class="flex cursor-pointer list-none items-center gap-2 rounded px-2 py-1.5 text-xs text-zinc-300 transition-colors hover:bg-blender-hover"
          style="padding-left: {(depth * 0.75 + 0.5).toFixed(2)}rem"
        >
          <span class="text-[10px] text-zinc-500" aria-hidden="true">▾</span>
          <span class="text-amber-400" aria-hidden="true">📁</span>
          <span class="truncate">{node.name}</span>
        </summary>
        <ul class="ml-2 border-l border-blender-border/70 pl-1" role="group">
          {#each node.children as child (child.name)}
            {@render treeNode(child, depth + 1)}
          {/each}
        </ul>
      </details>
    {:else}
      <div
        class="flex items-center gap-2 rounded px-2 py-1.5 text-[11px] text-zinc-400"
        style="padding-left: {(depth * 0.75 + 0.75).toFixed(2)}rem"
        title={node.component ? `${node.component.name} · ${node.component.entry}` : node.name}
        role="treeitem"
      >
        <span class="text-zinc-600" aria-hidden="true">└</span>
        <span class="text-blue-400" aria-hidden="true">▧</span>
        <span class="min-w-0 flex-1 truncate">{node.component?.name ?? node.name}</span>
        <span class="shrink-0 text-[10px] text-zinc-600">{node.name}</span>
      </div>
    {/if}
  </li>
{/snippet}

<aside
  class="{StyleComponent.layout.sidebar} flex h-full w-70 min-w-60 flex-col overflow-hidden border-l border-blender-border"
>
  <div class="{StyleComponent.layout.topbar} px-3">
    <span class="text-xs font-semibold uppercase tracking-widest text-zinc-500">Projecto</span>
  </div>

  <div class="min-h-0 flex-1 overflow-y-auto p-3">
    <section class="space-y-2 rounded-md border border-blender-border bg-blender-card p-2.5">
      <div class="flex items-center justify-between gap-2">
        <div class="min-w-0">
          <h2 class="text-sm font-semibold text-zinc-200">Projecto</h2>
          <p class="truncate text-[10px] text-zinc-500">Svelte, Vue, React ou Flutter</p>
        </div>
        <button
          type="button"
          class="{StyleComponent.layout.action} shrink-0 gap-2 px-2.5 py-1.5 text-xs"
          onclick={chooseProject}
          disabled={busy}
        >
          <i class={`${getIconClass(IconNamed.FolderAdd)} h-4 w-4 text-sm opacity-80`} aria-hidden="true"></i>
          <span>{busy ? "A abrir…" : "Escolher pasta"}</span>
        </button>
      </div>

      {#if activeProject}
        <div class="flex min-w-0 items-center justify-between gap-2 border-t border-blender-border pt-2">
          <div class="min-w-0">
            <p class="truncate text-xs font-semibold text-zinc-200" title={activeProject.path}>{activeProject.name}</p>
            <p class="truncate text-[10px] text-zinc-500" title={activeProject.path}>{activeProject.path}</p>
          </div>
          <span class="shrink-0 rounded bg-blue-500/10 px-2 py-1 text-[10px] font-medium text-blue-400">
            {activeProject.language}
          </span>
        </div>
      {:else}
        <p class="border-t border-blender-border pt-2 text-[11px] text-zinc-500">
          Escolha uma pasta para ver os componentes NegoDesign usados no projecto.
        </p>
      {/if}

      {#if message}
        <p class="wrap-break-word text-xs text-rose-500" role="status">{message}</p>
      {/if}
    </section>

    <section class="mt-3 rounded-md border border-blender-border bg-blender-card p-2.5">
      <div class="mb-2 flex items-center justify-between gap-2">
        <div class="min-w-0">
          <h2 class="text-xs font-semibold text-zinc-200">Componentes NegoDesign</h2>
          <p class="text-[10px] text-zinc-500">negodesign/</p>
        </div>
        <button
          type="button"
          class="{StyleComponent.layout.iconButton} h-7 w-7 text-xs"
          onclick={refreshComponentTree}
          disabled={!activeProject || treeLoading}
          aria-label="Actualizar árvore de componentes"
          title="Actualizar árvore"
        >
          {treeLoading ? "…" : "↻"}
        </button>
      </div>

      {#if treeMessage}
        <p class="wrap-break-word text-[11px] text-rose-500" role="status">{treeMessage}</p>
      {:else if !activeProject}
        <p class="rounded border border-dashed border-blender-border px-2 py-3 text-center text-[11px] text-zinc-500">
          Seleccione um projecto para ver a pasta.
        </p>
      {:else if treeLoading && !$activeProjectComponents.length}
        <p class="py-3 text-center text-[11px] text-zinc-500">A carregar a árvore…</p>
      {:else if componentTree.children.length}
        <div class="max-h-[45vh] overflow-auto rounded bg-blender-base/60 p-1" role="tree" aria-label="Componentes instalados na pasta negodesign">
          <div class="flex items-center gap-2 rounded px-2 py-1.5 text-xs font-semibold text-zinc-200" role="treeitem">
            <span class="text-amber-400" aria-hidden="true">📁</span>
            <span>negodesign</span>
          </div>
          <ul class="ml-2 border-l border-blender-border/70 pl-1">
            {#each componentTree.children as child (child.name)}
              {@render treeNode(child, 0)}
            {/each}
          </ul>
        </div>
      {:else}
        <p class="rounded border border-dashed border-blender-border px-2 py-3 text-center text-[11px] text-zinc-500">
          Ainda não há componentes instalados neste projecto.
        </p>
      {/if}
    </section>

    <details class="mt-3 rounded-md border border-blender-border bg-blender-card">
      <summary class="flex cursor-pointer list-none items-center justify-between px-3 py-2 text-xs font-semibold text-zinc-400 hover:bg-blender-hover">
        <span>Projectos guardados</span>
        <span class="text-[10px] font-normal text-zinc-500">{projects.length}</span>
      </summary>
      <div class="space-y-1 border-t border-blender-border p-2">
        {#if projects.length}
          {#each projects as project (project.id)}
            <button
              type="button"
              class="w-full rounded px-2.5 py-2 text-left transition-colors hover:bg-blender-hover {project.active ? 'bg-blue-500/10 ring-1 ring-blue-500/30' : ''}"
              onclick={() => activateProject(project)}
              title={project.path}
            >
              <span class="block truncate text-xs font-medium text-zinc-300">{project.name}</span>
              <span class="block truncate text-[10px] text-blue-400">{project.language}{project.active ? " · Activo" : ""}</span>
              <span class="mt-0.5 block truncate text-[10px] text-zinc-500">{project.path}</span>
            </button>
          {/each}
        {:else}
          <p class="rounded border border-dashed border-blender-border px-3 py-4 text-center text-xs text-zinc-500">
            Ainda não há projectos guardados.
          </p>
        {/if}
      </div>
    </details>
  </div>
</aside>