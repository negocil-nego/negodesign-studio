<script lang="ts">
  import Breadcrumbs from "../../../../../../project/ui/molecules/breadcrumb/ui/Breadcrumbs.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/breadcrumb/ui/Breadcrumbs.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/breadcrumb/ui/Breadcrumbs.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/breadcrumb/ui/Breadcrumbs.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/breadcrumb/ui/Breadcrumbs.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="breadcrumbs">
  {#snippet preview()}
    <div class="w-full max-w-lg space-y-2">
      <Breadcrumbs items={[{ label: "Home", active: true }]} />
      <Breadcrumbs items={[{ label: "Home", href: "#home" }, { label: "Library", active: true }]} />
      <Breadcrumbs
        items={[
          { label: "Home", href: "#home" },
          { label: "Library", href: "#library" },
          { label: "Data", active: true },
        ]}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/molecules/breadcrumb/ui/Breadcrumbs.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/molecules/breadcrumb/ui/Breadcrumbs.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/molecules/breadcrumb/ui/Breadcrumbs.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/molecules/breadcrumb/ui/Breadcrumbs.dart" source={flutterSource} />
  {/snippet}
</Terminal>
