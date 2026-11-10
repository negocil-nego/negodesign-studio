<script lang="ts">
  import Containers from "../../../../../../project/ui/molecules/container/ui/Containers.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/container/ui/Containers.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/container/ui/Containers.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/container/ui/Containers.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/container/ui/Containers.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="containers">
  {#snippet preview()}
    <div class="w-full max-w-2xl">
      <Containers />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/molecules/container/ui/Containers.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/molecules/container/ui/Containers.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/molecules/container/ui/Containers.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/molecules/container/ui/Containers.dart" source={flutterSource} />
  {/snippet}
</Terminal>
