<script lang="ts">
  import DataTable from "../../../organisms/data-table/ui/DataTable.svelte";
  import Menu from "../../../organisms/menu/simple/ui/Menu.svelte";
  import Sidebar from "../../../organisms/sidebar/ui/Sidebar.svelte";
  import type { AdminProps } from "../data/types";
  let { datatableProps, sidebarProps, modalFormProps, menuProps }: AdminProps =
    $props();
</script>

<section>
  {#if sidebarProps}
    <Sidebar {...sidebarProps} />
  {/if}
  {#if datatableProps || menuProps || modalFormProps}
    {#if menuProps}
      <Menu {...menuProps} />
    {/if}
    {#if datatableProps}
      <DataTable {...datatableProps} />
    {/if}
  {/if}
</section>
