import type { FormProps } from "../../../../molecules/form/data/types";
import Modal from "../../core/ui/Modal";
import Form from "../../../../molecules/form/ui/form";
type ModalFormProps = { open?: boolean; onClose?: () => void; title?: string; form?: FormProps };
export default function ModalForm({ form, ...modalProps }: ModalFormProps) { return <Modal {...modalProps}>{form && <Form {...form} />}</Modal>; }
