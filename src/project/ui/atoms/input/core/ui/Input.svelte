<script lang="ts">
  import type { InputProps } from "../data/types";

  let { type, required, disabled, placeholder, value }: InputProps = $props();
</script>

<input {type} {required} {disabled} {placeholder} {value} data-name="label" />
