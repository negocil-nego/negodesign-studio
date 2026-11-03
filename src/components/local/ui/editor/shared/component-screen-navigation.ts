export type ComponentScreenGroup = {
  label: string;
  badge: string;
  description: string;
  ids: string[];
  type: "input" | "card" | "menu";
};

export const componentScreenNavigation: ComponentScreenGroup[] = [
  {
    label: "Atoms",
    badge: "Atom",
    description: "Campos, controlos e elementos de texto reutilizáveis.",
    ids: [
      "input", "input-text", "input-username", "input-username-or-email",
      "input-username-or-phone", "input-otp", "input-email", "input-phone",
      "input-phone-flag", "input-label", "input-password", "input-date",
      "input-time", "input-number", "input-age", "select", "textarea",
      "toggle", "radio", "checkbox", "label", "country-popover",
    ],
    type: "input",
  },
  {
    label: "Molecules · Card",
    badge: "Molecule",
    description: "Cartões compostos por átomos e sub-cartões reutilizáveis.",
    ids: [
      "card-media", "card-profile", "card-product", "card", "card-image",
      "card-title", "card-description", "card-price", "card-rating",
      "card-tags", "card-logo", "card-phone", "card-video",
      "card-thumbnail-video", "card-min-file", "card-button",
      "card-cart-button", "card-favorite-button", "card-actions-button",
    ],
    type: "card",
  },
  {
    label: "Organisms · Menu",
    badge: "Organism",
    description: "Menus de navegação e variações de submenu.",
    ids: ["menu-simple", "submenu-grid-title", "menu-sidebar-grid"],
    type: "menu",
  },
];
