import { IconNamed, type IconNamed as IconName } from "../../../hooks/icon.svelte";

export type ComponentDefinition = {
  id: string;
  name: string;
  label?: string;
  category: "Input" | "Typography" | "Control";
  href?: string;
  sourceDirectory?: string;
  dependence?: string;
  icon: IconName;
  group: string;
  description: string;
};

export type ComponentNavigationGroup = {
  label: string;
  open?: boolean;
  items: ComponentDefinition[];
};

// ─── Catalog ────────────────────────────────────────────────────────────────

export const componentCatalog: ComponentDefinition[] = [
  // ── Atoms · Input ─────────────────────────────────────────────────────────
  {
    id: "input",
    name: "Input",
    label: "Input",
    category: "Input",
    href: "/main/project/input#input-text",
    sourceDirectory: "project/ui/atoms/input/core",
    dependence: "src/project/ui/atoms/input/core/dependence.json",
    icon: IconNamed.Text,
    group: "Atom",
    description: "Campo de entrada base.",
  },
  {
    id: "input-text",
    name: "InputText",
    label: "Text",
    category: "Input",
    href: "/main/project/input#input-text",
    sourceDirectory: "project/ui/atoms/input/children/text",
    dependence: "src/project/ui/atoms/input/children/text/dependence.json",
    icon: IconNamed.Text,
    group: "Atom · Input",
    description: "Campo de texto simples.",
  },
  {
    id: "input-email",
    name: "InputEmail",
    label: "Email",
    category: "Input",
    href: "/main/project/input#input-email",
    sourceDirectory: "project/ui/atoms/input/children/email",
    dependence: "src/project/ui/atoms/input/children/email/dependence.json",
    icon: IconNamed.Email,
    group: "Atom · Input",
    description: "Campo para endereços de email.",
  },
  {
    id: "input-phone",
    name: "InputPhone",
    label: "Phone",
    category: "Input",
    href: "/main/project/input#input-phone",
    sourceDirectory: "project/ui/atoms/input/children/phone",
    dependence: "src/project/ui/atoms/input/children/phone/dependence.json",
    icon: IconNamed.Phone,
    group: "Atom · Input",
    description: "Campo para números de telefone.",
  },
  {
    id: "input-phone-flag",
    name: "InputPhoneFlag",
    label: "Phone with country code",
    category: "Input",
    href: "/main/project/input#input-phone-flag",
    sourceDirectory: "project/ui/atoms/input/children/phone",
    dependence: "src/project/ui/atoms/input/children/phone/dependence.json",
    icon: IconNamed.Phone,
    group: "Atom · Input",
    description: "Telefone com selector de país.",
  },
  {
    id: "input-label",
    name: "InputLabel",
    label: "Input with label",
    category: "Input",
    href: "/main/project/input#input-text",
    sourceDirectory: "project/ui/atoms/input/children/label",
    dependence: "src/project/ui/atoms/input/children/label/dependence.json",
    icon: IconNamed.Label,
    group: "Atom · Input",
    description: "Campo de entrada associado a um rótulo.",
  },
  {
    id: "input-password",
    name: "InputPassword",
    label: "Password",
    category: "Input",
    href: "/main/project/input#input-password",
    sourceDirectory: "project/ui/atoms/input/children/password",
    dependence: "src/project/ui/atoms/input/children/password/dependence.json",
    icon: IconNamed.Password,
    group: "Atom · Input",
    description: "Campo para palavras-passe.",
  },
  {
    id: "input-date",
    name: "InputDate",
    label: "Date",
    category: "Input",
    href: "/main/project/input#input-date",
    sourceDirectory: "project/ui/atoms/input/children/date",
    dependence: "src/project/ui/atoms/input/children/date/dependence.json",
    icon: IconNamed.Date,
    group: "Atom · Input",
    description: "Campo para seleccionar uma data.",
  },
  {
    id: "input-time",
    name: "InputTime",
    label: "Time",
    category: "Input",
    href: "/main/project/input#input-time",
    sourceDirectory: "project/ui/atoms/input/children/time",
    dependence: "src/project/ui/atoms/input/children/time/dependence.json",
    icon: IconNamed.Time,
    group: "Atom · Input",
    description: "Campo para seleccionar uma hora.",
  },
  {
    id: "input-number",
    name: "InputNumber",
    label: "Number",
    category: "Input",
    href: "/main/project/input#input-number",
    sourceDirectory: "project/ui/atoms/input/children/number",
    dependence: "src/project/ui/atoms/input/children/number/dependence.json",
    icon: IconNamed.Number,
    group: "Atom · Input",
    description: "Campo para valores numéricos.",
  },
  {
    id: "input-age",
    name: "InputAge",
    label: "Age",
    category: "Input",
    href: "/main/project/input#input-age",
    sourceDirectory: "project/ui/atoms/input/children/age",
    dependence: "src/project/ui/atoms/input/children/age/dependence.json",
    icon: IconNamed.Age,
    group: "Atom · Input",
    description: "Campo numérico para idade.",
  },
  // ── Atoms · Control ───────────────────────────────────────────────────────
  {
    id: "select",
    name: "Select",
    label: "Select",
    category: "Control",
    href: "/main/project/controls#select",
    sourceDirectory: "project/ui/atoms/select",
    dependence: "src/project/ui/atoms/select/dependence.json",
    icon: IconNamed.CheckmarkSquare01,
    group: "Atom · Control",
    description: "Lista de opções suspensa.",
  },
  {
    id: "textarea",
    name: "Textarea",
    label: "Textarea",
    category: "Control",
    href: "/main/project/controls#textarea",
    sourceDirectory: "project/ui/atoms/textarea",
    dependence: "src/project/ui/atoms/textarea/dependence.json",
    icon: IconNamed.Layout04,
    group: "Atom · Control",
    description: "Área de texto multi-linha.",
  },
  {
    id: "toggle",
    name: "Toggle",
    label: "Toggle",
    category: "Control",
    href: "/main/project/controls#toggle",
    sourceDirectory: "project/ui/atoms/toggle",
    dependence: "src/project/ui/atoms/toggle/dependence.json",
    icon: IconNamed.Settings01,
    group: "Atom · Control",
    description: "Interruptor de alternância.",
  },
  {
    id: "radio",
    name: "Radio",
    label: "Radio",
    category: "Control",
    href: "/main/project/controls#radio",
    sourceDirectory: "project/ui/atoms/radio",
    dependence: "src/project/ui/atoms/radio/dependence.json",
    icon: IconNamed.HelpSquare,
    group: "Atom · Control",
    description: "Grupo de selecção exclusiva.",
  },
  {
    id: "checkbox",
    name: "Checkbox",
    label: "Checkbox",
    category: "Control",
    href: "/main/project/controls#checkbox",
    sourceDirectory: "project/ui/atoms/checkbox",
    dependence: "src/project/ui/atoms/checkbox/dependence.json",
    icon: IconNamed.CheckmarkSquare01,
    group: "Atom · Control",
    description: "Caixa de verificação.",
  },
  // ── Atoms · Typography ────────────────────────────────────────────────────
  {
    id: "label",
    name: "Label",
    label: "Label",
    category: "Typography",
    href: "/main/project/label#label",
    sourceDirectory: "project/ui/atoms/label",
    dependence: "src/project/ui/atoms/label/dependence.json",
    icon: IconNamed.CheckmarkSquare01,
    group: "Atom",
    description: "Rótulo associado a um controlo.",
  },
  // ── Molecules ─────────────────────────────────────────────────────────────
  {
    id: "country-popover",
    name: "CountryPhonePopover",
    label: "Country phone selector",
    category: "Input",
    href: "/main/project/input#input-phone-flag",
    sourceDirectory: "project/ui/molecules/popover",
    dependence: "src/project/ui/molecules/popover/dependence.json",
    icon: IconNamed.Home,
    group: "Molecule",
    description: "Selector de país e indicativo telefónico.",
  },
];

// ─── Navigation groups (Atomic Design) ──────────────────────────────────────

const byCategory = (cat: ComponentDefinition["category"]) =>
  componentCatalog.filter((c) => c.category === cat);

const byGroup = (group: string) =>
  componentCatalog.filter((c) => c.group === group);

export const componentNavigationGroups: ComponentNavigationGroup[] = [
  // ── Atoms ─────────────────────────────────────────────────────────────────
  {
    label: "Atoms · Input",
    open: true,
    items: componentCatalog.filter((c) => c.group === "Atom · Input" || (c.group === "Atom" && c.category === "Input")),
  },
  {
    label: "Atoms · Control",
    open: false,
    items: componentCatalog.filter((c) => c.group === "Atom · Control"),
  },
  {
    label: "Atoms · Typography",
    open: false,
    items: componentCatalog.filter((c) => c.category === "Typography" && c.group === "Atom"),
  },
  // ── Molecules ─────────────────────────────────────────────────────────────
  {
    label: "Molecules",
    open: false,
    items: componentCatalog.filter((c) => c.group === "Molecule"),
  },
  // ── Organisms (placeholder) ───────────────────────────────────────────────
  {
    label: "Organisms",
    open: false,
    items: [
      { id: "form", name: "Form", category: "Input", icon: IconNamed.Layout04, group: "Organism", description: "Formulário composto por múltiplos campos." },
      { id: "card", name: "Card", category: "Input", icon: IconNamed.DocumentValidation, group: "Organism", description: "Cartão de conteúdo reutilizável." },
      { id: "modal", name: "Modal", category: "Input", icon: IconNamed.HelpSquare, group: "Organism", description: "Caixa de diálogo sobreposta ao conteúdo." },
    ],
  },
  // ── Templates (placeholder) ───────────────────────────────────────────────
  {
    label: "Templates",
    open: false,
    items: [
      { id: "auth-page", name: "Auth Page", category: "Input", icon: IconNamed.Security, group: "Template", description: "Página de autenticação." },
      { id: "dashboard", name: "Dashboard", category: "Input", icon: IconNamed.Layout04, group: "Template", description: "Painel de controlo." },
    ],
  },
  // ── Media ─────────────────────────────────────────────────────────────────
  {
    label: "Media",
    open: false,
    items: [
      { id: "image", name: "Image", category: "Input", icon: IconNamed.Layout04, group: "Media", description: "Imagem." },
      { id: "icon", name: "Icon", category: "Input", icon: IconNamed.HelpSquare, group: "Media", description: "Ícone." },
      { id: "avatar", name: "Avatar", category: "Input", icon: IconNamed.Home, group: "Media", description: "Avatar." },
      { id: "video", name: "Video", category: "Input", icon: IconNamed.DocumentValidation, group: "Media", description: "Vídeo." },
    ],
  },
  // ── Navigation ────────────────────────────────────────────────────────────
  {
    label: "Navigation",
    open: false,
    items: [
      { id: "navbar", name: "Navbar", category: "Input", icon: IconNamed.Layout04, group: "Navigation", description: "Barra de navegação." },
      { id: "sidebar", name: "Sidebar", category: "Input", icon: IconNamed.FolderAdd, group: "Navigation", description: "Barra lateral." },
      { id: "tabs", name: "Tabs", category: "Input", icon: IconNamed.Languages, group: "Navigation", description: "Separadores." },
      { id: "breadcrumb", name: "Breadcrumb", category: "Input", icon: IconNamed.DocumentValidation, group: "Navigation", description: "Trilho de navegação." },
    ],
  },
];
