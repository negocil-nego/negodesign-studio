export type IconDefinition = {
  /** Semantic name used by components and icon pickers. */
  name: string;
  /** Hugeicons classes passed directly to an <i> element. */
  tag: string;
  /** Optional local SVG fallback in static/hugeicons/svg. */
  path?: string;
};

const icon = (name: string, hugeicons: string, path?: string): IconDefinition => ({
  name,
  tag: `hgi-stroke hgi-${hugeicons}`,
  ...(path ? { path } : {}),
});

/** Central list of icon definitions available to Negodesign components. */
export const IconNamed = {
  Age: icon("age", "user-02"),
  ArrowUp: icon("arrow-up", "arrow-up-01"),
  ArrowDown: icon("arrow-down", "arrow-down-01"),
  ArrowLeft: icon("square-chevron-left", "square-chevron-left"),
  ArrowRight: icon("square-chevron-right", "square-chevron-right"),
  Cancel: icon("cancel", "cancel-01"),
  Calendar: icon("calendar", "calendar-03"),
  Cart: icon("cart", "shopping-cart-add-01"),
  CalendarDays: icon("calendar-days", "calendar-days"),
  CheckmarkSquare01: icon("checkmark", "checkmark-square-01"),
  Date: icon("date", "calendar-03"),
  DocumentValidation: icon("document", "document-validation"),
  Email: icon("email", "mail-01"),
  InputCursorText: icon("input-cursor-text", "input-cursor-text"),
  Favorite: icon("favourite", "favourite"),
  FolderAdd: icon("folder-add", "folder-add"),
  Home04: icon("home", "home-04"),
  HelpSquare: icon("help", "help-square"),
  Link: icon("link", "link-01"),
  Label: icon("label", "tag-01"),
  Layout04: icon("layout", "layout-04"),
  Location: icon("location", "location-01"),
  Languages: icon("languages", "languages"),
  Message: icon("message", "chat-dot-01"),
  MoreHorizontal: icon("more-horizontal", "more-horizontal"),
  Number: icon("number", "square-01"),
  Star: icon("star", "star"),
  Security: icon("security", "security"),
  Settings01: icon("settings", "settings-01"),
  Play: icon("play", "play"),
  Phone: icon("phone", "call"),
  Password: icon("password", "security-password"),
  Time: icon("time", "clock-01"),
  TextFont: icon("text-font", "text-font"),
  Text: icon("text", "text"),
  User: icon("user", "user"),
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
  cart: "shopping-cart-add-01",
  favourite: "favourite",
  star: "star",
  play: "play",
  calendar: "calendar-03",
  arrowLeft: "square-chevron-left",
  arrowRight: "square-chevron-right",
  moreHorizontal: "more-horizontal",
  arrowDown: "arrow-down-01",
  arrowUp: "arrow-up-01",
};

/** Accepts either a registry entry or a Hugeicons name. */
export function getIconClass(icon: IconNamed | string): string {
  if (typeof icon !== "string") return icon.tag;
  return `hgi-stroke hgi-${hugeiconsNames[icon] ?? icon}`;
}
