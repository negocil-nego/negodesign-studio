import SimpleSection from "../../../../templates/section/simple/ui/SimpleSection";
const fields = [
  { component: "input-text", labelProps: { label: "Nome", for: "register-name" }, inputProps: { id: "register-name", name: "name", type: "text", placeholder: "O seu nome", required: true } },
  { component: "input-email", labelProps: { label: "Email", for: "register-email" }, inputProps: { id: "register-email", name: "email", type: "email", placeholder: "nome@exemplo.com", required: true } },
  { component: "input-password", labelProps: { label: "Palavra-passe", for: "register-password" }, inputProps: { id: "register-password", name: "password", type: "password", placeholder: "Crie uma palavra-passe", required: true } },
];
export default function PageRegister() { return <SimpleSection sectionFormProps={{ isScreen: true, className: "flex items-center justify-center bg-blender-card px-6 py-8 text-zinc-100", titleProps: { text: "Criar conta" }, subtitleProps: { text: "Comece hoje" }, descriptionProps: { text: "Preencha os dados para se registar." }, formProps: { style: "inline", fields }, buttonSubmitProps: { text: "Criar conta" }, buttonsLoginProps: [{ text: "Já tem conta? Iniciar sessão", variant: "outline" }] }} />; }
