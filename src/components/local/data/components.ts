import { IconNamed, type IconNamed as IconName } from "../../../hooks/icon.svelte";

export type ComponentDefinition = {
  id: string;
  name: string;
  label?: string;
  category: "Input" | "Typography";
  href?: string;
  sourceDirectory?: string;
  dependence?: string;
  icon: IconName;
  group: string;
  description: string;
};

export type ComponentNavigationGroup = {
  label: string;
  items: ComponentDefinition[];
  open?: boolean;
};

export const componentCatalog: ComponentDefinition[] = [
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

const inputComponents = componentCatalog.filter((component) => component.category === "Input");
const typographyComponents = componentCatalog.filter((component) => component.category === "Typography");

export const componentNavigationGroups: ComponentNavigationGroup[] = [
  {
    label: "Input",
    open: true,
    items: inputComponents,
  },
  {
    label: "Controls",
    open: false,
    items: [
      { id: "textarea", name: "Textarea", category: "Input", icon: IconNamed.Layout04, group: "Control", description: "Área de texto." },
      { id: "select", name: "Select", category: "Input", icon: IconNamed.CheckmarkSquare01, group: "Control", description: "Lista de opções." },
      { id: "toggle", name: "Toggle", category: "Input", icon: IconNamed.Settings01, group: "Control", description: "Controlo de alternância." },
      { id: "slider", name: "Slider", category: "Input", icon: IconNamed.Languages, group: "Control", description: "Controlo deslizante." },
      { id: "rating", name: "Rating", category: "Input", icon: IconNamed.HelpSquare, group: "Control", description: "Classificação." },
    ],
  },
  {
    label: "Typography",
    open: false,
    items: [
      ...typographyComponents,
      { id: "heading", name: "Heading", category: "Typography", icon: IconNamed.DocumentValidation, group: "Atom", description: "Título." },
      { id: "text-typography", name: "Text", category: "Typography", icon: IconNamed.Layout04, group: "Atom", description: "Texto." },
      { id: "badge", name: "Badge", category: "Typography", icon: IconNamed.HelpSquare, group: "Atom", description: "Etiqueta compacta." },
    ],
  },
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
