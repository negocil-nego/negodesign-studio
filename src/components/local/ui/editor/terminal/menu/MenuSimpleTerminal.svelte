<script lang="ts">
  import MenuSimple from "../../../../../../project/ui/organisms/menu/simple/ui/Menu.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/menu/simple/ui/MenuSimple.svelte?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="menu-simple">
  {#snippet preview()}
    <div class="w-full rounded-lg border border-blender-border bg-blender-base p-4">
      <MenuSimple
        logo={{ src: "/img/programmer-01.jpg", alt: "Negodesign", className: "h-9 w-9 rounded-md object-cover" }}
        menus={[
          { icon: { name: "Home" }, textProps: { text: "Início" } },
          { icon: { name: "FolderAdd" }, textProps: { text: "Projectos" } },
          { icon: { name: "User" }, textProps: { text: "Equipa" } },
        ]}
        buttonsProps={[
          { text: "Entrar", variant: "outline" },
          { text: "Criar conta", variant: "solid" },
        ]}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/organisms/menu/simple/ui/MenuSimple.svelte"
      source={svelteSource}
    />
  {/snippet}
</Terminal>
