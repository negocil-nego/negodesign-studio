<script lang="ts">
  import InputUsername from "../../../ui/InputUsername.svelte";
  import type { InputUsernameOrEmailProps } from "../data/types";

  let { ...props }: InputUsernameOrEmailProps = $props();
</script>

<InputUsername {...props} validationMode="username-or-email" />
