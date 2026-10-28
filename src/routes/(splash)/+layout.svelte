<script lang="ts">
  import { onMount } from "svelte";
  import MenuGroup from "../../components/local/ui/MenuGroup.svelte";
  import LogoApp from "../../components/local/ui/varients/LogoApp.svelte";
  import VersionApp from "../../components/local/ui/varients/VersionApp.svelte";
  import { IconNamed } from "../../hooks/icon.svelte";
  import { StyleComponent } from "../../components/local/data/style.component";
  import { goto } from "$app/navigation";

  const { children } = $props();
  let theme = $state<"light" | "dark">("light");
  let showProjectOptions = $state(false);

  onMount(() => {
    theme =
      localStorage.getItem("negodesign-theme") === "dark" ? "dark" : "light";
    document.documentElement.dataset.theme = theme;
  });

  function toggleTheme() {
    theme = theme === "light" ? "dark" : "light";
    document.documentElement.dataset.theme = theme;
    localStorage.setItem("negodesign-theme", theme);
  }
</script>

<main class="{StyleComponent.layout.app} flex flex-col">
  <div class="flex min-h-0 flex-1">
    <aside
      class="{StyleComponent.layout.sidebar} h-full w-64 shrink-0 p-4 flex flex-col justify-between"
    >
      <div class="space-y-4">
        <div class="flex items-start gap-2">
          <div class="min-w-0 flex-1"><LogoApp /></div>
          <button
            type="button"
            class="{StyleComponent.layout.iconButton} h-8 w-8 shrink-0"
            onclick={toggleTheme}
            title={theme === "light"
              ? "Ativar modo escuro"
              : "Ativar modo claro"}
            aria-label={theme === "light"
              ? "Ativar modo escuro"
              : "Ativar modo claro"}>{theme === "light" ? "☾" : "☀"}</button
          >
        </div>

        <div class="space-y-4">
          <MenuGroup
            name="Menu"
            items={[
              {
                name: "Novo Projecto",
                path: "/main/project",
                icon: IconNamed.FolderAdd,
                onclick: () => (showProjectOptions = true),
              },
              {
                name: "Templates",
                path: "/main/template",
                icon: IconNamed.Layout04,
              },
            ]}
          />

          <MenuGroup
            name="Configurações"
            items={[
              {
                name: "Idioma",
                path: "/settings/language",
                icon: IconNamed.Languages,
              },
              {
                name: "Segurança",
                path: "/settings/security",
                icon: IconNamed.Security,
              },
            ]}
          />

          <MenuGroup
            name="Página"
            items={[
              {
                name: "Documentação",
                path: "/page/docs",
                icon: IconNamed.DocumentValidation,
              },
              {
                name: "Ajuda",
                path: "/page/help",
                icon: IconNamed.HelpSquare,
              },
            ]}
          />

          <MenuGroup name="Projectos Recentes" items={[]} />
        </div>
      </div>

      <VersionApp />
    </aside>

    <section class="{StyleComponent.layout.workspace} {StyleComponent.layout.circuitBg}">
      {@render children()}
    </section>
  </div>

  {#if showProjectOptions}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/55 p-4" role="presentation" onclick={(event) => { if (event.target === event.currentTarget) showProjectOptions = false; }} onkeydown={(event) => { if (event.key === "Escape") showProjectOptions = false; }}>
      <section class="w-full max-w-lg rounded-xl border border-blender-border bg-white p-6 text-slate-900 shadow-2xl dark:bg-blender-card dark:text-zinc-100" role="dialog" aria-modal="true" aria-labelledby="project-options-title">
        <div class="mb-5 flex items-start justify-between gap-4">
          <div>
            <p class="text-xs font-semibold uppercase tracking-wider text-blue-500">Frontend</p>
            <h2 id="project-options-title" class="mt-1 text-xl font-semibold">Como quer começar?</h2>
            <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">Escolha uma opção para criar o seu projecto.</p>
          </div>
          <button type="button" class="rounded p-2 text-zinc-500 hover:bg-blender-hover" aria-label="Fechar" onclick={() => (showProjectOptions = false)}>✕</button>
        </div>
        <div class="grid gap-3 sm:grid-cols-2">
          <button type="button" class="rounded-lg border border-blender-border p-4 text-left hover:border-blue-500 hover:bg-blue-50 dark:hover:bg-blue-950/30" onclick={() => { showProjectOptions = false; void goto('/main/template'); }}>
            <span class="text-2xl" aria-hidden="true">▦</span>
            <span class="mt-3 block font-semibold">Layouts</span>
            <span class="mt-1 block text-sm text-zinc-500 dark:text-zinc-400">Conheça os layouts prontos da plataforma.</span>
          </button>
          <button type="button" class="rounded-lg border border-blender-border p-4 text-left hover:border-blue-500 hover:bg-blue-50 dark:hover:bg-blue-950/30" onclick={() => { showProjectOptions = false; void goto('/main/project'); }}>
            <span class="text-2xl" aria-hidden="true">◫</span>
            <span class="mt-3 block font-semibold">Componentes</span>
            <span class="mt-1 block text-sm text-zinc-500 dark:text-zinc-400">Explore os componentes disponíveis.</span>
          </button>
        </div>
      </section>
    </div>
  {/if}
</main>
