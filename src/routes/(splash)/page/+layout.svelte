<script lang="ts">
  import type { Snippet } from "svelte";
  import ButtonProject from "../../../components/local/ui/button/ButtonProject.svelte";
  import ButtonSettings from "../../../components/local/ui/button/ButtonSettings.svelte";
  import ButtonTemplate from "../../../components/local/ui/button/ButtonTemplate.svelte";
  import { StyleComponent } from "../../../components/local/data/style.component";

  let { children }: { children: Snippet } = $props();
</script>

<div class="{StyleComponent.layout.app} flex flex-col">
  <header class="{StyleComponent.layout.topbar} mt-5">
    <div class="flex items-center gap-3">
      <a
        href="/"
        class="flex items-center gap-2 text-sm text-zinc-400 hover:text-slate-900 transition-colors"
      >
        <span class="text-sm">←</span>
        <span>Voltar ao Studio</span>
      </a>
      <span class="text-zinc-600">|</span>
      <span class="text-xs font-semibold text-zinc-300 tracking-wide">
        NegoDesign Studio
      </span>
    </div>

    <div class="flex items-center gap-2">
      <ButtonProject />
      <ButtonTemplate />
      <ButtonSettings />
    </div>
  </header>

  <main class="flex-1 bg-blender-base overflow-y-auto p-4 md:p-6 lg:p-8">
    {@render children()}
  </main>
</div>
