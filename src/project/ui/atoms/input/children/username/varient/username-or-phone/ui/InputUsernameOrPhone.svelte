<script lang="ts">
  import InputUsername from "../../../ui/InputUsername.svelte";
  import type { InputUsernameOrPhoneProps } from "../data/types";

  let { ...props }: InputUsernameOrPhoneProps = $props();
</script>

<InputUsername {...props} validationMode="username-or-phone" />
