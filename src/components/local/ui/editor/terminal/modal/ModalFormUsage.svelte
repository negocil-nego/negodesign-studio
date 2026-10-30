<script lang="ts">
  import Button from "../../../../../../project/ui/atoms/button/variant/default/ui/Button.svelte";
  import ModalForm from "../../../../../../project/ui/organisms/modal/form/ui/ModalForm.svelte";

  let open = $state(false);
  const form = {
    style: "grid" as const,
    fields: [
      {
        component: "input-text" as const,
        id: "project-name",
        labelProps: { label: "Nome do projecto", for: "project-name" },
        inputProps: { id: "project-name", name: "projectName", type: "text" as const, placeholder: "O seu projecto" },
      },
      {
        component: "input-email" as const,
        id: "contact-email",
        labelProps: { label: "Email de contacto", for: "contact-email" },
        inputProps: { id: "contact-email", name: "email", type: "email" as const, placeholder: "nome@exemplo.com" },
      },
    ],
  };
</script>

<div class="flex min-h-48 items-center justify-center rounded-lg border border-blender-border bg-blender-base p-6">
  <Button text="Abrir formulário" onClick={() => (open = true)} />
  <ModalForm
    {open}
    title="Novo projecto"
    {form}
    onClose={() => (open = false)}
  />
</div>
