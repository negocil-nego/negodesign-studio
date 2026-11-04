<script lang="ts">
  import type { Component } from "svelte";

  type Loader = () => Promise<{ default: Component<any> }>;

  let {
    loader,
    componentProps = {},
  }: {
    loader: Loader;
    componentProps?: Record<string, any>;
  } = $props();

  let retryKey = $state(0);
  const loadedComponent = $derived.by(async () => {
    retryKey;
    const module = await loader();
    return module.default;
  });

  function retry() {
    retryKey += 1;
  }
</script>

{#await loadedComponent}
  <div
    class="flex min-h-32 items-center justify-center gap-2 text-xs text-zinc-500"
    role="status"
    aria-live="polite"
  >
    <span class="h-4 w-4 animate-spin rounded-full border-2 border-zinc-600 border-t-blue-400"></span>
    <span>A carregar componentes…</span>
  </div>
{:then LoadedComponent}
  <LoadedComponent {...componentProps} />
{:catch error}
  <div
    class="flex min-h-32 flex-col items-center justify-center gap-3 text-xs text-red-300"
    role="alert"
  >
    <p>{error instanceof Error ? error.message : "Não foi possível carregar este grupo."}</p>
    <button
      type="button"
      class="rounded border border-blender-border px-3 py-1.5 text-zinc-300 hover:bg-blender-hover"
      onclick={retry}
    >
      Tentar novamente
    </button>
  </div>
{/await}
