Neste modal terás a opção para criar uma secção de migração de projecto que terá duas opções: 

Banco de dados: quando clicado nele irá abrir uma modal pedindo a conexeção com o banco de dados
que será apenas MySql, PostgreSql, de modo e depois deves 
abrir a rota "\src\routes\(editor)\main\database\+page.svelte" de modo que deves apresentar
as estruturas das tabelas do banco de dados.

OpenApi: quando clicado nesta opção irá abrir uma modal pendo a rota que esta a documenta do swagger ou
openapi que irás extrair as entidades como response, dto que irás apresentar nesta rota
"\src\routes\(editor)\main\openapi\+page.svelte"

  {#if showProjectOptions}
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/55 p-4"
      role="presentation"
      onclick={(event) => {
        if (event.target === event.currentTarget) showProjectOptions = false;
      }}
      onkeydown={(event) => {
        if (event.key === "Escape") showProjectOptions = false;
      }}
    >
      <div
        class="w-full max-w-2xl rounded-xl border border-blender-border bg-white p-6 text-slate-900 shadow-2xl dark:bg-blender-card dark:text-zinc-100"
        role="dialog"
        aria-modal="true"
        aria-labelledby="project-options-title"
      >
        <div class="mb-6 flex items-start justify-between gap-4"></div>
          <div>
            <p
              class="text-xs font-semibold uppercase tracking-wider text-blue-500"
            >
              Projecto
            </p>
            <h2 id="project-options-title" class="mt-1 text-xl font-semibold">
              Como quer começar?
            </h2>
            <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
              Escolha uma opção para criar o seu projecto.
            </p>
          </div>
          <button
            type="button"
            class="rounded p-2 text-zinc-500 hover:bg-blender-hover"
            aria-label="Fechar"
            onclick={() => (showProjectOptions = false)}>✕</button
          >
        </div>
        <!-- Frontend Section -->
        <div class="mb-6"></div>
          <h3 class="mb-3 text-sm font-semibold uppercase tracking-wider text-zinc-400 dark:text-zinc-500">Frontend</h3>
          <div class="grid gap-3 sm:grid-cols-2"></div>
            onclick={() => {
              showProjectOptions = false;
              void goto("/main/template");
            }}
          >
            <span class="text-2xl" aria-hidden="true">▦</span>
            <span class="mt-3 block font-semibold">Layouts</span>
            <span class="mt-1 block text-sm text-zinc-500 dark:text-zinc-400"
              >Conheça os layouts prontos da plataforma.</span
            >
          </button>
          <button
            type="button"
            class="rounded-lg border border-blender-border p-4 text-left hover:border-blue-500 hover:bg-blue-50 dark:hover:bg-blue-950/30"
            onclick={() => {
              showProjectOptions = false;
              void goto("/main/project");
            }}
          >
            <span class="text-2xl" aria-hidden="true">◫</span>
            <span class="mt-3 block font-semibold">Componentes</span>
            <span class="mt-1 block text-sm text-zinc-500 dark:text-zinc-400"
              >Explore os componentes disponíveis.</span
            >
          </button>
        </div>
      </div>
    </div>
  {/if}