<script lang="ts">
  import InputUsername from "../../../ui/InputUsername.svelte";
  import type { InputUsernameOrPhoneProps } from "../data/types";

  let { ...props }: InputUsernameOrPhoneProps = $props();
</script>

<InputUsername
  {...props}
  inputProps={{ ...props.inputProps, icon: props.inputProps.icon ?? "Phone" }}
  validationMode="username-or-phone"
/>
