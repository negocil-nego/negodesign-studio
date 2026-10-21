<script lang="ts">
  type Props = { path: string; source: string };

  let { path, source }: Props = $props();
</script>

<div class="space-y-2">
  <p class="truncate text-[10px] text-slate-500">{path}</p>
  <pre class="max-h-128 overflow-auto text-[11px] leading-5 text-emerald-800"><code>{source}</code></pre>
</div>
