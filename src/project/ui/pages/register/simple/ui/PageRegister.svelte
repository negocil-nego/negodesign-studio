<script lang="ts">
  import SimpleSection from "../../../../templates/section/simple/ui/SimpleSection.svelte";

  const fields = [
    {
      component: "input-text" as const,
      labelProps: { label: "Nome", for: "register-name" },
      inputProps: {
        id: "register-name",
        name: "name",
        type: "text" as const,
        placeholder: "O seu nome",
        required: true,
      },
    },
    {
      component: "input-email" as const,
      labelProps: { label: "Email", for: "register-email" },
      inputProps: {
        id: "register-email",
        name: "email",
        type: "email" as const,
        placeholder: "nome@exemplo.com",
        required: true,
      },
    },
    {
      component: "input-password" as const,
      labelProps: { label: "Palavra-passe", for: "register-password" },
      inputProps: {
        id: "register-password",
        name: "password",
        type: "password" as const,
        placeholder: "Crie uma palavra-passe",
        required: true,
      },
    },
  ];
</script>

<SimpleSection
  sectionFormProps={{
    isScreen: true,
    className:
      "flex items-center justify-center bg-blender-card px-6 py-8 text-zinc-800 dark:text-zinc-100",
    titleProps: { text: "Criar conta" },
    subtitleProps: { text: "Comece hoje" },
    descriptionProps: { text: "Preencha os dados para se registar." },
    formProps: { style: "inline", fields },
    buttonSubmitProps: { text: "Criar conta" },
    buttonsLoginProps: [
      { text: "Já tem conta? Iniciar sessão", variant: "outline" },
    ],
  }}
/>
