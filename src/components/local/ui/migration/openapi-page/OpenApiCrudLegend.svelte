<div
  class="rounded-xl border border-blender-border/70 bg-blender-card/60 p-3.5 text-[11px] text-zinc-400 space-y-1.5"
>
  <div class="flex flex-wrap items-center justify-between gap-2">
    <span class="font-semibold text-zinc-200">
      Identificação de Endpoints para `AdminCrud` (RESTful & Nomenclatura de Métodos):
    </span>
    <div class="flex flex-wrap gap-3 text-[10px]">
      <span>
        <span class="font-semibold text-blue-400">DTO / Request</span> → Formulário de criação · Modal Form
      </span>
      <span>
        <span class="font-semibold text-purple-400">Response / Schema</span> → DataTable · Grid · Carousel
      </span>
    </div>
  </div>
  <div class="grid gap-2 sm:grid-cols-4 text-[10px]">
    <div class="rounded border border-emerald-500/20 bg-emerald-500/5 px-2.5 py-1.5">
      <span class="font-semibold text-emerald-400">GET (Listar):</span>
      <span class="ml-1 font-mono text-zinc-400">/list, /all, /find-all, /search, REST GET</span>
    </div>
    <div class="rounded border border-blue-500/20 bg-blue-500/5 px-2.5 py-1.5">
      <span class="font-semibold text-blue-400">POST (Criar):</span>
      <span class="ml-1 font-mono text-zinc-400">/create, /add, /insert, /new, REST POST</span>
    </div>
    <div class="rounded border border-amber-500/20 bg-amber-500/5 px-2.5 py-1.5">
      <span class="font-semibold text-amber-400">PUT/PATCH (Editar):</span>
      <span class="ml-1 font-mono text-zinc-400">/update, /edit, /save, /modify, REST PUT/PATCH</span>
    </div>
    <div class="rounded border border-rose-500/20 bg-rose-500/5 px-2.5 py-1.5">
      <span class="font-semibold text-rose-400">DELETE (Remover):</span>
      <span class="ml-1 font-mono text-zinc-400">/delete, /remove, /destroy, REST DELETE</span>
    </div>
  </div>
</div>
