<script lang="ts">
  import { getIconUrl, IconNamed } from "../../../../hooks/icon.svelte";

  const actions = [
    {
      title: "Novo Projecto Frontend",
      desc: "Svelte, React, Vue ou Flutter com Tailwind CSS nativo",
      icon: IconNamed.FolderAdd,
      badge: "Frontend",
      href: "/main/project",
    },
    {
      title: "Gerar via OpenAPI",
      desc: "Lê Swagger/Scalar e gera interface administrativa completa",
      icon: IconNamed.DocumentValidation,
      badge: "OpenAPI",
      href: "/main/project",
    },
    {
      title: "Templates Prontos",
      desc: "Dashboards, autenticação e tabelas de dados configuradas",
      icon: IconNamed.Layout04,
      badge: "Templates",
      href: "/main/template",
    },
  ];
</script>

<div class="space-y-3 flex-1 flex flex-col justify-between">
  <div>
    <div class="blender-header justify-between border-b border-blender-border pb-2 mb-3">
      <span>Início Rápido</span>
      <span class="text-[10px] text-zinc-400 font-normal">Criar ou Importar</span>
    </div>

    <div class="space-y-2.5">
      {#each actions as item}
        <a href={item.href} class="blender-action group">
          <div class="flex items-center gap-3.5 min-w-0">
            <div class="w-9 h-9 rounded bg-blender-base border border-blender-border flex items-center justify-center shrink-0 group-hover:border-blue-400/60 group-hover:scale-105 transition-all">
              <img
                src={getIconUrl(item.icon)}
                alt={item.title}
                class="w-5 h-5 opacity-80 group-hover:opacity-100"
              />
            </div>
            <div class="min-w-0">
              <div class="text-xs md:text-sm font-semibold text-zinc-100 group-hover:text-blue-400 transition-colors truncate">
                {item.title}
              </div>
              <div class="text-[11px] md:text-xs text-zinc-400 line-clamp-1 mt-0.5">
                {item.desc}
              </div>
            </div>
          </div>
          <span class="blender-badge ml-3">
            {item.badge}
          </span>
        </a>
      {/each}
    </div>
  </div>

  <!-- Ação secundária no rodapé da coluna -->
  <div class="pt-2">
    <a
      href="/main/project"
      class="text-sm text-zinc-400 hover:text-slate-900 flex items-center gap-1.5 p-1 transition-colors"
    >
      <span>📂 Abrir pasta de projecto existente...</span>
    </a>
  </div>
</div>
