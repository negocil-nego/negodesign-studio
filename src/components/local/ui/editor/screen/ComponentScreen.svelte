<script lang="ts">
  // ── Atoms · Input ──────────────────────────────────────────────────────────
  import Input from "../../../../../project/ui/atoms/input/core/ui/Input.svelte";
  import InputText from "../../../../../project/ui/atoms/input/children/text/ui/InputText.svelte";
  import InputEmail from "../../../../../project/ui/atoms/input/children/email/ui/InputEmail.svelte";
  import InputPhone from "../../../../../project/ui/atoms/input/children/phone/ui/InputPhone.svelte";
  import InputPhoneFlag from "../../../../../project/ui/atoms/input/children/phone/ui/InputPhoneFlag.svelte";
  import InputPassword from "../../../../../project/ui/atoms/input/children/password/ui/InputPassword.svelte";
  import InputDate from "../../../../../project/ui/atoms/input/children/date/ui/InputDate.svelte";
  import InputTime from "../../../../../project/ui/atoms/input/children/time/ui/InputTime.svelte";
  import InputNumber from "../../../../../project/ui/atoms/input/children/number/ui/InputNumber.svelte";
  import InputAge from "../../../../../project/ui/atoms/input/children/age/ui/InputAge.svelte";
  import InputLabel from "../../../../../project/ui/atoms/input/children/label/ui/InputLabel.svelte";

  // ── Atoms · Control ────────────────────────────────────────────────────────
  import Select from "../../../../../project/ui/atoms/select/ui/Select.svelte";
  import Textarea from "../../../../../project/ui/atoms/textarea/ui/Textarea.svelte";
  import Toggle from "../../../../../project/ui/atoms/toggle/ui/Toggle.svelte";
  import Radio from "../../../../../project/ui/atoms/radio/ui/Radio.svelte";
  import Checkbox from "../../../../../project/ui/atoms/checkbox/ui/Checkbox.svelte";

  // ── Molecules ──────────────────────────────────────────────────────────────
  import CountryPhonePopover from "../../../../../project/ui/molecules/popover/ui/CountryPhonePopover.svelte";
  import { FlagNamed } from "../../../../../hooks/flag.svelte";

  import { componentCatalog, type ComponentDefinition } from "../../../data/components";
  import { activeProject } from "../../../data/projects";
  import { copyComponentToActiveProject } from "../../../data/component-copy";
    import Label from "../../../../../project/ui/atoms/label/ui/label.svelte";

  // ─── Types & helpers ────────────────────────────────────────────────────────

  type Group = {
    label: string;
    badge: string;
    description: string;
    ids: string[];
  };

  /** Logical grouping for display – keeps Input atoms and Control atoms separate. */
  const screenGroups: Group[] = [
    {
      label: "Atoms · Input",
      badge: "Atom",
      description: "Campos de entrada de texto e dados estruturados.",
      ids: [
        "input",
        "input-text",
        "input-email",
        "input-phone",
        "input-phone-flag",
        "input-label",
        "input-password",
        "input-date",
        "input-time",
        "input-number",
        "input-age",
      ],
    },
    {
      label: "Atoms · Control",
      badge: "Control",
      description: "Controlos de selecção e alternância.",
      ids: ["select", "textarea", "toggle", "radio", "checkbox"],
    },
    {
      label: "Typography",
      badge: "Atom",
      description: "Elementos de texto e rótulo.",
      ids: ["label"],
    },
    {
      label: "Molecules",
      badge: "Molecule",
      description: "Componentes compostos por múltiplos átomos.",
      ids: ["country-popover"],
    },
  ];

  const catalogById = Object.fromEntries(componentCatalog.map((c) => [c.id, c]));

  const components = componentCatalog;
  let copyingId = $state<string | null>(null);
  let copyMessage = $state("");

  // ── Demo data for control previews ──────────────────────────────────────────
  const selectOptions = [
    { value: "ao", label: "Angola" },
    { value: "pt", label: "Portugal" },
    { value: "br", label: "Brasil" },
  ];

  const radioOptions = [
    { value: "svelte", label: "Svelte" },
    { value: "react", label: "React" },
    { value: "vue", label: "Vue" },
  ];

  async function copyComponent(component: (typeof components)[number]) {
    if (!component.dependence) return;
    copyingId = component.id;
    copyMessage = "";
    try {
      const result = await copyComponentToActiveProject(component.id);
      copyMessage = `${component.label ?? component.name}: ${result.copiedFiles} ficheiro(s) e ${result.exportedComponents} componente(s) disponíveis em ${result.project.name}/negodesign.`;
    } catch (error) {
      copyMessage = error instanceof Error ? error.message : String(error);
    } finally {
      copyingId = null;
    }
  }
</script>

<main class="space-y-8">
  <!-- ── Header ──────────────────────────────────────────────────────────── -->
  <header class="flex items-end justify-between gap-4">
    <div>
      <p class="text-xs font-semibold uppercase tracking-[0.16em] text-blue-500">
        Project UI
      </p>
      <h1 class="mt-1 text-2xl font-semibold text-zinc-100">Componentes</h1>
      <p class="mt-1 text-sm text-zinc-400">
        Pré-visualização dos componentes existentes neste projecto.
      </p>
    </div>
    <span
      class="shrink-0 rounded-full border border-blender-border bg-blender-card px-3 py-1 text-xs font-medium text-zinc-400"
    >
      {components.length} componentes
    </span>
  </header>

  <!-- ── Active project bar ──────────────────────────────────────────────── -->
  <section
    class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-blender-border bg-blender-card px-4 py-3"
  >
    <div class="min-w-0">
      <p class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">
        Linguagem do projecto activo
      </p>
      {#if $activeProject}
        <p class="mt-1 truncate text-sm font-semibold text-zinc-100">
          {$activeProject.language}
          <span class="font-normal text-zinc-400">· {$activeProject.name}</span>
        </p>
      {:else}
        <p class="mt-1 text-sm text-zinc-400">
          Escolha um projecto no Inspector para activar a cópia de componentes.
        </p>
      {/if}
    </div>
    {#if $activeProject}
      <span
        class="rounded-full border border-blue-500/30 bg-blue-500/10 px-3 py-1 text-xs font-medium text-blue-400"
        >{$activeProject.language}</span
      >
    {/if}
  </section>

  {#if copyMessage}
    <p class="text-xs text-zinc-400" role="status">{copyMessage}</p>
  {/if}

  <!-- ── Component groups ────────────────────────────────────────────────── -->
  {#each screenGroups as group}
    {@const groupItems = group.ids.map((id) => catalogById[id]).filter(Boolean)}
    {#if groupItems.length}
      <section class="space-y-4">
        <!-- Group header -->
        <div class="flex items-end gap-3 border-b border-blender-border pb-2">
          <div class="min-w-0 flex-1">
            <p class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">
              {group.badge}
            </p>
            <h2 class="text-base font-semibold text-zinc-100">{group.label}</h2>
            <p class="mt-0.5 text-xs text-zinc-500">{group.description}</p>
          </div>
          <span
            class="shrink-0 rounded-full border border-blender-border bg-blender-base px-2.5 py-0.5 text-[10px] font-medium text-zinc-500"
            >{groupItems.length}</span
          >
        </div>

        <!-- Cards grid -->
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {#each groupItems as component (component.id)}
            <article
              class="overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-sm transition-shadow hover:shadow-md"
            >
              <!-- Preview area -->
              <div class="flex min-h-36 items-center justify-center bg-blender-base p-5">
                {#if component.id === "input"}
                  <div class="w-full max-w-xs">
                    <Input type="text" placeholder="Introduza um valor" />
                  </div>
                {:else if component.id === "input-text"}
                  <div class="w-full max-w-xs">
                    <InputText
                      inputProps={{ id: "gallery-text", type: "text", placeholder: "Introduza um valor" }}
                    />
                  </div>
                {:else if component.id === "input-email"}
                  <div class="w-full max-w-xs">
                    <InputEmail
                      inputProps={{ id: "gallery-email", type: "email", placeholder: "nome@exemplo.com" }}
                    />
                  </div>
                {:else if component.id === "input-phone"}
                  <div class="w-full max-w-xs">
                    <InputPhone
                      inputProps={{ id: "gallery-phone", type: "tel", placeholder: "+244 900 000 000" }}
                    />
                  </div>
                {:else if component.id === "input-phone-flag"}
                  <div class="w-full max-w-xs">
                    <InputPhoneFlag
                      flag={[FlagNamed.Ao, FlagNamed.Ng]}
                      inputProps={{ id: "gallery-phone-flag", type: "tel", placeholder: "900 000 000" }}
                    />
                  </div>
                {:else if component.id === "input-password"}
                  <div class="w-full max-w-xs">
                    <InputPassword
                      inputProps={{ id: "gallery-password", type: "password", placeholder: "Palavra-passe" }}
                    />
                  </div>
                {:else if component.id === "input-date"}
                  <div class="w-full max-w-xs">
                    <InputDate inputProps={{ id: "gallery-date", type: "date" }} />
                  </div>
                {:else if component.id === "input-time"}
                  <div class="w-full max-w-xs">
                    <InputTime inputProps={{ id: "gallery-time", type: "time" }} />
                  </div>
                {:else if component.id === "input-number"}
                  <div class="w-full max-w-xs">
                    <InputNumber
                      inputProps={{ id: "gallery-number", type: "number", placeholder: "0" }}
                    />
                  </div>
                {:else if component.id === "input-age"}
                  <div class="w-full max-w-xs">
                    <InputAge
                      labelProps={{ label: "Idade", for: "gallery-age" }}
                      inputProps={{ id: "gallery-age", type: "number", placeholder: "18" }}
                    />
                  </div>
                {:else if component.id === "input-label"}
                  <div class="w-full max-w-xs">
                    <InputLabel
                      labelProps={{ label: "Nome", for: "gallery-input-label" }}
                      inputProps={{ id: "gallery-input-label", type: "text", placeholder: "O seu nome" }}
                    />
                  </div>
                {:else if component.id === "label"}
                  <div class="w-full max-w-xs space-y-2">
                    <Label label="Nome" for="gallery-label" />
                    <input
                      id="gallery-label"
                      type="text"
                      placeholder="O seu nome"
                      class="w-full rounded-md border border-blender-border bg-blender-card px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-500"
                    />
                  </div>
                {:else if component.id === "select"}
                  <div class="w-full max-w-xs">
                    <Select
                      id="gallery-select"
                      options={selectOptions}
                      placeholder="Seleccione um país…"
                    />
                  </div>
                {:else if component.id === "textarea"}
                  <div class="w-full max-w-xs">
                    <Textarea
                      id="gallery-textarea"
                      placeholder="Escreva aqui o seu texto…"
                      rows={3}
                    />
                  </div>
                {:else if component.id === "toggle"}
                  <div class="flex flex-col gap-3">
                    <Toggle id="gallery-toggle-off" label="Desactivado" />
                    <Toggle id="gallery-toggle-on" checked={true} label="Activado" />
                  </div>
                {:else if component.id === "radio"}
                  <Radio name="gallery-radio" options={radioOptions} value="svelte" />
                {:else if component.id === "checkbox"}
                  <div class="flex flex-col gap-3">
                    <Checkbox id="gallery-checkbox-off" label="Não seleccionado" />
                    <Checkbox id="gallery-checkbox-on" checked={true} label="Seleccionado" />
                  </div>
                {:else}
                  <CountryPhonePopover flag={FlagNamed.Ao} locale="pt" />
                {/if}
              </div>

              <!-- Card footer -->
              <div class="space-y-1.5 p-4">
                <div class="flex items-center justify-between gap-3">
                  <h3 class="truncate text-sm font-semibold text-zinc-100">
                    {#if component.href}
                      <a class="hover:text-blue-500" href={component.href}>
                        {component.label ?? component.name}
                      </a>
                    {:else}
                      {component.label ?? component.name}
                    {/if}
                  </h3>
                  <span
                    class="shrink-0 text-[10px] font-medium uppercase tracking-wide text-zinc-400"
                    >{component.group}</span
                  >
                </div>
                <p class="text-xs leading-5 text-zinc-400">
                  {component.description}
                </p>
                {#if component.href || component.dependence}
                  <div class="flex flex-wrap gap-2 pt-2">
                    {#if component.href}
                      <a
                        href={component.href}
                        class="rounded border border-blender-border px-2.5 py-1.5 text-[11px] font-medium text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
                        >Ver terminal</a
                      >
                    {/if}
                    {#if component.dependence}
                      <button
                        type="button"
                        class="rounded bg-blue-600 px-2.5 py-1.5 text-[11px] font-medium text-white transition-colors hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-40"
                        onclick={() => copyComponent(component)}
                        disabled={!$activeProject ||
                          $activeProject.language === "Unknown" ||
                          copyingId === component.id}
                        >{copyingId === component.id
                          ? "A copiar…"
                          : "Copiar para projecto"}</button
                      >
                    {/if}
                  </div>
                {/if}
              </div>
            </article>
          {/each}
        </div>
      </section>
    {/if}
  {/each}
</main>
