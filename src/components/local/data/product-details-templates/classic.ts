import type { PageProductDetailsProps } from "../../../../project/ui/pages/product-details/data/types";

export const classic: PageProductDetailsProps = {
  variant: "classic",
  topBarText: "Call Us : +123-456-789",
  topBarRightText: "Sign up and GET 20% OFF for your first order",
  bannerTitle: "Shop",
  menuProps: {
    logoText: "Beauty Shop.",
    menus: [
      { textProps: { text: "Home" }, href: "#home" },
      { textProps: { text: "Shop" }, href: "#shop" },
      { textProps: { text: "Skin Care" }, href: "#skin" },
      { textProps: { text: "Makeup" }, href: "#makeup" },
      { textProps: { text: "Hair Care" }, href: "#hair" },
      { textProps: { text: "About Us" }, href: "#about" },
    ],
  },
  breadcrumbProps: {
    items: [
      { label: "Home", href: "#home" },
      { label: "Shop", href: "#shop" },
      { label: "Product Details", active: true },
    ],
  },
  productGalleryProps: {
    layout: "bottom-thumbs",
    showArrows: true,
    imageProps: { src: "/img/ux-01.jpg", alt: "SilkSkin Serum" },
    minFileProps: [
      { imageProps: { src: "/img/ux-01.jpg", alt: "Serum 1" }, active: true },
      { imageProps: { src: "/img/programmer-01.jpg", alt: "Serum 2" } },
      { imageProps: { src: "/img/programmer-02.jpg", alt: "Serum 3" } },
      { imageProps: { src: "/img/ux-01.jpg", alt: "Serum 4" } },
    ],
  },
  productInfoProps: {
    category: "Skin Care",
    stockBadge: "In Stock",
    titleProps: { title: "SilkSkin Serum" },
    ratingProps: { rating: 5 },
    reviewCountText: "4.8 (245 Review)",
    priceProps: { number: 48 },
    oldPrice: "$60.00",
    descriptionProps: {
      description: "Concentrated botanical serum with Vitamin C and hyaluronic acid for deep hydration and radiant skin tone.",
    },
    sizesLabel: "Size/Volume",
    sizes: [{ label: "30 ml", value: "30ml" }, { label: "60 ml", value: "60ml" }, { label: "80 ml", value: "80ml" }, { label: "100 ml", value: "100ml" }],
    showQuantity: true,
    initialQuantity: 4,
    buttonProps: { text: "Add To Cart" },
    secondaryButtonProps: { text: "Buy Now" },
    favoriteButtonProps: {},
    metaItems: [
      { label: "SKU", value: "GRFR85648HGJ" },
      { label: "Tags", value: "Skincare, Serums, Vitamin C" },
    ],
  },
  productTabsProps: {
    underlineNavProps: {
      items: [
        { value: "description", label: "Description" },
        { value: "additional", label: "Additional Information" },
        { value: "review", label: "Review" },
      ],
      value: "review",
    },
    descriptionProps: {
      description: "Dermatologist-tested formula suitable for all skin types. Free of parabens, sulfates, and artificial fragrances.",
    },
  },
  productRatingProps: {
    summaryLayout: "score-left",
    averageScore: "4.8",
    totalReviewsText: "(245 Review)",
    filterTitle: "Review List",
    resultCountText: "Showing 1-4 of 24 results",
    reviews: [
      {
        id: "bs-1", author: "Kristin Watson", verifiedText: "(Verified)", date: "1 month ago",
        headline: "Absolutely love this product!", ratingProps: { rating: 5 },
        descriptionProps: { description: "Leaves my skin feeling hydrated all day without any sticky residue. Noticeable glow after two weeks." },
        mediaImages: [
          { src: "/img/ux-01.jpg", alt: "Review photo 1" },
          { src: "/img/programmer-01.jpg", alt: "Review photo 2" },
          { src: "/img/programmer-02.jpg", alt: "Review photo 3" },
        ],
      },
      {
        id: "bs-2", author: "Jenny Wilson", verifiedText: "(Verified)", date: "2 months ago",
        headline: "Perfect for my skincare routine!", ratingProps: { rating: 5 },
        descriptionProps: { description: "Pairs seamlessly with moisturizer and sunscreen. Gentle enough for sensitive skin." },
      },
    ],
  },
};
