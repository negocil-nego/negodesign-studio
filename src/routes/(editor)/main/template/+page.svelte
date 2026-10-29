<script lang="ts">
  import PageLogin from "../../../../project/ui/pages/login/simple/ui/PageLogin.svelte";
  import PageLoginDual from "../../../../project/ui/pages/login/dual-section/ui/PageLoginDual.svelte";
  import PageRegister from "../../../../project/ui/pages/register/simple/ui/PageRegister.svelte";
  import PageRegisterDual from "../../../../project/ui/pages/register/dual-section/ui/PageRegisterDual.svelte";
  import PageOtp from "../../../../project/ui/pages/otp/simple/ui/PageOtp.svelte";
  import PageOtpDual from "../../../../project/ui/pages/otp/dual-section/ui/PageOtpDual.svelte";

  const groups = [
    {
      name: "Login",
      pages: [
        { name: "PageLogin", component: PageLogin },
        { name: "PageLoginDual", component: PageLoginDual },
      ],
    },
    {
      name: "Register",
      pages: [
        { name: "PageRegister", component: PageRegister },
        { name: "PageRegisterDual", component: PageRegisterDual },
      ],
    },
    {
      name: "Verificação de OTP",
      pages: [
        { name: "PageOtp", component: PageOtp },
        { name: "PageOtpDual", component: PageOtpDual },
      ],
    },
  ];
  const totalTemplates = groups.reduce((total, group) => total + group.pages.length, 0);
</script>

<div class="min-h-full w-full bg-blender-base p-4 md:p-6 xl:p-8">
  <div class="mx-auto flex w-full max-w-[1800px] flex-col gap-7">
    <header
      class="flex flex-wrap items-end justify-between gap-4 border-b border-blender-border pb-5"
    >
      <div>
        <p
          class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-500"
        >
          Frontend <span class="px-1 text-zinc-500">/</span> Layouts
        </p>
        <h1
          class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100"
        >
          Templates
        </h1>
        <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
          Pré-visualize os modelos de autenticação, registro e verificação OTP.
        </p>
      </div>
      <span
        class="rounded-full border border-blender-border bg-blender-card px-3 py-1.5 text-xs text-zinc-500 dark:text-zinc-400"
        >{totalTemplates} modelos disponíveis</span
      >
    </header>

    {#each groups as group (group.name)}
      <section class="space-y-4" aria-labelledby={`group-${group.name}`}>
        <div class="flex items-center gap-3">
          <h2
            id={`group-${group.name}`}
            class="text-xl font-semibold text-zinc-800 dark:text-zinc-100"
          >
            {group.name}
          </h2>
          <span class="h-px flex-1 bg-blender-border"></span>
          <span class="text-xs text-zinc-500">{group.pages.length} modelos</span
          >
        </div>

        <div
          class="grid grid-cols-[repeat(auto-fit,minmax(min(100%,420px),1fr))] gap-5"
        >
          {#each group.pages as page (page.name)}
            {@const PreviewComponent = page.component}
            <article
              class="group flex min-w-0 flex-col overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-sm transition duration-200 hover:-translate-y-0.5 hover:border-blue-500/60 hover:shadow-lg"
            >
              <div
                class="relative aspect-[1.55] min-h-70 max-h-115 overflow-hidden bg-blender-base"
              >
                <div
                  class="origin-top-left scale-75"
                  style="width: 133.333%; height: 133.333%;"
                >
                  <PreviewComponent />
                </div>
                <div
                  class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
                ></div>
              </div>
              <div
                class="flex items-center justify-between gap-3 border-t border-blender-border px-4 py-3.5"
              >
                <div class="min-w-0">
                  <h3
                    class="truncate font-medium text-zinc-800 dark:text-zinc-100"
                  >
                    {page.name}
                  </h3>
                  <p class="mt-0.5 text-xs text-zinc-500">
                    {group.name} · Preview
                  </p>
                </div>
                <span
                  class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-blender-border text-zinc-500 transition group-hover:border-blue-500/50 group-hover:text-blue-500"
                  aria-hidden="true">↗</span
                >
              </div>
            </article>
          {/each}
        </div>
      </section>
    {/each}
  </div>
</div>
