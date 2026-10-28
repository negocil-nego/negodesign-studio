<script lang="ts">
  import DualSection from "../../../../templates/section/dual-section/ui/DualSection.svelte";

  const fields = [
    {
      component: "input-otp" as const,
      id: "page-otp-dual-code",
      name: "verification-code",
      value: "456789",
      length: 6,
      separatorEvery: 3,
      labelProps: { label: "Código de verificação", for: "page-otp-dual-code" },
    },
  ];
</script>

<DualSection
  leftSectionForm={{
    isScreen: true,
    className: "flex items-center justify-center bg-blue-600 px-8 py-10 text-white",
    titleProps: { text: "Conta protegida" },
    subtitleProps: { text: "Mais um passo para continuar." },
    descriptionProps: { text: "A verificação em duas etapas ajuda a manter a sua conta segura." },
  }}
  rightSectionForm={{
    isScreen: true,
    className: "flex items-center justify-center bg-blender-card px-6 py-8 text-zinc-800 dark:text-zinc-100",
    titleProps: { text: "Introduza o código" },
    subtitleProps: { text: "Verificação de segurança" },
    descriptionProps: { text: "Enviámos um código de 6 dígitos para o seu email." },
    formProps: { fields },
    buttonSubmitProps: { text: "Confirmar código" },
    buttonsLoginProps: [{ text: "Não recebeu? Enviar novamente", variant: "outline" }],
  }}
/>
