import type { PageProductDetailsProps } from "../../../../project/ui/pages/product-details/data/types";

export const minimal: PageProductDetailsProps = {
  variant: "minimal",
  menuProps: {
    logoText: "VELVETY",
    logoSubtitle: "Facial & skincare",
    menus: [
      { textProps: { text: "PAGES" }, href: "#pages" },
      { textProps: { text: "SHOP" }, href: "#shop" },
      { textProps: { text: "ABOUT" }, href: "#about" },
      { textProps: { text: "LOGIN" }, href: "#login" },
      { textProps: { text: "CART (0)" }, href: "#cart" },
    ],
  },
  breadcrumbProps: {
    items: [
      { label: "Home", href: "#home" },
      { label: "Shop", href: "#shop" },
      { label: "Chicori", active: true },
    ],
  },
  productGalleryProps: {
    layout: "bottom-thumbs",
    imageProps: { src: "/img/programmer-01.jpg", alt: "Chicori" },
    minFileProps: [
      { imageProps: { src: "/img/programmer-01.jpg", alt: "Thumb 1" }, active: true },
      { imageProps: { src: "/img/programmer-02.jpg", alt: "Thumb 2" } },
      { imageProps: { src: "/img/ux-01.jpg", alt: "Thumb 3" } },
      { imageProps: { src: "/img/programmer-01.jpg", alt: "Thumb 4" } },
      { imageProps: { src: "/img/programmer-02.jpg", alt: "Thumb 5" } },
    ],
  },
  productInfoProps: {
    stockBadge: "Best Seller",
    titleProps: { title: "CHICORI" },
    subtitle: "Protect  |  Size: 50 ml",
    ratingProps: { rating: 5 },
    reviewCountText: "4.6",
    priceProps: { number: 20 },
    priceUnit: "Or 4 easy payments of $5.00",
    socialProofText: "89 people have purchased this product",
    showQuantity: true,
    stockCountText: "Current stock: 27",
    purchaseOptions: [
      { id: "once", label: "One time purchase", price: "$20" },
      { id: "sub", label: "Subscribe 30 days (Save 20%)", price: "$16" },
    ],
    buttonProps: { text: "Add to cart" },
    secondaryButtonProps: { text: "Favorite" },
    inlineTabsProps: {
      underlineNavProps: {
        items: [
          { value: "how", label: "How to use" },
          { value: "benefit", label: "Benefit" },
          { value: "ingredients", label: "Ingredients" },
          { value: "return", label: "Return Policy" },
        ],
        value: "how",
      },
      paragraphs: [
        "We always recommend speaking to a specialist before starting any new regimen so you receive tailored guidance for your skin goals.",
        "Apply 2 drops in the morning and 2 drops in the evening on clean skin before moisturizer.",
      ],
    },
  },
  productSimiliarProps: {
    variant: "carousel",
    titleProps: { title: "You might also like" },
    items: [
      { variant: "01", imageProps: { src: "/img/programmer-01.jpg", alt: "Opulent" }, titleProps: { title: "OPULENT" }, priceProps: { number: 19 }, ratingProps: { rating: 5 } },
      { variant: "01", imageProps: { src: "/img/programmer-02.jpg", alt: "Grace" }, titleProps: { title: "GRACE" }, priceProps: { number: 13 }, ratingProps: { rating: 4 } },
      { variant: "01", imageProps: { src: "/img/ux-01.jpg", alt: "Velvet" }, titleProps: { title: "VELVET" }, priceProps: { number: 23 }, ratingProps: { rating: 5 } },
      { variant: "01", imageProps: { src: "/img/programmer-01.jpg", alt: "Iridescent" }, titleProps: { title: "IRIDESCENT" }, priceProps: { number: 25 }, ratingProps: { rating: 4 } },
    ],
    moreButtonProps: { text: "More products →" },
  },
  footerProps: {
    variant: "02",
    logoText: "VELVETY",
    logoSubtitle: "Facial & skincare",
    openingHours: ["Opening hours", "Monday to Saturday:", "10:30 a.m. to 7 p.m."],
    newsletterProps: {
      bannerTitle: "Subscribe to get 10% off your first order",
      placeholder: "Drop your email here",
      buttonProps: { text: "→" },
    },
    columns: [
      { title: "Shop", links: [{ label: "Skincare" }, { label: "Facial" }, { label: "Soap" }, { label: "Candles" }, { label: "Gifts" }] },
      { title: "Help Desk", links: [{ label: "Chat" }, { label: "FAQ" }, { label: "Shipping & Returns" }, { label: "Contact" }, { label: "Policies" }] },
      { title: "Stores", links: [{ label: "Manhattan" }, { label: "Brooklyn" }, { label: "Tokyo" }, { label: "Jakarta" }, { label: "Paris" }] },
    ],
    bottomLinks: [{ label: "Licenses" }, { label: "Privacy" }, { label: "Terms" }],
  },
};
