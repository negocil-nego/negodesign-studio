export type ComponentScreenGroup = {
  label: string;
  badge: string;
  description: string;
  ids: string[];
  type: "input" | "card" | "menu" | "sidebar" | "data-table" | "carousel" | "grid" | "grid-carousel";
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
      "toggle", "radio", "checkbox", "input-range", "radio-button",
      "block-button", "progress", "pagination-nav", "breadcrumbs",
      "underline-nav", "indicators", "containers", "list-groups",
      "label", "country-popover",
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
  {
    label: "Organisms · Sidebar",
    badge: "Organism",
    description: "Sidebars de navegação com secções, temas e modos compactos.",
    ids: ["sidebar"],
    type: "sidebar",
  },
  {
    label: "Organisms · Data Table",
    badge: "Organism",
    description: "Tabelas de dados com ordenação, estados, acções e paginação.",
    ids: ["data-table"],
    type: "data-table",
  },
  {
    label: "Organisms · Carousel",
    badge: "Organism",
    description: "Carousels de cartões para media, produtos e perfis.",
    ids: [
      "section-carousel-buttons",
      "section-carousel",
      "carousel-card-media",
      "carousel-card-product",
      "carousel-card-profile",
    ],
    type: "carousel",
  },
  {
    label: "Organisms · Grid",
    badge: "Organism",
    description: "Cartões apresentados numa grelha responsiva.",
    ids: ["section-grid", "grid-card-media", "grid-card-product", "grid-card-profile"],
    type: "grid",
  },
  {
    label: "Organisms · Grid Carousel",
    badge: "Organism",
    description: "Cartões em grupos de linhas e colunas com navegação horizontal.",
    ids: [
      "section-grid-carousel",
      "grid-carousel-card-media",
      "grid-carousel-card-product",
      "grid-carousel-card-profile",
    ],
    type: "grid-carousel",
  },
];




