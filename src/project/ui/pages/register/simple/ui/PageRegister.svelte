<div class="flex h-full min-h-full items-center justify-center bg-blender-card px-6 py-7 text-zinc-800 dark:text-zinc-100">
  <div class="w-full max-w-sm">
    <div class="mb-6 flex h-10 w-10 items-center justify-center rounded-lg bg-blue-600 font-semibold text-white">N</div>
    <p class="text-xs font-semibold uppercase tracking-wider text-blue-600">Comece hoje</p>
    <h1 class="mt-1 text-2xl font-semibold">Criar conta</h1>
    <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">Preencha os dados para se registar.</p>
    <form class="mt-6 space-y-3" onsubmit={(event) => event.preventDefault()}>
      <label class="block text-sm font-medium">Nome<input type="text" placeholder="O seu nome" class="mt-1 block w-full rounded-md border border-blender-border bg-blender-base px-3 py-2 text-sm outline-none focus:border-blue-500" /></label>
      <label class="block text-sm font-medium">Email<input type="email" placeholder="nome@exemplo.com" class="mt-1 block w-full rounded-md border border-blender-border bg-blender-base px-3 py-2 text-sm outline-none focus:border-blue-500" /></label>
      <label class="block text-sm font-medium">Palavra-passe<input type="password" placeholder="Crie uma palavra-passe" class="mt-1 block w-full rounded-md border border-blender-border bg-blender-base px-3 py-2 text-sm outline-none focus:border-blue-500" /></label>
      <button class="w-full rounded-md bg-blue-600 px-4 py-2.5 text-sm font-semibold text-white hover:bg-blue-700">Criar conta</button>
    </form>
    <p class="mt-4 text-center text-xs text-zinc-500">Já tem conta? <span class="font-medium text-blue-600">Iniciar sessão</span></p>
  </div>
</div>