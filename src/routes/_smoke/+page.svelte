<script lang="ts">
    import ModalForm from "../../project/ui/molecules/modal/form/ui/ModalForm.svelte";
    import type { FormProps } from "../../project/ui/molecules/form/data/types";

    let open = $state(false);

    const form: FormProps = {
        fields: [{ id: "email", component: "input-email", name: "email", label: "Email" }],
        onSubmit: () => {
            open = false;
        },
    };
</script>

<div class="flex flex-col gap-4 p-10">
    <button class="rounded bg-blue-600 px-4 py-2 text-white" onclick={() => (open = true)}>abrir modal</button>
    <p id="estado">estado: {open}</p>
</div>

<ModalForm {open} onClose={() => (open = false)} title="Modal de teste" {form} />
