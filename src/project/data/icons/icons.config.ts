export type NegoIconEntry = {
  name: string;
  tag: string;
  svgPath: string;
};

const def = (name: string, hugeicon: string): NegoIconEntry => ({
  name,
  tag: `hgi-stroke hgi-${hugeicon}`,
  svgPath: `assets/hugeicons/svg/${hugeicon}.svg`,
});

/**
 * Unified Icon Configuration.
 * Web frameworks (Vue, React, Svelte) use `.tag` (Hugeicons CSS classes).
 * Flutter uses `.svgPath` to load the SVG asset.
 */
export const NegoIconsConfig = {
  Age: def("age", "user-02"),
  ArrowUp: def("arrow-up", "arrow-up-01"),
  ArrowDown: def("arrow-down", "arrow-down-01"),
  ArrowDown01: def("arrow-down-01", "arrow-down-01"),
  ArrowLeft: def("square-chevron-left", "square-chevron-left"),
  ArrowRight: def("square-chevron-right", "square-chevron-right"),
  Cancel: def("cancel", "cancel-01"),
  Calendar: def("calendar", "calendar-03"),
  Cart: def("cart", "shopping-cart-add-01"),
  CalendarDays: def("calendar-days", "calendar-days"),
  CheckmarkSquare01: def("checkmark", "checkmark-square-01"),
  Date: def("date", "calendar-03"),
  DocumentValidation: def("document", "document-validation"),
  DragDropVertical: def("drag-drop-vertical", "drag-drop-vertical"),
  Email: def("email", "mail-01"),
  FilterHorizontal: def("filter-horizontal", "filter-horizontal"),
  Favorite: def("favourite", "favourite"),
  FolderAdd: def("folder-add", "folder-add"),
  Home04: def("home", "home-04"),
  HelpSquare: def("help", "help-square"),
  Image02: def("image-02", "image-02"),
  InputCursorText: def("input-cursor-text", "input-cursor-text"),
  Label: def("label", "tag-01"),
  Languages: def("languages", "languages"),
  Layout04: def("layout", "layout-04"),
  LayoutThreeColumn: def("layout-three-column", "layout-three-column"),
  Link: def("link", "link-01"),
  Location: def("location", "location-01"),
  Message: def("message", "chat-dot-01"),
  MoreHorizontal: def("more-horizontal", "more-horizontal"),
  MoreVertical: def("more-vertical", "more-vertical"),
  Number: def("number", "square-01"),
  Password: def("password", "security-password"),
  Phone: def("phone", "call"),
  Play: def("play", "play"),
  Search01: def("search-01", "search-01"),
  Security: def("security", "security"),
  Settings01: def("settings", "settings-01"),
  Star: def("star", "star"),
  Text: def("text", "text"),
  TextFont: def("text-font", "text-font"),
  Time: def("time", "clock-01"),
  User: def("user", "user"),
} as const;

export type NegoIconKey = keyof typeof NegoIconsConfig;

export function resolveIconClass(icon: NegoIconEntry | string): string {
  if (typeof icon !== "string") return icon.tag;
  const entry = (NegoIconsConfig as Record<string, NegoIconEntry | undefined>)[icon];
  return entry ? entry.tag : `hgi-stroke hgi-${icon}`;
}
