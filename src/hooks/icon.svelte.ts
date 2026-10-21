export type IconDefinition = {
  /** Semantic name used by components and icon pickers. */
  name: string;
  /** Hugeicons classes passed directly to an <i> element. */
  tag: string;
  /** Optional local SVG fallback in static/hugeicons/svg. */
  path?: string;
};

const icon = (name: string, hugeicons: string, file?: string): IconDefinition => ({
  name,
  tag: `hgi-stroke hgi-${hugeicons}`,
  ...(file ? { path: `/hugeicons/svg/${file}.svg` } : {}),
});

/** Central list of icon definitions available to Negodesign components. */
export const IconNamed = {
  FolderAdd: icon("folder-add", "folder-add", "folder-add-stroke-rounded"),
  Layout04: icon("layout", "layout-04", "layout-04-stroke-rounded"),
  Settings01: icon("settings", "settings-01", "settings-01-stroke-rounded"),
  DocumentValidation: icon("document", "document-validation", "document-validation-stroke-rounded"),
  HelpSquare: icon("help", "help-square", "help-square-stroke-rounded"),
  CheckmarkSquare01: icon("checkmark", "checkmark-square-01", "checkmark-square-01-stroke-rounded"),
  Languages: icon("languages", "languages", "languages-stroke-rounded"),
  Security: icon("security", "security", "security-stroke-rounded"),
  Home: icon("home", "home-04", "home-04-stroke-rounded"),
  Email: icon("email", "mail-01"),
  Phone: icon("phone", "call"),
  Password: icon("password", "security-password"),
  Date: icon("date", "calendar-03"),
  Time: icon("time", "clock-01"),
  Number: icon("number", "square-01"),
  Age: icon("age", "user-02"),
  Text: icon("text", "text-font"),
  Label: icon("label", "tag-01"),
} as const satisfies Record<string, IconDefinition>;

export type IconNamed = (typeof IconNamed)[keyof typeof IconNamed];

const hugeiconsNames: Record<string, string> = {
  email: "mail-01",
  phone: "call",
  password: "security-password",
  date: "calendar-03",
  time: "clock-01",
  number: "square-01",
  age: "user-02",
  text: "text-font",
  label: "tag-01",
};

/** Accepts either a registry entry or a Hugeicons name. */
export function getIconClass(icon: IconNamed | string): string {
  if (typeof icon !== "string") return icon.tag;
  return `hgi-stroke hgi-${hugeiconsNames[icon] ?? icon}`;
}
