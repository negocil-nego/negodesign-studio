<script lang="ts">
  import SectionForm from "../../../../organisms/section/form/ui/SectionForm.svelte";
  import type { SimpleProps } from "../data/types";

  let { sectionFormProps }: SimpleProps = $props();
</script>

<section class="h-full min-h-full">
  <SectionForm {...sectionFormProps} />
</section>
