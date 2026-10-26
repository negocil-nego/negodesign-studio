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
  } from "../data/projects";

  let projects = $state<ProjectRecord[]>([]);
  let activeProject = $state<ProjectRecord | null>(null);
  let busy = $state(false);
  let message = $state("");

  onMount(() => {
    void loadProjects();
  });

  async function loadProjects() {
    try {
      projects = await listProjects();
      activeProject = projects.find((project) => project.active) ?? null;
      activeProjectStore.set(activeProject);
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
      message = "";
      projects = await listProjects();
    } catch (error) {
      message = error instanceof Error ? error.message : String(error);
    }
  }
</script>

<aside
  class="{StyleComponent.layout
    .sidebar} flex h-full w-70 min-w-60 flex-col overflow-hidden border-l border-blender-border"
>
  <div class="{StyleComponent.layout.topbar} px-3">
    <span class="text-xs font-semibold uppercase tracking-widest text-zinc-500"
      >Projecto</span
    >
  </div>

  <div class="min-h-0 flex-1 overflow-y-auto p-3">
    <section
      class="space-y-3 rounded-md border border-blender-border bg-blender-card p-3"
    >
      <div>
        <h2 class="text-sm font-semibold text-zinc-200">Abrir projecto</h2>
        <p class="mt-1 text-xs leading-5 text-zinc-500">
          Seleccione a pasta de um projecto Svelte, Vue, React ou Flutter.
        </p>
      </div>

      <button
        type="button"
        class="{StyleComponent.layout.action} w-full gap-3 px-3 py-2.5 text-sm"
        onclick={chooseProject}
        disabled={busy}
      >
        <span class="flex min-w-0 items-center gap-2.5">
          <i
            class={`${getIconClass(IconNamed.FolderAdd)} h-5 w-5 shrink-0 text-base opacity-80`}
            aria-hidden="true"
          ></i>
          <span>{busy ? "A abrir..." : "Escolher pasta"}</span>
        </span>
        <span aria-hidden="true" class="text-zinc-500">↗</span>
      </button>

      {#if message}
        <p class="wrap-break-word text-xs text-rose-500" role="status">
          {message}
        </p>
      {/if}

      {#if activeProject}
        <div class="space-y-1 border-t border-blender-border pt-3">
          <p
            class="text-[10px] font-semibold uppercase tracking-wider text-blue-500"
          >
            Projecto activo
          </p>
          <h2 class="wrap-break-word text-sm font-semibold text-zinc-200">
            {activeProject.name}
          </h2>
          <p class="text-xs font-medium text-zinc-400">
            {activeProject.language}
          </p>
          <p class="break-all text-[11px] leading-5 text-zinc-500">
            {activeProject.path}
          </p>
        </div>
      {/if}

      <div class="space-y-2 border-t border-blender-border pt-3">
        <h2
          class="px-1 text-[10px] font-semibold uppercase tracking-wider text-zinc-500"
        >
          Projectos guardados
        </h2>
        {#if projects.length}
          <ul class="space-y-1">
            {#each projects as project (project.id)}
              <li>
                <button
                  type="button"
                  class="w-full rounded px-2.5 py-2 text-left transition-colors hover:bg-blender-hover {project.active
                    ? 'bg-blue-500/10 ring-1 ring-blue-500/30'
                    : ''}"
                  onclick={() => activateProject(project)}
                  title={project.path}
                >
                  <span class="block truncate text-xs font-medium text-zinc-300"
                    >{project.name}</span
                  >
                  <span class="block text-[10px] text-blue-400"
                    >{project.language}{project.active ? " · Activo" : ""}</span
                  >
                  <span class="mt-0.5 block truncate text-[10px] text-zinc-500"
                    >{project.path}</span
                  >
                </button>
              </li>
            {/each}
          </ul>
        {:else}
          <p
            class="rounded border border-dashed border-blender-border px-3 py-4 text-center text-xs text-zinc-500"
          >
            Ainda não há projectos guardados.
          </p>
        {/if}
      </div>
    </section>
  </div>
</aside>
