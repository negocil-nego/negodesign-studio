import type { PageProductDetailsProps } from "../../../../project/ui/pages/product-details/data/types";

export const catalogSpecs: PageProductDetailsProps = {
  variant: "catalog",
  menuProps: {
    logoText: "ROZETKA",
    menus: [
      { textProps: { text: "Catalog" }, href: "#catalog" },
      { textProps: { text: "Sneakers" }, href: "#sneakers" },
      { textProps: { text: "Brands" }, href: "#brands" },
      { textProps: { text: "Deals" }, href: "#deals" },
    ],
  },
  breadcrumbProps: {
    items: [
      { label: "Catalog", href: "#catalog" },
      { label: "Footwear", href: "#footwear" },
      { label: "Nike Air Jordan 1 Mid Green", active: true },
    ],
  },
  productGalleryProps: {
    imageProps: { src: "/img/ux-01.jpg", alt: "Air Jordan 1 Mid Green" },
    minFileProps: [
      { imageProps: { src: "/img/ux-01.jpg", alt: "Side View" }, active: true },
      { imageProps: { src: "/img/programmer-01.jpg", alt: "Top View" } },
      { imageProps: { src: "/img/programmer-02.jpg", alt: "Pair View" } },
      { imageProps: { src: "/img/ux-01.jpg", alt: "Heel View" } },
    ],
  },
  productInfoProps: {
    stockBadge: "In Stock",
    titleProps: { title: "Sneakers Nike Air Jordan 1 Mid Green" },
    ratingProps: { rating: 5 },
    reviewCountText: "58 reviews",
    colorsLabel: "Color",
    colors: [
      { label: "Pine Green", value: "green", hex: "#16a34a" },
      { label: "Obsidian", value: "black", hex: "#18181b" },
    ],
    sizesLabel: "EU Size",
    sizes: [{ label: "40.5", value: "40.5" }, { label: "41", value: "41" }, { label: "42", value: "42" }, { label: "42.5", value: "42.5" }, { label: "43", value: "43" }],
    priceProps: { number: 3065 },
    buttonProps: { text: "Add to Basket" },
    secondaryButtonProps: { text: "Buy with Credit" },
    favoriteButtonProps: {},
  },
  productTabsProps: {
    specsStyle: "table",
    underlineNavProps: {
      items: [
        { value: "all", label: "Overview" },
        { value: "specs", label: "Specifications" },
        { value: "reviews", label: "Reviews (58)" },
        { value: "media", label: "Photos" },
      ],
    },
    titleProps: { title: "Specifications & Description" },
    specs: [
      { label: "Purpose", value: "Everyday / Basketball" },
      { label: "Upper Material", value: "100% Genuine Leather & Synthetic" },
      { label: "Lining Material", value: "100% Textile Mesh" },
      { label: "Outsole", value: "Rubber Cupsole with Air-Sole unit" },
      { label: "SKU Article", value: "554724-067" },
    ],
    descriptionProps: {
      description: "Inspired by the original AJ1, the Air Jordan 1 Mid offers fans a chance to follow in the footsteps of greatness with classic court paneling and fresh color blocking.",
    },
  },
  productRatingProps: {
    titleProps: { title: "Customer Reviews (58)" },
    averageScore: "4.9/5",
    totalReviewsText: "58 verified buyers",
    reviews: [
      {
        id: "rz-1", author: "Daniel K.", date: "19 June 2026", ratingProps: { rating: 5 },
        descriptionProps: { description: "Authentic pair, leather quality is soft and durable, and the green colorway looks great in person." },
      },
      {
        id: "rz-2", author: "Elena M.", date: "12 June 2026", ratingProps: { rating: 5 },
        descriptionProps: { description: "True to EU sizing. ankle support is very comfortable for long city walks." },
      },
    ],
  },
  productSimiliarProps: {
    variant: "grid",
    columns: 5,
    titleProps: { title: "Recently Viewed Products" },
    items: [
      { variant: "01", imageProps: { src: "/img/programmer-01.jpg", alt: "Item 1" }, titleProps: { title: "24\" Discovery Trail Bike" }, priceProps: { number: 3570 }, ratingProps: { rating: 5 } },
      { variant: "01", imageProps: { src: "/img/programmer-02.jpg", alt: "Item 2" }, titleProps: { title: "Inverter Welder SAB-258" }, priceProps: { number: 1659 }, ratingProps: { rating: 5 } },
      { variant: "01", imageProps: { src: "/img/ux-01.jpg", alt: "Item 3" }, titleProps: { title: "Sparkling Brut Reserve" }, priceProps: { number: 89 }, ratingProps: { rating: 4 } },
      { variant: "01", imageProps: { src: "/img/programmer-01.jpg", alt: "Item 4" }, titleProps: { title: "Reusable Sport Mask" }, priceProps: { number: 129 }, ratingProps: { rating: 5 } },
      { variant: "01", imageProps: { src: "/img/programmer-02.jpg", alt: "Item 5" }, titleProps: { title: "Wireless Optical Mouse" }, priceProps: { number: 279 }, ratingProps: { rating: 5 } },
    ],
  },
};
