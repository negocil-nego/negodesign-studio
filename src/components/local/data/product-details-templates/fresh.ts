import type { PageProductDetailsProps } from "../../../../project/ui/pages/product-details/data/types";

export const fresh: PageProductDetailsProps = {
  variant: "fresh",
  backButtonText: "Back",
  menuProps: {
    logoText: "AgriProducts",
    menus: [
      { textProps: { text: "Home" }, href: "#home" },
      { textProps: { text: "Categories" }, href: "#categories" },
      { textProps: { text: "How it works" }, href: "#how" },
      { textProps: { text: "Featured Sellers" }, href: "#sellers" },
      { textProps: { text: "About Us" }, href: "#about" },
      { textProps: { text: "Contact" }, href: "#contact" },
    ],
  },
  breadcrumbProps: {
    items: [
      { label: "Home", href: "#home" },
      { label: "Categories", href: "#categories" },
      { label: "Vegetables", active: true },
    ],
  },
  productGalleryProps: {
    layout: "bottom-thumbs",
    imageProps: { src: "/img/programmer-01.jpg", alt: "Fresh Broccoli" },
    minFileProps: [
      { imageProps: { src: "/img/programmer-01.jpg", alt: "Broccoli 1" }, active: true },
      { imageProps: { src: "/img/programmer-02.jpg", alt: "Broccoli 2" } },
      { imageProps: { src: "/img/ux-01.jpg", alt: "Broccoli 3" } },
    ],
  },
  productInfoProps: {
    badge: "Organic Certified",
    titleProps: { title: "Broccoli" },
    ratingProps: { rating: 5 },
    reviewCountText: "4.8 (126 reviews)",
    priceProps: { number: 2.5 },
    priceUnit: "/500g",
    sizesLabel: "Size",
    sizes: [
      { label: "500g", value: "500g", priceLabel: "$2.50" },
      { label: "1kg", value: "1kg", priceLabel: "$5.00" },
      { label: "2kg", value: "2kg", priceLabel: "$10.00" },
      { label: "5kg", value: "5kg", priceLabel: "$24.00" },
    ],
    showQuantity: true,
    subtotalText: "$2.50",
    deliveryText: "Delivered: Wednesday, Aug 27",
    urgencyText: "Only 6 items left",
    buttonProps: { text: "Add to cart" },
    favoriteButtonProps: {},
    trustBadges: [
      { icon: "User01", text: "Direct from our farm" },
      { icon: "Clock01", text: "24H Delivery" },
      { icon: "Sparkles", text: "100% Organic" },
    ],
  },
  productTabsProps: {
    underlineNavProps: {
      items: [
        { value: "description", label: "Description" },
        { value: "nutrition", label: "Nutrition" },
        { value: "reviews", label: "Reviews" },
      ],
    },
    descriptionProps: {
      description: "Crisp, nutrient-rich broccoli harvested fresh from our farms and delivered directly to your door. Packed with vitamins C, K, and fiber, it's the perfect choice for healthy meals.",
    },
    highlights: ["100% Organic", "Harvested within 24 hours", "Great for boosting immunity & digestion"],
  },
  productRatingProps: undefined,
  productSimiliarProps: {
    variant: "grid",
    titleProps: { title: "You may also like" },
    columns: 4,
    items: [
      { variant: "01", imageProps: { src: "/img/programmer-02.jpg", alt: "Tomatoes" }, titleProps: { title: "Tomatoes" }, descriptionProps: { description: "New Arrival · 2 days Left" }, priceProps: { number: 2.2 }, ratingProps: { rating: 5 }, cartButtonProps: {} },
      { variant: "01", imageProps: { src: "/img/programmer-01.jpg", alt: "Cucumber" }, titleProps: { title: "Cucumber" }, descriptionProps: { description: "Best Seller · 2 days Left" }, priceProps: { number: 1.5 }, ratingProps: { rating: 5 }, cartButtonProps: {} },
      { variant: "01", imageProps: { src: "/img/ux-01.jpg", alt: "Eggplant" }, titleProps: { title: "Eggplant" }, descriptionProps: { description: "Best Seller · 2 days Left" }, priceProps: { number: 2.5 }, ratingProps: { rating: 5 }, cartButtonProps: {} },
      { variant: "01", imageProps: { src: "/img/programmer-02.jpg", alt: "Cauliflower" }, titleProps: { title: "Cauliflower" }, descriptionProps: { description: "New Arrival · 2 days Left" }, priceProps: { number: 2.5 }, ratingProps: { rating: 5 }, cartButtonProps: {} },
    ],
  },
};
