<script lang="ts">
  import type { Snippet } from "svelte";
  import "./layout.css";

  let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
  <title>Negodesign</title>
  <link rel="stylesheet" href="/fonts/fonts.css" />
  <link rel="stylesheet" href="/hugeicons/css/hugeicons.css" />
  <link rel="stylesheet" href="/styles/popover.css" />
  <link rel="stylesheet" href="/styles/input-phone-flag.css" />
</svelte:head>

{@render children()}
