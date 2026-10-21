<script lang="ts">
  import LabelTerminal from "../terminal/LabelTerminal.svelte";
</script>

<div class="space-y-6">
  <div>
    <p
      class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400"
    >
      Átomos / Label
    </p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Label</h2>
    <p class="mt-2 max-w-3xl text-xs leading-5 text-zinc-400">
      Texto associado a um controlo de formulário. A propriedade <code
        class="text-blue-300">for</code
      >
      deve corresponder ao <code class="text-blue-300">id</code> do campo para que
      o utilizador possa focar o controlo ao seleccionar o rótulo.
    </p>
  </div>

  <section
    class="rounded-md border border-blender-border bg-blender-base/60 p-4"
  >
    <h3 class="mb-4 text-xs font-semibold text-zinc-200">
      Preview e código-fonte
    </h3>
    <LabelTerminal />
  </section>
</div>
