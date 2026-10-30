import SimpleSection from "../../../../templates/section/simple/ui/SimpleSection";
const fields = [
  { component: "input-email", labelProps: { label: "Email", for: "login-email" }, inputProps: { id: "login-email", name: "email", type: "email", placeholder: "nome@exemplo.com", required: true } },
  { component: "input-password", labelProps: { label: "Palavra-passe", for: "login-password" }, inputProps: { id: "login-password", name: "password", type: "password", placeholder: "A sua palavra-passe", required: true } },
];
export default function PageLogin() { return <SimpleSection sectionFormProps={{ isScreen: true, className: "flex items-center justify-center bg-blender-card px-6 py-8 text-zinc-100", titleProps: { text: "Iniciar sessão" }, subtitleProps: { text: "Bem-vindo de volta" }, descriptionProps: { text: "Aceda à sua conta para continuar." }, formProps: { style: "inline", fields }, buttonSubmitProps: { text: "Entrar" }, buttonsLoginProps: [{ text: "Ainda não tem conta? Criar conta", variant: "outline" }] }} />; }
