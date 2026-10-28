<script lang="ts">
    import type { ModalProps } from "../data/types";
    import { StyleComponent } from "../../../../../data/style.component";

    let { open = $bindable(false), onClose, title, children }: ModalProps = $props();

    let dialog: HTMLDialogElement | undefined = $state();

    // O estado nativo do <dialog> e a prop `open` precisam continuar sincronizados nos dois sentidos.
    $effect(() => {
        if (!dialog) return;
        if (open && !dialog.open) dialog.showModal();
        if (!open && dialog.open) dialog.close();
    });

    function handleClose() {
        open = false;
        onClose?.();
    }

    function handleClick(event: MouseEvent) {
        if (event.target === dialog) dialog?.close();
    }
</script>

<dialog
    bind:this={dialog}
    class={StyleComponent.modal.root}
    aria-label={title}
    aria-modal="true"
    onclick={handleClick}
    onclose={handleClose}
>
    {@render children?.()}
</dialog>
