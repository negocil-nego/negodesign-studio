<script lang="ts">
  import Progress from "../../../../../../project/ui/atoms/progress/ui/Progress.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/progress/ui/Progress.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/progress/ui/Progress.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/progress/ui/Progress.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/progress/ui/Progress.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="progress">
  {#snippet preview()}
    <div class="w-full max-w-lg space-y-3">
      <Progress value={25} variant="primary" />
      <Progress value={25} variant="success" />
      <Progress value={50} variant="info" />
      <Progress value={75} variant="warning" />
      <Progress value={100} variant="danger" />
      <Progress
        bars={[
          { value: 15, variant: "primary" },
          { value: 30, variant: "success" },
          { value: 20, variant: "info" },
        ]}
      />
      <Progress value={75} variant="primary" striped={true} animated={true} />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/progress/ui/Progress.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/progress/ui/Progress.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/progress/ui/Progress.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/progress/ui/Progress.dart" source={flutterSource} />
  {/snippet}
</Terminal>
