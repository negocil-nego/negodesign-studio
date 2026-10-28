<div class="grid h-full min-h-full grid-cols-2 bg-blender-card text-zinc-800 dark:text-zinc-100">
  <div class="flex flex-col justify-between bg-blue-600 p-6 text-white">
    <span class="flex h-9 w-9 items-center justify-center rounded-lg bg-white/15 font-semibold">N</span>
    <div><p class="text-xs font-semibold uppercase tracking-wider text-blue-100">Negodesign</p><h1 class="mt-2 text-xl font-semibold">O seu próximo projecto começa aqui.</h1><p class="mt-2 text-xs leading-relaxed text-blue-100">Junte-se à comunidade e transforme ideias em interfaces.</p></div>
    <span class="text-[10px] text-blue-100">© 2026 Negodesign</span>
  </div>
  <div class="flex items-center px-5 py-7">
    <div class="w-full">
      <h2 class="text-xl font-semibold">Criar conta</h2><p class="mt-1 text-xs text-zinc-500 dark:text-zinc-400">É rápido e gratuito.</p>
      <form class="mt-5 space-y-3" onsubmit={(event) => event.preventDefault()}>
        <label class="block text-xs font-medium">Nome<input type="text" placeholder="O seu nome" class="mt-1 block w-full rounded-md border border-blender-border bg-blender-base px-2.5 py-2 text-xs outline-none focus:border-blue-500" /></label>
        <label class="block text-xs font-medium">Email<input type="email" placeholder="nome@exemplo.com" class="mt-1 block w-full rounded-md border border-blender-border bg-blender-base px-2.5 py-2 text-xs outline-none focus:border-blue-500" /></label>
        <label class="block text-xs font-medium">Palavra-passe<input type="password" placeholder="Crie uma palavra-passe" class="mt-1 block w-full rounded-md border border-blender-border bg-blender-base px-2.5 py-2 text-xs outline-none focus:border-blue-500" /></label>
        <button class="w-full rounded-md bg-blue-600 px-3 py-2 text-xs font-semibold text-white hover:bg-blue-700">Criar conta</button>
      </form>
      <p class="mt-4 text-center text-[10px] text-zinc-500">Já tem conta? <span class="font-medium text-blue-600">Iniciar sessão</span></p>
    </div>
  </div>
</div>