import { useEffect, useRef, type ReactNode } from "react";
import { StyleComponent } from "../../../../../data/style.component";
type ModalProps = { open?: boolean; onClose?: () => void; title?: string; children?: ReactNode };

export default function Modal({ open = false, onClose, title, children }: ModalProps) {
  const ref = useRef<HTMLDialogElement>(null);
  useEffect(() => { const dialog = ref.current; if (!dialog) return; if (open && !dialog.open) dialog.showModal(); if (!open && dialog.open) dialog.close(); }, [open]);
  const close = () => onClose?.();
  return <dialog ref={ref} className={StyleComponent.modal.root} aria-label={title} aria-modal="true" onClose={close} onClick={event => { if (event.target === ref.current) ref.current?.close(); }}>{children}</dialog>;
}
