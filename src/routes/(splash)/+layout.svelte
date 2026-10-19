<script>
  import MenuGroup from "../../components/local/ui/MenuGroup.svelte";
  import LogoApp from "../../components/local/ui/varients/LogoApp.svelte";
  import VersionApp from "../../components/local/ui/varients/VersionApp.svelte";
  import { IconNamed } from "../../hooks/icon.svelte";

  const { children } = $props();
</script>

<main class="blender-app flex">
  <aside
    class="blender-sidebar w-64 h-screen p-4 flex flex-col justify-between shrink-0"
  >
    <div class="space-y-4">
      <LogoApp />

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

  <section class="blender-workspace blender-circuit-bg">
    {@render children()}
  </section>
</main>
