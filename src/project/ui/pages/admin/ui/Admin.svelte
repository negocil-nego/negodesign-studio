<script lang="ts">
  import AdminCrud from "./AdminCrud.svelte";
  import type { AdminProps } from "../data/types";

  let { datatableProps, sidebarProps, modalFormProps, menuProps, ...rest }: AdminProps =
    $props();
</script>

<AdminCrud {datatableProps} {sidebarProps} {modalFormProps} {menuProps} {...rest} />
