<script lang="ts">
  import PaginationNav from "../../../../../../project/ui/molecules/pagination/ui/PaginationNav.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/pagination/ui/PaginationNav.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/pagination/ui/PaginationNav.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/pagination/ui/PaginationNav.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/pagination/ui/PaginationNav.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="pagination-nav">
  {#snippet preview()}
    <div class="flex flex-col gap-3">
      <PaginationNav totalPages={5} currentPage={1} size="lg" />
      <PaginationNav totalPages={5} currentPage={2} size="md" />
      <PaginationNav totalPages={5} currentPage={3} size="sm" />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/molecules/pagination/ui/PaginationNav.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/molecules/pagination/ui/PaginationNav.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/molecules/pagination/ui/PaginationNav.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/molecules/pagination/ui/PaginationNav.dart" source={flutterSource} />
  {/snippet}
</Terminal>
