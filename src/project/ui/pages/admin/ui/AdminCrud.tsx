import { useState } from "react";
import DataTable from "../../../organisms/data-table/ui/DataTable";
import Menu from "../../../organisms/menu/simple/ui/Menu";
import ModalForm from "../../../organisms/modal/form/ui/ModalForm";
import Sidebar from "../../../organisms/sidebar/ui/Sidebar";
import { StyleComponent } from "../../../../data/style.component";
import type { DataTextProps } from "../../../organisms/data-table/data/types";
import type { AdminProps } from "../data/types";

export default function AdminCrud({
  datatableProps,
  sidebarProps,
  modalFormProps,
  menuProps,
  title,
  breadcrumb = ["Admin Dashboard"],
  addButtonLabel,
  className = "",
  onCreateClick,
  onRowAction,
}: AdminProps) {
  const S = StyleComponent.pages.admin;
  const [internalModalOpen, setInternalModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<"create" | "edit">("create");

  const isModalOpen = modalFormProps?.open ?? internalModalOpen;
  const pageTitle = title ?? datatableProps?.title ?? modalFormProps?.title ?? "Team List";
  const resolvedAddLabel = addButtonLabel ?? datatableProps?.addLabel ?? (modalFormProps ? "ADD NEW" : undefined);

  const openCreateModal = () => {
    setModalMode("create");
    setInternalModalOpen(true);
    datatableProps?.onAdd?.();
    onCreateClick?.();
  };

  const closeModal = () => {
    setInternalModalOpen(false);
    modalFormProps?.onClose?.();
  };

  const handleRowAction = (action: string, rowIndex: number, row: DataTextProps[]) => {
    if (action.toLowerCase() === "edit" || action.toLowerCase() === "editar") {
      setModalMode("edit");
      setInternalModalOpen(true);
    }
    datatableProps?.onRowAction?.(action, rowIndex, row);
    onRowAction?.(action, rowIndex, row);
  };

  return (
    <section className={`${S.root} ${className}`}>
      {sidebarProps && (
        <div className={S.sidebarWrap}>
          <Sidebar {...sidebarProps} />
        </div>
      )}

      <div className={S.mainWrap}>
        {menuProps && (
          <header className={S.topBar}>
            <Menu {...menuProps} />
          </header>
        )}

        <div className={S.headerBar}>
          <div>
            <h1 className={S.title}>{pageTitle}</h1>
            <nav className={S.breadcrumb} aria-label="Breadcrumb">
              {breadcrumb.map((item, idx) => (
                <span key={idx} className="inline-flex items-center gap-1.5">
                  <span className={idx === 0 ? S.breadcrumbActive : ""}>{item}</span>
                  <span aria-hidden>›</span>
                </span>
              ))}
              <span>{pageTitle}</span>
            </nav>
          </div>

          {resolvedAddLabel && (
            <button type="button" className={S.addButton} onClick={openCreateModal}>
              <span>+ {resolvedAddLabel}</span>
            </button>
          )}
        </div>

        {datatableProps && (
          <main className={S.content}>
            <DataTable
              {...datatableProps}
              onAdd={openCreateModal}
              onRowAction={handleRowAction}
            />
          </main>
        )}
      </div>

      {modalFormProps && (
        <ModalForm
          {...modalFormProps}
          open={isModalOpen}
          title={modalFormProps.title ?? (modalMode === "edit" ? `Edit ${pageTitle}` : `Create ${pageTitle}`)}
          onClose={closeModal}
        />
      )}
    </section>
  );
}

