<script lang="ts">
  import type { Snippet } from "svelte";
  import "./layout.css";

  let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
  <title>Negodesign</title>
</svelte:head>

{@render children()}
