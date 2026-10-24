<script lang="ts">
  import BackendSupport from "../varients/BackendSupport.svelte";
  import FrameworkSupport from "./../varients/FrameworkSupport.svelte";
  import { StyleComponent } from "../../data/style.component";
</script>

<div class="space-y-3 flex-1 flex flex-col justify-between">
  <div>
    <div class="{StyleComponent.layout.header} justify-between border-b border-blender-border pb-2 mb-3">
      <span>Ecossistema Suportado</span>
      <span class="text-[10px] text-zinc-400 font-normal">Multiplataforma</span>
    </div>

    <div class="space-y-2.5">
      <div class={StyleComponent.layout.panel}>
        <div class="text-[11px] font-medium text-zinc-300 mb-2 flex items-center justify-between">
          <span>Frontend & Multiplataforma</span>
          <span class="text-[10px] text-zinc-400 font-mono">Web • App • Mobile</span>
        </div>
        <FrameworkSupport />
      </div>

      <div class={StyleComponent.layout.panel}>
        <div class="text-[11px] font-medium text-zinc-300 mb-2 flex items-center justify-between">
          <span>Backend (OpenAPI • Swagger • Scalar)</span>
          <span class="text-[10px] text-zinc-400 font-mono">DTOs & Endpoints</span>
        </div>
        <BackendSupport />
      </div>
    </div>
  </div>

  <!-- Detalhes expandidos visíveis quando a janela do app estiver ampla ou maximizada -->
  <div class="pt-2">
    <div class="p-3 rounded bg-blender-base border border-blender-border text-xs text-zinc-400 space-y-1.5">
      <div class="flex items-center gap-1.5 text-zinc-200 font-medium text-xs">
        <span class="w-2 h-2 rounded-full bg-blue-500"></span>
        <span>Modo Studio Desktop Nativo</span>
      </div>
      <p class="text-[11px] text-zinc-400 leading-relaxed">
        Gera componentes 100% nativos em Tailwind CSS. Mantém a consistência de UI/UX em Svelte, React, Vue, Flutter e Tauri.
      </p>
    </div>
  </div>
</div>
