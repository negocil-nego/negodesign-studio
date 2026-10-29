<script lang="ts">
  import SimpleSection from "../../../../templates/section/simple/ui/SimpleSection.svelte";

  const fields = [
    {
      component: "input-email" as const,
      labelProps: { label: "Email", for: "login-email" },
      inputProps: {
        id: "login-email",
        name: "email",
        type: "email" as const,
        placeholder: "nome@exemplo.com",
        required: true,
      },
    },
    {
      component: "input-password" as const,
      labelProps: { label: "Palavra-passe", for: "login-password" },
      inputProps: {
        id: "login-password",
        name: "password",
        type: "password" as const,
        placeholder: "A sua palavra-passe",
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
    titleProps: { text: "Iniciar sessão" },
    subtitleProps: { text: "Bem-vindo de volta" },
    descriptionProps: { text: "Aceda à sua conta para continuar." },
    formProps: { style: "inline", fields },
    buttonSubmitProps: { text: "Entrar" },
    buttonsLoginProps: [
      { text: "Ainda não tem conta? Criar conta", variant: "outline" },
    ],
  }}
/>
