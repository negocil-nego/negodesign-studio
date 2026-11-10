<script lang="ts">
  import UnderlineNav from "../../../../../../project/ui/molecules/underline-nav/ui/UnderlineNav.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/underline-nav/ui/UnderlineNav.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/underline-nav/ui/UnderlineNav.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/underline-nav/ui/UnderlineNav.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/underline-nav/ui/UnderlineNav.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="underline-nav">
  {#snippet preview()}
    <div class="w-full max-w-md">
      <UnderlineNav
        value="active"
        items={[
          { value: "active", label: "Active" },
          { value: "link-1", label: "Link" },
          { value: "link-2", label: "Link" },
          { value: "disabled", label: "Disabled", disabled: true },
        ]}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/molecules/underline-nav/ui/UnderlineNav.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/molecules/underline-nav/ui/UnderlineNav.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/molecules/underline-nav/ui/UnderlineNav.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/molecules/underline-nav/ui/UnderlineNav.dart" source={flutterSource} />
  {/snippet}
</Terminal>
