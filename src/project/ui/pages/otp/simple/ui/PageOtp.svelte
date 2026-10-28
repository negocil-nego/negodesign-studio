<script lang="ts">
  import SimpleSection from "../../../../templates/section/simple/ui/SimpleSection.svelte";

  const fields = [
    {
      component: "input-otp" as const,
      id: "page-otp-code",
      name: "verification-code",
      value: "456789",
      length: 6,
      separatorEvery: 3,
      labelProps: { label: "Código de verificação", for: "page-otp-code" },
    },
  ];
</script>

<SimpleSection
  sectionFormProps={{
    isScreen: true,
    className:
      "flex items-center justify-center bg-blender-card px-6 py-8 text-zinc-800 dark:text-zinc-100",
    titleProps: { text: "Confirme o seu código" },
    subtitleProps: { text: "Verificação de segurança" },
    descriptionProps: {
      text: "Introduza o código de 6 dígitos enviado para o seu email.",
    },
    formProps: { fields },
    buttonSubmitProps: { text: "Verificar código" },
    buttonsLoginProps: [
      { text: "Não recebeu o código? Reenviar código", variant: "outline" },
    ],
  }}
/>
