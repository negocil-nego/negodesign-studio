<script lang="ts">
    import Form from "../../../form/ui/form.svelte";
    import Modal from "../../core/ui/Modal.svelte";
    import type { ModalFormProps } from "../data/types";

    let { form, ...restProps }: ModalFormProps = $props();
</script>

<Modal {...restProps}>
    {#if form}
        <Form {...form} />
    {/if}
</Modal>
