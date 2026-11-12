export type TemplateScreenGroupType = "hero" | "authentication" | "product-details";

export type TemplateScreenGroup = {
  type: TemplateScreenGroupType;
  label: string;
  badge: string;
  description: string;
  count: number;
};

export const templateScreenNavigation: TemplateScreenGroup[] = [
  {
    type: "hero",
    label: "Hero",
    badge: "Template",
    description: "Páginas de destaque com menu, texto e grelha em mosaico.",
    count: 4,
  },
  {
    type: "authentication",
    label: "Autenticação",
    badge: "Template",
    description: "Fluxos de início de sessão, registo e verificação por OTP.",
    count: 6,
  },
  {
    type: "product-details",
    label: "Produto · Detalhes",
    badge: "Template",
    description: "Variações de página de detalhe de produto.",
    count: 7,
  },
];
