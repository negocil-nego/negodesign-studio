<script lang="ts">
  import { LocalStyleComponent } from "../../../data/style.component";
  import type { BackendFramework, SpringLanguage } from "../../../data/migration";
  import { currentOrmOptions, ormOptionsByBackend } from "./options";

  let {
    backendFramework = $bindable<BackendFramework>("spring-boot"),
    backendPath = $bindable(""),
    basePackage = $bindable(""),
    springLanguage = $bindable<SpringLanguage>("java"),
    backendBaseUrl = $bindable(""),
    ormStrategy = $bindable("spring-data-jpa"),
    onpickfolder,
  }: { backendFramework?: BackendFramework; backendPath?: string; basePackage?: string; springLanguage?: SpringLanguage; backendBaseUrl?: string; ormStrategy?: string; onpickfolder: () => void } = $props();

  const ormOptions = $derived(currentOrmOptions(backendFramework));

  const backendLogos: Record<BackendFramework, string> = {
    "spring-boot": "/langs/spring-boot.svg",
    nestjs: "/langs/nestjs.svg",
    laravel: "/langs/laravel.svg",
    "aspnet-core": "/langs/aspnet-core.svg",
  };

  function selectBackendFramework(fw: BackendFramework) {
    backendFramework = fw;
    ormStrategy = ormOptionsByBackend[fw][0].id;
  }
</script>

<!-- 4. Configuração do Backend (Split vs Monolith) -->
<section class="space-y-4 rounded-xl border border-blender-border bg-blender-base/50 p-4">
  <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">4. Configuração do Backend & Estratégia de Repositório (ORM)</h3>

  <div class="grid gap-3 sm:grid-cols-4">
    {#each [
      { id: "spring-boot", label: "Spring Boot", sub: "Java · Modulith Feature" },
      { id: "nestjs", label: "NestJS", sub: "TypeScript · Feature Modules" },
      { id: "laravel", label: "Laravel", sub: "PHP · Feature Architecture" },
      { id: "aspnet-core", label: "ASP.NET Core", sub: "C# · Feature Controllers" },
    ] as bfw (bfw.id)}
      <button type="button" class={`flex items-center gap-3 rounded-lg border p-3 text-left transition ${backendFramework === bfw.id ? "border-blue-500 bg-blue-500/15 text-zinc-100" : "border-blender-border bg-blender-card text-zinc-400 hover:border-zinc-600"}`} onclick={() => selectBackendFramework(bfw.id as BackendFramework)}>
        <img src={backendLogos[bfw.id as BackendFramework]} alt={bfw.label} class="size-7 shrink-0 object-contain" />
        <div class="min-w-0">
          <span class="block truncate text-xs font-semibold text-zinc-100">{bfw.label}</span>
          <span class="mt-0.5 block text-[10px] text-zinc-400">{bfw.sub}</span>
        </div>
      </button>
    {/each}
  </div>

  <div class="grid gap-4 sm:grid-cols-2">
    <label class="block sm:col-span-2">
      <span class={LocalStyleComponent.migration.label}>Caminho da Aplicação Backend</span>
      <div class="mt-1 flex gap-2">
        <input class={LocalStyleComponent.migration.field} bind:value={backendPath} placeholder="/caminho/para/projecto-backend" required />
        <button type="button" class="shrink-0 rounded-lg border border-blender-border bg-blender-card px-3 py-2 text-xs font-medium text-zinc-200 hover:bg-blender-hover" onclick={onpickfolder}>Procurar…</button>
      </div>
    </label>

    {#if backendFramework === "spring-boot"}
      <label class="block">
        <span class={LocalStyleComponent.migration.label}>Linguagem do Spring Boot</span>
        <div class="mt-1 grid grid-cols-2 gap-2">
          {#each [
            { id: "java" as SpringLanguage, label: "Java", logo: "/langs/java.svg" },
            { id: "kotlin" as SpringLanguage, label: "Kotlin", logo: "/langs/kotlin.svg" },
          ] as lang (lang.id)}
            <button
              type="button"
              class={`flex items-center gap-2 rounded-lg border px-3 py-2 text-xs font-medium transition ${springLanguage === lang.id ? "border-blue-500 bg-blue-500/15 text-blue-300" : "border-blender-border bg-blender-card text-zinc-400 hover:border-zinc-600"}`}
              onclick={() => (springLanguage = lang.id)}
            >
              <img src={lang.logo} alt={lang.label} class="size-5 object-contain" />
              <span>{lang.label}</span>
            </button>
          {/each}
        </div>
      </label>

      <label class="block">
        <span class={LocalStyleComponent.migration.label}>Pacote Base (Feature Package)</span>
        <input class={LocalStyleComponent.migration.field} bind:value={basePackage} placeholder="com.negocil.negoturismo.admin" />
      </label>
    {/if}

    <label class="block">
      <span class={LocalStyleComponent.migration.label}>URL Base do Servidor Backend</span>
      <input class={LocalStyleComponent.migration.field} bind:value={backendBaseUrl} placeholder="http://localhost:8080" />
    </label>
  </div>

  <div class="space-y-2">
    <span class={LocalStyleComponent.migration.label}>Estratégia do Repositório / ORM para {backendFramework}</span>
    <div class="grid gap-3 sm:grid-cols-3">
      {#each ormOptions as opt (opt.id)}
        <button type="button" class={`rounded-lg border p-3 text-left transition ${ormStrategy === opt.id ? "border-emerald-500 bg-emerald-500/10 text-zinc-100" : "border-blender-border bg-blender-card text-zinc-400 hover:border-zinc-600"}`} onclick={() => (ormStrategy = opt.id)}>
          <span class="block text-xs font-semibold text-zinc-100">{opt.label}</span>
          <span class="mt-1 block text-[11px] leading-relaxed text-zinc-400">{opt.desc}</span>
        </button>
      {/each}
    </div>
  </div>
</section>
