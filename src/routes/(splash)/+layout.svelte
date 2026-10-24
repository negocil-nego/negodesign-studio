<script lang="ts">
  import { onMount } from "svelte";
  import MenuGroup from "../../components/local/ui/MenuGroup.svelte";
  import LogoApp from "../../components/local/ui/varients/LogoApp.svelte";
  import VersionApp from "../../components/local/ui/varients/VersionApp.svelte";
  import { IconNamed } from "../../hooks/icon.svelte";
  import { StyleComponent } from "../../components/local/data/style.component";

  const { children } = $props();
  let theme = $state<"light" | "dark">("light");

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
</main>
