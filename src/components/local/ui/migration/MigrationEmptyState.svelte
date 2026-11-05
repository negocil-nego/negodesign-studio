<script lang="ts">
  let {
    title,
    description,
  }: { title: string; description: string } = $props();
</script>

<div class="rounded-xl border border-dashed border-blender-border px-6 py-16 text-center">
  <p class="text-sm font-semibold text-zinc-700 dark:text-zinc-200">{title}</p>
  <p class="mx-auto mt-2 max-w-md text-xs leading-5 text-zinc-500">{description}</p>
  <a
    href="/"
    class="mt-5 inline-flex rounded-md bg-blue-600 px-4 py-2 text-xs font-semibold text-white transition hover:bg-blue-500"
  >
    Voltar ao início
  </a>
</div>
