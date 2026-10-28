<script lang="ts">
  import SectionForm from "../../../../organisms/section/form/ui/SectionForm.svelte";
  import type { LoginSimpleProps } from "../data/types";

  let { sectionFormProps }: LoginSimpleProps = $props();
</script>

<section>
  <SectionForm {...sectionFormProps} />
</section>
