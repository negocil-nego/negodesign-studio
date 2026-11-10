<script lang="ts">
  import RadioButton from "../../../../../../project/ui/atoms/radio-button/ui/RadioButton.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/radio-button/ui/RadioButton.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/radio-button/ui/RadioButton.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/radio-button/ui/RadioButton.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/radio-button/ui/RadioButton.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="radio-button">
  {#snippet preview()}
    <div class="flex flex-col gap-3">
      <RadioButton
        id="docs-chk-btn"
        name="btncheck"
        multiple={true}
        values={["c1"]}
        options={[
          { value: "c1", label: "Checkbox 1" },
          { value: "c2", label: "Checkbox 2" },
          { value: "c3", label: "Checkbox 3" },
        ]}
      />
      <RadioButton
        id="docs-rad-btn"
        name="btnradio"
        value="r1"
        options={[
          { value: "r1", label: "Radio 1" },
          { value: "r2", label: "Radio 2" },
          { value: "r3", label: "Radio 3" },
        ]}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/radio-button/ui/RadioButton.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/radio-button/ui/RadioButton.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/radio-button/ui/RadioButton.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/radio-button/ui/RadioButton.dart" source={flutterSource} />
  {/snippet}
</Terminal>
