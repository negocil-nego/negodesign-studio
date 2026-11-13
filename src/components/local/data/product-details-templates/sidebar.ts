import type { PageProductDetailsProps } from "../../../../project/ui/pages/product-details/data/types";

export const sidebar: PageProductDetailsProps = {
  variant: "sidebar",
  topBarText: "user@gmail.com  ·  +880 123 45678",
  topBarRightText: "EN / USD",
  menuProps: {
    logoText: "Fashion House",
    menus: [
      { textProps: { text: "Home" }, href: "#home" },
      { textProps: { text: "Categories" }, href: "#categories" },
      { textProps: { text: "Great Deals" }, href: "#deals" },
      { textProps: { text: "Blog" }, href: "#blog" },
      { textProps: { text: "About Us" }, href: "#about" },
      { textProps: { text: "Contact Us" }, href: "#contact" },
    ],
    buttonsProps: [{ text: "My Account", variant: "outline" }],
  },
  breadcrumbProps: {
    items: [{ label: "Home", href: "#home" }, { label: "Mens", href: "#mens" }, { label: "Shirt", href: "#shirt" }, { label: "Casual Shirt", active: true }],
  },
  productGalleryProps: {
    layout: "left-thumbs",
    imageProps: { src: "/img/programmer-01.jpg", alt: "Casual Shirt" },
    minFileProps: [
      { imageProps: { src: "/img/programmer-01.jpg", alt: "Thumb 1" }, active: true },
      { imageProps: { src: "/img/programmer-02.jpg", alt: "Thumb 2" } },
      { imageProps: { src: "/img/ux-01.jpg", alt: "Thumb 3" } },
    ],
  },
  productInfoProps: {
    titleProps: { title: "Long Sleeve Casual Shirt For Man" },
    ratingProps: { rating: 5 },
    reviewCountText: "150 Reviews",
    priceProps: { number: 300 },
    oldPrice: "$500",
    discountText: "Save 50% right now",
    featuresTitle: "Features:",
    features: ["Made with full cotton", "Slim fit for any body", "Quality control certified"],
    colors: [{ label: "Olive", value: "olive", hex: "#4d5d3b" }, { label: "Beige", value: "beige", hex: "#c7b299" }, { label: "Slate", value: "slate", hex: "#94a3b8" }, { label: "Navy", value: "navy", hex: "#1e293b" }],
    sizes: [{ label: "L", value: "L" }, { label: "S", value: "S" }, { label: "XL", value: "XL" }],
    buttonProps: { text: "Add to cart" },
    favoriteButtonProps: {},
    trustBadges: [{ icon: "Globe", text: "Free shipping worldwide" }, { icon: "Shield01", text: "100% Secured Payment" }, { icon: "User01", text: "Made by the Professionals" }],
  },
  productTabsProps: {
    underlineNavProps: {
      items: [
        { value: "description", label: "Description" },
        { value: "reviews", label: "Reviews (150)" },
        { value: "support", label: "Support" },
      ],
    },
    titleProps: { title: "Product Details : Premium Quality Stylish Long Sleeve Casual Shirt For Man" },
    specs: [{ label: "Product Type", value: "Shirt" }, { label: "Color", value: "Ash" }, { label: "Main Material", value: "Cotton Stylish and fashionable" }, { label: "Gender", value: "Men" }],
    descriptionProps: { description: "Specifications of Premium Quality Stylish Long Sleeve Casual Shirt For Man Brand No Brand SKU 212212768_BD-1263689825 Main Material Cotton." },
  },
  productRatingProps: {
    titleProps: { title: "Ratings & Reviews of Premium Quality Stylish Long Sleeve Casual Shirt For Man" },
    summaryLayout: "bars-left",
    averageScore: "4.5/5",
    totalReviewsText: "150 Reviews",
    recommendedPercent: "90%",
    writeReviewButtonProps: { text: "Write a Review" },
    reviews: [
      {
        id: "fh-1", author: "Kristin Watson", date: "March 14, 2026", ratingProps: { rating: 5 },
        descriptionProps: { description: "You made it so simple. My new site is so much faster and easier to work with than my old site. I just choose the page, make the changes." },
      },
      {
        id: "fh-2", author: "Jenny Wilson", date: "January 28, 2026", ratingProps: { rating: 5 },
        descriptionProps: { description: "You made it so simple. My new site is so much faster and easier to work with than my old site. I just choose the page, make the changes." },
      },
    ],
    loadMoreText: "LOAD MORE REVIEWS",
  },
  productSimiliarProps: {
    variant: "sidebar",
    titleProps: { title: "Similar Products" },
    items: [
      {
        variant: "02", imageProps: { src: "/img/programmer-01.jpg", alt: "Blazer" },
        titleProps: { title: "Black Synthetic Casual Blazer For Man" }, priceProps: { number: 1950 }, ratingProps: { rating: 5 }, cartButtonProps: {}, favoriteButtonProps: {},
      },
      {
        variant: "02", imageProps: { src: "/img/programmer-02.jpg", alt: "Jumper" },
        titleProps: { title: "Full Sleeve High Neck Zipper Jumper" }, priceProps: { number: 700 }, ratingProps: { rating: 4 }, cartButtonProps: {}, favoriteButtonProps: {},
      },
      {
        variant: "02", imageProps: { src: "/img/ux-01.jpg", alt: "Sweater" },
        titleProps: { title: "Full Sleeve Cotton Sweater For Man" }, priceProps: { number: 300 }, ratingProps: { rating: 5 }, cartButtonProps: {}, favoriteButtonProps: {},
      },
    ],
    moreButtonProps: { text: "LOAD MORE" },
  },
};
