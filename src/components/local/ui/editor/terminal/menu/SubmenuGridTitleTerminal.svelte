<script lang="ts">
  import SubmenuGridTitle from "../../../../../../project/ui/organisms/menu/shared/submenu/grid/title/ui/SubmenuGridTitle.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/menu/shared/submenu/grid/title/ui/SubmenuGridTitle.svelte?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="submenu-grid-title">
  {#snippet preview()}
    <div class="flex min-h-[68vh] items-center overflow-hidden rounded-xl border border-blender-border bg-blender-base p-3 @md:p-5">
      <div class="@container w-full overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-lg">
        <SubmenuGridTitle
          title={{
            title: { text: "Uma plataforma para criar com confiança" },
            description: { text: "Organize projectos, componentes e sistemas num só espaço, pronto para crescer com a sua equipa." },
            href: "/main/project",
            link: { text: "Saiba mais" },
          }}
          items={[
            {
              title: { text: "PRINCIPAIS RECURSOS" },
              items: [
                { href: "/main/project", title: { text: "Biblioteca de componentes" }, description: { text: "Construa interfaces consistentes com componentes reutilizáveis." } },
                { href: "/main/project", title: { text: "Editor visual" }, description: { text: "Explore e ajuste componentes num espaço interactivo." } },
                { href: "/main/project", title: { text: "Gestão de projecto" }, description: { text: "Mantenha recursos e decisões organizados." } },
                { href: "/main/project", title: { text: "Colaboração" }, description: { text: "Partilhe ferramentas e ideias com a sua equipa." } },
              ],
            },
            {
              title: { text: "DESCOBRIR" },
              items: [
                { href: "/main/project", title: { text: "Começar" }, description: { text: "Conheça os primeiros passos da plataforma." } },
                { href: "/main/project", title: { text: "Documentação" }, description: { text: "Consulte padrões, propriedades e exemplos." } },
                { href: "/main/project", title: { text: "Actualizações" }, description: { text: "Veja as novidades mais recentes." } },
              ],
            },
          ]}
        />
      </div>
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/organisms/menu/shared/submenu/grid/title/ui/SubmenuGridTitle.svelte"
      source={svelteSource}
    />
  {/snippet}
</Terminal>
