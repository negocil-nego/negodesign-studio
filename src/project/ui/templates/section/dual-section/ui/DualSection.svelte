<script lang="ts">
  import SectionForm from "../../../../organisms/section/form/ui/SectionForm.svelte";
  import type { LoginDualSectionProps } from "../data/types";

  let { leftSectionForm, rightSectionForm }: LoginDualSectionProps = $props();
</script>

<section class="grid grid-cols-1 md:grid-cols-2">
  <SectionForm {...leftSectionForm} />
  <SectionForm {...rightSectionForm} />
</section>
