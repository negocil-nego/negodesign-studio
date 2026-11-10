<script lang="ts">
  import ListGroups from "../../../../../../project/ui/molecules/list-group/ui/ListGroups.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/list-group/ui/ListGroups.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/list-group/ui/ListGroups.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/list-group/ui/ListGroups.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/list-group/ui/ListGroups.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="list-groups">
  {#snippet preview()}
    <div class="grid w-full max-w-2xl grid-cols-1 gap-4 md:grid-cols-2">
      <ListGroups />
      <ListGroups
        items={[
          {
            id: "rich-1",
            label: "List group item heading",
            meta: "3 days ago",
            description: "Donec id elit non mi porta gravida at eget metus. Maecenas sed diam eget risus varius blandit.",
            footer: "Donec id elit non mi porta.",
            active: true,
          },
          {
            id: "rich-2",
            label: "List group item heading",
            meta: "3 days ago",
            description: "Donec id elit non mi porta gravida at eget metus. Maecenas sed diam eget risus varius blandit.",
            footer: "Donec id elit non mi porta.",
          },
        ]}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/molecules/list-group/ui/ListGroups.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/molecules/list-group/ui/ListGroups.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/molecules/list-group/ui/ListGroups.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/molecules/list-group/ui/ListGroups.dart" source={flutterSource} />
  {/snippet}
</Terminal>
