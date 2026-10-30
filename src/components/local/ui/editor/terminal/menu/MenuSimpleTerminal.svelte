<script lang="ts">
  import MenuSimple from "../../../../../../project/ui/organisms/menu/simple/ui/Menu.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/menu/simple/ui/Menu.svelte?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="menu-simple">
  {#snippet preview()}
    <article class="min-h-[68vh] overflow-hidden rounded-xl border border-blender-border bg-blender-base">
      <MenuSimple
        logo={{ src: "/img/programmer-01.jpg", alt: "Negodesign", className: "h-9 w-9 rounded-md object-cover" }}
        menus={[
          { href: "/", icon: { name: "Home" }, textProps: { text: "Início" } },
          { href: "/main/project", textProps: { text: "Projectos" } },
          { href: "/main/project", textProps: { text: "Equipa" } },
        ]}
        buttonsProps={[{ text: "Entrar", variant: "outline" }, { text: "Criar conta", variant: "solid" }]}
      />
      <div class="flex min-h-[60vh] flex-col items-center justify-center px-8 py-12 text-center">
        <p class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-400">Negodesign</p>
        <h2 class="mt-4 max-w-xl text-3xl font-semibold text-zinc-100">Crie experiências digitais à sua maneira</h2>
        <p class="mt-3 max-w-lg text-sm leading-relaxed text-zinc-400">Um menu simples, com navegação e acções alinhadas no centro da página.</p>
      </div>
    </article>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/organisms/menu/simple/ui/Menu.svelte"
      source={svelteSource}
    />
  {/snippet}
</Terminal>
