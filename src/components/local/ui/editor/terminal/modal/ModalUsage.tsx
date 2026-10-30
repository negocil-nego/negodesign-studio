import { useState } from "react";
import Button from "../../../../../../project/ui/atoms/button/variant/default/ui/Button";
import Modal from "../../../../../../project/ui/organisms/modal/core/ui/Modal";

export default function ModalUsage() {
  const [open, setOpen] = useState(false);
  return <div className="flex min-h-48 items-center justify-center rounded-lg border border-blender-border bg-blender-base p-6">
    <Button text="Abrir modal" onClick={() => setOpen(true)} />
    <Modal open={open} title="Detalhes do projecto" onClose={() => setOpen(false)}>
      <div className="w-full max-w-md space-y-4 p-5">
        <div><p className="text-xs font-semibold uppercase tracking-wider text-blue-400">Negodesign</p><h2 className="mt-1 text-lg font-semibold text-zinc-100">Detalhes do projecto</h2><p className="mt-2 text-sm leading-6 text-zinc-400">Use o modal para apresentar conteúdo contextual sem sair da página actual.</p></div>
        <div className="flex justify-end"><Button text="Fechar" onClick={() => setOpen(false)} /></div>
      </div>
    </Modal>
  </div>;
}
