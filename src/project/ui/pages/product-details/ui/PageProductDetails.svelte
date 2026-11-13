<script lang="ts">
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import Breadcrumbs from "../../../molecules/breadcrumb/ui/Breadcrumbs.svelte";
  import Footer from "../../../organisms/footer/ui/Footer.svelte";
  import Menu from "../../../organisms/menu/simple/ui/Menu.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import type { PageProductDetailsProps } from "../data/types";
  import ProductDetailsSidebarLayout from "./layouts/ProductDetailsSidebarLayout.svelte";
  import ProductDetailsCatalogLayout from "./layouts/ProductDetailsCatalogLayout.svelte";
  import ProductDetailsDefaultLayout from "./layouts/ProductDetailsDefaultLayout.svelte";

  let {
    variant = "sidebar", topBarText, topBarRightText,
    menuProps = { logoText: "Negodesign Store",
      menus: [{ textProps: { text: "Home" }, href: "#home" }, { textProps: { text: "Shop" }, href: "#shop" }, { textProps: { text: "Categories" }, href: "#categories" }, { textProps: { text: "About Us" }, href: "#about" }, { textProps: { text: "Contact" }, href: "#contact" }],
      buttonsProps: [{ text: "Sign In", variant: "outline" }],
    },
    breadcrumbProps = { items: [{ label: "Home", href: "#home" }, { label: "Shop", href: "#shop" }, { label: "Product Details", active: true }] },
    bannerTitle, backButtonText,
    productGalleryProps = { layout: "left-thumbs", imageProps: { src: "/img/programmer-01.jpg", alt: "Product Main" },
      minFileProps: [{ imageProps: { src: "/img/programmer-01.jpg", alt: "Thumb 1" }, active: true }, { imageProps: { src: "/img/programmer-02.jpg", alt: "Thumb 2" } }, { imageProps: { src: "/img/ux-01.jpg", alt: "Thumb 3" } }],
    },
    productInfoProps = {
      titleProps: { title: "Long Sleeve Casual Shirt For Man" }, ratingProps: { rating: 5 }, reviewCountText: "150 Reviews",
      priceProps: { number: 300 }, oldPrice: "$500", discountText: "Save 50% right now",
      features: ["Made with full cotton", "Slim fit for any body", "Quality control certified"],
      colors: [{ label: "Olive", value: "olive", hex: "#4d5d3b" }, { label: "Sand", value: "sand", hex: "#c7b299" }, { label: "Slate", value: "slate", hex: "#64748b" }, { label: "Navy", value: "navy", hex: "#1e293b" }],
      sizes: [{ label: "S", value: "S" }, { label: "M", value: "M" }, { label: "L", value: "L" }, { label: "XL", value: "XL" }],
      buttonProps: { text: "Add to cart" }, favoriteButtonProps: {},
      trustBadges: [{ icon: "Globe", text: "Free shipping worldwide" }, { icon: "Shield01", text: "100% Secured Payment" }, { icon: "User01", text: "Made by the Professionals" }],
    },
    productTabsProps = { titleProps: { title: "Product Details : Premium Quality Stylish Long Sleeve Casual Shirt" },
      specs: [{ label: "Product Type", value: "Shirt" }, { label: "Color", value: "Ash / Olive" }, { label: "Main Material", value: "Cotton Stylish and fashionable" }, { label: "Gender", value: "Men" }],
      descriptionProps: { description: "Specifications of Premium Quality Stylish Long Sleeve Casual Shirt For Man. Crafted with breathable combed cotton and reinforced seams for daily comfort." },
    },
    productRatingProps = { titleProps: { title: "Ratings & Reviews of Premium Quality Stylish Long Sleeve Casual Shirt" },
      averageScore: "4.5/5", totalReviewsText: "150 Reviews", recommendedPercent: "90%", writeReviewButtonProps: { text: "Write a Review" },
      reviews: [
        { id: "r1", author: "Kristin Watson", verifiedText: "Verified", date: "March 14, 2026", ratingProps: { rating: 5 }, descriptionProps: { description: "You made it so simple. My new site is so much faster and easier to work with than my old site." } },
        { id: "r2", author: "Jenny Wilson", verifiedText: "Verified", date: "January 28, 2026", ratingProps: { rating: 5 }, descriptionProps: { description: "Excellent build quality and fabric texture. Fits true to size and arrived ahead of schedule." } },
      ],
      loadMoreText: "LOAD MORE REVIEWS",
    },
    productSimiliarProps = { variant: "sidebar", titleProps: { title: "Similar Products" },
      items: [
        { variant: "02", imageProps: { src: "/img/programmer-01.jpg", alt: "Blazer" }, titleProps: { title: "Black Synthetic Casual Blazer" }, priceProps: { number: 1950 }, ratingProps: { rating: 5 }, cartButtonProps: {}, favoriteButtonProps: {} },
        { variant: "02", imageProps: { src: "/img/programmer-02.jpg", alt: "Jumper" }, titleProps: { title: "Full Sleeve High Neck Jumper" }, priceProps: { number: 700 }, ratingProps: { rating: 4 }, cartButtonProps: {}, favoriteButtonProps: {} },
        { variant: "02", imageProps: { src: "/img/ux-01.jpg", alt: "Sweater" }, titleProps: { title: "Full Sleeve Cotton Sweater" }, priceProps: { number: 300 }, ratingProps: { rating: 5 }, cartButtonProps: {}, favoriteButtonProps: {} },
      ],
      moreButtonProps: { text: "LOAD MORE" },
    },
    featureCardsProps,
    footerProps = {
      logoText: "Negodesign Store", descriptionProps: { text: "Modern e-commerce experiences built with composable UI molecules and organisms." },
      contactItems: [{ icon: "Location01", text: "789 Wellington Road, San Francisco" }, { icon: "Mail01", text: "store@negodesign.studio" }, { icon: "Call", text: "+880 123 45678" }],
      columns: [
        { title: "Company", links: [{ label: "Home" }, { label: "Great Deals" }, { label: "Blog" }, { label: "About Us" }, { label: "Contact Us" }] },
        { title: "Help", links: [{ label: "User FAQs" }, { label: "How to Buy" }, { label: "Report Abuse" }, { label: "Customer Services" }, { label: "Returns and Refunds" }] },
      ],
      newsletterProps: { title: "Newsletter", placeholder: "Enter your email", buttonProps: { text: "Subscribe Now" } },
      paymentBadges: ["VISA", "MasterCard", "PayPal"],
    },
    className = "",
  }: PageProductDetailsProps = $props();

  const S = StyleComponent.pages.productDetails;
  const sectionProps = $derived({ productGalleryProps, productInfoProps, productTabsProps, productRatingProps, productSimiliarProps });
</script>

<div class={`${S.root} ${className}`}>
  {#if topBarText || topBarRightText}
    <div class={S.topUtilityBar}><div class={S.topUtilityInner}><span>{topBarText ?? ""}</span>{#if topBarRightText}<span>{topBarRightText}</span>{/if}</div></div>
  {/if}
  {#if menuProps}
    <header class={S.navWrap}><div class={S.navInner}><Menu {...menuProps} /></div></header>
  {/if}
  {#if bannerTitle}
    <div class={S.bannerHeader}><h1 class={S.bannerTitle}>{bannerTitle}</h1>
      {#if breadcrumbProps}<div class="mt-2 flex justify-center"><Breadcrumbs {...breadcrumbProps} /></div>{/if}
    </div>
  {:else if breadcrumbProps || backButtonText}
    <div class={S.breadcrumbBar}><div class="flex flex-wrap items-center gap-3">
      {#if backButtonText}<button type="button" class={S.backBtn}><Icon name="ArrowLeft01" className="size-3.5" /><span>{backButtonText}</span></button>{/if}
      {#if breadcrumbProps}<Breadcrumbs {...breadcrumbProps} />{/if}
    </div></div>
  {/if}
  <main class={S.container}>
    {#if variant === "sidebar"}
      <ProductDetailsSidebarLayout {...sectionProps} />
    {:else if variant === "catalog"}
      <ProductDetailsCatalogLayout {...sectionProps} />
    {:else}
      <ProductDetailsDefaultLayout {...sectionProps} {variant} {featureCardsProps} />
    {/if}
  </main>
  {#if footerProps}<Footer {...footerProps} />{/if}
</div>
