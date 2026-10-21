<script lang="ts">
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
  import Label from "../../../../../project/ui/atoms/label/ui/Label.svelte";
  import CountryPhonePopover from "../../../../../project/ui/molecules/popover/ui/CountryPhonePopover.svelte";
  import { FlagNamed } from "../../../../../hooks/flag.svelte";
  import { componentCatalog } from "../../../data/components";

  const components = componentCatalog;
</script>

<main class="space-y-5">
  <header class="flex items-end justify-between gap-4">
    <div>
      <p
        class="text-xs font-semibold uppercase tracking-[0.16em] text-blue-500"
      >
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

  {#each ["Input", "Typography"] as category}
    {@const categoryComponents = components.filter(
      (component) => component.category === category,
    )}
    <section class="space-y-3">
      <h2 class="text-sm font-semibold uppercase tracking-wider text-zinc-200">
        {category}
      </h2>
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
        {#each categoryComponents as component (component.id)}
          <article
            class="overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-sm transition-shadow hover:shadow-md"
          >
            <div
              class="flex min-h-36 items-center justify-center bg-blender-base p-5"
            >
              {#if component.id === "input"}
                <div class="w-full max-w-xs">
                  <Input type="text" placeholder="Introduza um valor" />
                </div>
              {:else if component.id === "text"}
                <div class="w-full max-w-xs">
                  <InputText
                    inputProps={{
                      id: "gallery-text",
                      type: "text",
                      placeholder: "Introduza um valor",
                    }}
                  />
                </div>
              {:else if component.id === "email"}
                <div class="w-full max-w-xs">
                  <InputEmail
                    inputProps={{
                      id: "gallery-email",
                      type: "email",
                      placeholder: "nome@exemplo.com",
                    }}
                  />
                </div>
              {:else if component.id === "phone"}
                <div class="w-full max-w-xs">
                  <InputPhone
                    inputProps={{
                      id: "gallery-phone",
                      type: "tel",
                      placeholder: "+244 900 000 000",
                    }}
                  />
                </div>
              {:else if component.id === "phone-flag"}
                <div class="w-full max-w-xs">
                  <InputPhoneFlag
                    flag={[FlagNamed.Ao, FlagNamed.Ng]}
                    inputProps={{
                      id: "gallery-phone-flag",
                      type: "tel",
                      placeholder: "900 000 000",
                    }}
                  />
                </div>
              {:else if component.id === "password"}
                <div class="w-full max-w-xs">
                  <InputPassword
                    inputProps={{
                      id: "gallery-password",
                      placeholder: "Palavra-passe",
                    }}
                  />
                </div>
              {:else if component.id === "date"}
                <div class="w-full max-w-xs">
                  <InputDate inputProps={{ id: "gallery-date" }} />
                </div>
              {:else if component.id === "time"}
                <div class="w-full max-w-xs">
                  <InputTime inputProps={{ id: "gallery-time" }} />
                </div>
              {:else if component.id === "number"}
                <div class="w-full max-w-xs">
                  <InputNumber
                    inputProps={{ id: "gallery-number", placeholder: "0" }}
                  />
                </div>
              {:else if component.id === "age"}
                <div class="w-full max-w-xs">
                  <InputAge
                    labelProps={{ label: "Idade", for: "gallery-age" }}
                    inputProps={{ id: "gallery-age", placeholder: "18" }}
                  />
                </div>
              {:else if component.id === "input-label"}
                <div class="w-full max-w-xs">
                  <InputLabel
                    labelProps={{ label: "Nome", for: "gallery-input-label" }}
                    inputProps={{
                      id: "gallery-input-label",
                      type: "text",
                      placeholder: "O seu nome",
                    }}
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
              {:else}
                <CountryPhonePopover flag={FlagNamed.Ao} locale="pt" />
              {/if}
            </div>
            <div class="space-y-1.5 p-4">
              <div class="flex items-center justify-between gap-3">
                <h2 class="truncate text-sm font-semibold text-zinc-100">
                  {#if component.href}
                    <a class="hover:text-blue-500" href={component.href}>
                      {component.label ?? component.name}
                    </a>
                  {:else}
                    {component.label ?? component.name}
                  {/if}
                </h2>
                <span
                  class="shrink-0 text-[10px] font-medium uppercase tracking-wide text-zinc-400"
                  >{component.group}</span
                >
              </div>
              <p class="text-xs leading-5 text-zinc-400">
                {component.description}
              </p>
            </div>
          </article>
        {/each}
      </div>
    </section>
  {/each}
</main>
