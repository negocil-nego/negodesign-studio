<script lang="ts">
  import Breadcrumbs from "../../../molecules/breadcrumb/ui/Breadcrumbs.svelte";
  import DataTable from "../../../organisms/data-table/ui/DataTable.svelte";
  import Menu from "../../../organisms/menu/simple/ui/Menu.svelte";
  import ModalForm from "../../../organisms/modal/form/ui/ModalForm.svelte";
  import Sidebar from "../../../organisms/sidebar/ui/Sidebar.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import type { BreadcrumbProps } from "../../../molecules/breadcrumb/data/types";
  import type { DataTextProps } from "../../../organisms/data-table/data/types";
  import type { AdminProps } from "../data/types";

  let {
    datatableProps,
    sidebarProps,
    modalFormProps,
    menuProps,
    breadcrumb,
    breadcrumbProps,
    title,
    addButtonLabel,
    className = "",
    onCreateClick,
    onRowAction,
  }: AdminProps = $props();

  const S = StyleComponent.pages.admin;
  let internalModalOpen = $state(false);
  let modalMode = $state<"create" | "edit">("create");

  const isModalOpen = $derived(modalFormProps?.open ?? internalModalOpen);
  const pageTitle = $derived(title ?? datatableProps?.title ?? modalFormProps?.title ?? "Team List");
  const resolvedAddLabel = $derived(addButtonLabel ?? datatableProps?.addLabel ?? (modalFormProps ? "ADD NEW" : undefined));
  const resolvedBreadcrumb = $derived<BreadcrumbProps>(
    breadcrumb ??
      breadcrumbProps ?? {
        items: [
          { label: "Admin Dashboard", href: "#admin" },
          { label: pageTitle, active: true },
        ],
      },
  );

  function openCreateModal() {
    modalMode = "create";
    internalModalOpen = true;
    datatableProps?.onAdd?.();
    onCreateClick?.();
  }

  function closeModal() {
    internalModalOpen = false;
    modalFormProps?.onClose?.();
  }

  function handleRowAction(action: string, rowIndex: number, row: DataTextProps[]) {
    if (action.toLowerCase() === "edit" || action.toLowerCase() === "editar") {
      modalMode = "edit";
      internalModalOpen = true;
    }
    datatableProps?.onRowAction?.(action, rowIndex, row);
    onRowAction?.(action, rowIndex, row);
  }
</script>

<section class={`${S.root} ${className}`}>
  {#if sidebarProps}
    <div class={S.sidebarWrap}>
      <Sidebar {...sidebarProps} />
    </div>
  {/if}

  <div class={S.mainWrap}>
    {#if menuProps}
      <header class={S.topBar}>
        <Menu {...menuProps} />
      </header>
    {/if}

    <div class={S.headerBar}>
      <div class="space-y-1">
        <h1 class={S.title}>{pageTitle}</h1>
        <Breadcrumbs {...resolvedBreadcrumb} />
      </div>

      {#if resolvedAddLabel}
        <button type="button" class={S.addButton} onclick={openCreateModal}>
          <span>+ {resolvedAddLabel}</span>
        </button>
      {/if}
    </div>

    {#if datatableProps}
      <main class={S.content}>
        <DataTable
          {...datatableProps}
          onAdd={openCreateModal}
          onRowAction={handleRowAction}
        />
      </main>
    {/if}
  </div>

  {#if modalFormProps}
    <ModalForm
      {...modalFormProps}
      open={isModalOpen}
      title={modalFormProps.title ?? (modalMode === "edit" ? `Edit ${pageTitle}` : `Create ${pageTitle}`)}
      onClose={closeModal}
    />
  {/if}
</section>
