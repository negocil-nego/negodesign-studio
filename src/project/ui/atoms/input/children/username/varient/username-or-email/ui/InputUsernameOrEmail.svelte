<script lang="ts">
  import InputUsername from "../../../ui/InputUsername.svelte";
  import type { InputUsernameOrEmailProps } from "../data/types";

  let { ...props }: InputUsernameOrEmailProps = $props();
</script>

<InputUsername
  {...props}
  inputProps={{ ...props.inputProps, icon: props.inputProps.icon ?? "Email" }}
  validationMode="username-or-email"
/>
