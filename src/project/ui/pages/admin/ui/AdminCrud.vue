<script setup lang="ts">
import { computed, ref } from "vue";
import DataTable from "../../../organisms/data-table/ui/DataTable.vue";
import Menu from "../../../organisms/menu/simple/ui/Menu.vue";
import ModalForm from "../../../organisms/modal/form/ui/ModalForm.vue";
import Sidebar from "../../../organisms/sidebar/ui/Sidebar.vue";
import { StyleComponent } from "../../../../data/style.component";
import type { DataTextProps } from "../../../organisms/data-table/data/types";
import type { AdminProps } from "../data/types";

const props = withDefaults(defineProps<AdminProps>(), {
  breadcrumb: () => ["Admin Dashboard"],
  className: "",
});

const S = StyleComponent.pages.admin;
const internalModalOpen = ref(false);
const modalMode = ref<"create" | "edit">("create");

const isModalOpen = computed(() => props.modalFormProps?.open ?? internalModalOpen.value);
const pageTitle = computed(
  () => props.title ?? props.datatableProps?.title ?? props.modalFormProps?.title ?? "Team List",
);
const resolvedAddLabel = computed(
  () => props.addButtonLabel ?? props.datatableProps?.addLabel ?? (props.modalFormProps ? "ADD NEW" : undefined),
);

function openCreateModal() {
  modalMode.value = "create";
  internalModalOpen.value = true;
  props.datatableProps?.onAdd?.();
  props.onCreateClick?.();
}

function closeModal() {
  internalModalOpen.value = false;
  props.modalFormProps?.onClose?.();
}

function handleRowAction(action: string, rowIndex: number, row: DataTextProps[]) {
  if (action.toLowerCase() === "edit" || action.toLowerCase() === "editar") {
    modalMode.value = "edit";
    internalModalOpen.value = true;
  }
  props.datatableProps?.onRowAction?.(action, rowIndex, row);
  props.onRowAction?.(action, rowIndex, row);
}
</script>

<template>
  <section :class="`${S.root} ${className}`">
    <div v-if="sidebarProps" :class="S.sidebarWrap">
      <Sidebar v-bind="sidebarProps" />
    </div>

    <div :class="S.mainWrap">
      <header v-if="menuProps" :class="S.topBar">
        <Menu v-bind="menuProps" />
      </header>

      <div :class="S.headerBar">
        <div>
          <h1 :class="S.title">{{ pageTitle }}</h1>
          <nav :class="S.breadcrumb" aria-label="Breadcrumb">
            <template v-for="(item, idx) in breadcrumb" :key="idx">
              <span :class="idx === 0 ? S.breadcrumbActive : ''">{{ item }}</span>
              <span aria-hidden="true">›</span>
            </template>
            <span>{{ pageTitle }}</span>
          </nav>
        </div>

        <button v-if="resolvedAddLabel" type="button" :class="S.addButton" @click="openCreateModal">
          <span>+ {{ resolvedAddLabel }}</span>
        </button>
      </div>

      <main v-if="datatableProps" :class="S.content">
        <DataTable
          v-bind="datatableProps"
          :onAdd="openCreateModal"
          :onRowAction="handleRowAction"
        />
      </main>
    </div>

    <ModalForm
      v-if="modalFormProps"
      v-bind="modalFormProps"
      :open="isModalOpen"
      :title="modalFormProps.title ?? (modalMode === 'edit' ? `Edit ${pageTitle}` : `Create ${pageTitle}`)"
      @close="closeModal"
    />
  </section>
</template>

