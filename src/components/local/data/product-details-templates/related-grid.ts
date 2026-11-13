import type { PageProductDetailsProps } from "../../../../project/ui/pages/product-details/data/types";

export const relatedGrid: PageProductDetailsProps = {
  variant: "classic",
  topBarText: "Mid-season sale up to 20% OFF. Use code 'SALEOFF20'",
  topBarRightText: "Eng · USD",
  menuProps: {
    logoText: "Pedona.",
    menus: [
      { textProps: { text: "HOME" }, href: "#home" },
      { textProps: { text: "SHOP" }, href: "#shop" },
      { textProps: { text: "BLOG" }, href: "#blog" },
      { textProps: { text: "PORTFOLIO" }, href: "#portfolio" },
      { textProps: { text: "PAGES" }, href: "#pages" },
      { textProps: { text: "CONTACT US" }, href: "#contact" },
    ],
  },
  breadcrumbProps: {
    items: [
      { label: "Home", href: "#home" },
      { label: "Shop", href: "#shop" },
      { label: "Clothing", href: "#clothing" },
      { label: "Sneakers & Footwear", active: true },
    ],
  },
  productGalleryProps: {
    layout: "bottom-thumbs",
    showArrows: true,
    imageProps: { src: "/img/programmer-02.jpg", alt: "Athletic Sneaker" },
    minFileProps: [
      { imageProps: { src: "/img/programmer-02.jpg", alt: "Angle 1" }, active: true },
      { imageProps: { src: "/img/programmer-01.jpg", alt: "Angle 2" } },
      { imageProps: { src: "/img/ux-01.jpg", alt: "Angle 3" } },
      { imageProps: { src: "/img/programmer-02.jpg", alt: "Angle 4" } },
    ],
  },
  productInfoProps: {
    titleProps: { title: "Ipsum Imperdie Omittam Inciderint" },
    ratingProps: { rating: 4 },
    reviewCountText: "1 customer review",
    priceProps: { number: 100 },
    descriptionProps: {
      description: "Engineered mesh upper with responsive cushioning for all-day comfort and high-traction outsole durability.",
    },
    showQuantity: true,
    buttonProps: { text: "ADD TO CART" },
    wishlistText: "Browse Wishlist",
    compareText: "Add to compare",
    metaItems: [
      { label: "Categories", value: "Clothing, Footwear & Running" },
      { label: "Tag", value: "sneakers, sport" },
    ],
  },
  productTabsProps: {
    underlineNavProps: {
      items: [
        { value: "description", label: "Description" },
        { value: "reviews", label: "Reviews (1)" },
      ],
    },
    paragraphs: [
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam fringilla augue nec est tristique auctor. Donec non est at libero vulputate rutrum.",
      "Pellentesque aliquet, sem eget laoreet ultrices, ipsum metus feugiat sem, quis fermentum turpis eros eget velit. Donec ac tempus ante.",
    ],
  },
  productRatingProps: undefined,
  productSimiliarProps: {
    variant: "grid",
    headerAlign: "center",
    titleProps: { title: "Related products" },
    columns: 4,
    items: [
      { variant: "01", imageProps: { src: "/img/programmer-01.jpg", alt: "Product 1" }, titleProps: { title: "Fusce Dolor Enim No" }, descriptionProps: { description: "Clothing, Footwear" }, priceProps: { number: 40 }, ratingProps: { rating: 4 } },
      { variant: "01", imageProps: { src: "/img/programmer-02.jpg", alt: "Product 2" }, titleProps: { title: "Lacus Dignissim Posidonium" }, descriptionProps: { description: "Accessories, Handbag" }, priceProps: { number: 80 }, ratingProps: { rating: 4 } },
      { variant: "01", imageProps: { src: "/img/ux-01.jpg", alt: "Product 3" }, titleProps: { title: "Fusce Tempor Volu" }, descriptionProps: { description: "Backpacks & Travel" }, priceProps: { number: 55 }, ratingProps: { rating: 4 } },
      { variant: "01", imageProps: { src: "/img/programmer-01.jpg", alt: "Product 4" }, titleProps: { title: "Integer Eget Augue Audire" }, descriptionProps: { description: "Apparel, Hoodies" }, priceProps: { number: 100 }, ratingProps: { rating: 4 } },
    ],
  },
};
