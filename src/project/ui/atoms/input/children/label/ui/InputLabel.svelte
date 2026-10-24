<script lang="ts">
  import type { InputLabelProps } from "../data/types";
  import Input from "../../../core/ui/Input.svelte";
    import Label from "../../../../label/ui/label.svelte";

  let { labelProps, inputProps }: InputLabelProps = $props();
</script>

<Label {...labelProps} />
<Input {...inputProps} />
