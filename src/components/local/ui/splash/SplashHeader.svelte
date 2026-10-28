<script lang="ts">
  import { getIconClass, IconNamed } from "../../../../hooks/icon.svelte";
</script>

<div
  class="relative w-full h-44 md:h-52 lg:h-64 xl:h-72 shrink-0 overflow-hidden select-none border-b border-blender-border"
>
  <img
    src="/img/programmer-01.jpg"
    alt="NegoDesign Banner"
    class="w-full h-full object-cover object-center filter brightness-75 contrast-105 transform hover:scale-102 transition-transform duration-700"
  />
  <div
    class="absolute inset-0 bg-linear-to-t from-blender-surface via-blender-surface/60 to-transparent"
  ></div>
  <div
    class="absolute inset-0 bg-linear-to-r from-blender-surface/90 via-transparent to-blender-surface/60"
  ></div>

  <div class="absolute inset-0 p-5 md:p-6 lg:p-8 flex flex-col justify-between">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span
          class="px-2.5 py-1 rounded bg-blue-600/90 text-white text-[11px] font-semibold tracking-wider uppercase flex items-center gap-1.5 shadow-sm"
        >
          <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
          v0.1.0 Desktop
        </span>
        <span class="text-xs text-zinc-300 hidden sm:inline font-medium"
          >NegoDesign Studio</span
        >
      </div>
    </div>

    <div>
      <h1
        class="text-2xl md:text-3xl lg:text-4xl font-extrabold tracking-tight text-slate-900 dark:text-zinc-100 flex items-center gap-2.5"
      >
        <span>NegoDesign</span>
        <span
          class="text-slate-700 dark:text-zinc-300 font-normal text-lg md:text-xl lg:text-2xl"
          >Desktop</span
        >
      </h1>
      <p class="text-xs md:text-sm text-zinc-300 mt-1 max-w-xl leading-relaxed">
        Ecossistema de UI multiplataforma e conversão OpenAPI para Frontend
      </p>
    </div>
  </div>
</div>
