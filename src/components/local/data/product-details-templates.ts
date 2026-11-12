import type { PageProductDetailsProps } from "../../../project/ui/pages/product-details/data/types";

export const productDetailsTemplates: Record<string, PageProductDetailsProps> = {
  "PageProductDetails · Sidebar": {
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
      items: [
        { label: "Home", href: "#home" },
        { label: "Mens", href: "#mens" },
        { label: "Shirt", href: "#shirt" },
        { label: "Casual Shirt", active: true },
      ],
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
      features: [
        "Made with full cotton",
        "Slim fit for any body",
        "Quality control certified",
      ],
      colors: [
        { label: "Olive", value: "olive", hex: "#4d5d3b" },
        { label: "Beige", value: "beige", hex: "#c7b299" },
        { label: "Slate", value: "slate", hex: "#94a3b8" },
        { label: "Navy", value: "navy", hex: "#1e293b" },
      ],
      sizes: [
        { label: "L", value: "L" },
        { label: "S", value: "S" },
        { label: "XL", value: "XL" },
      ],
      buttonProps: { text: "Add to cart" },
      favoriteButtonProps: {},
      trustBadges: [
        { icon: "Globe", text: "Free shipping worldwide" },
        { icon: "Shield01", text: "100% Secured Payment" },
        { icon: "User01", text: "Made by the Professionals" },
      ],
    },
    productTabsProps: {
      underlineNavProps: {
        items: [
          { value: "description", label: "Description" },
          { value: "reviews", label: "Reviews (150)" },
          { value: "support", label: "Support" },
        ],
      },
      titleProps: {
        title: "Product Details : Premium Quality Stylish Long Sleeve Casual Shirt For Man",
      },
      specs: [
        { label: "Product Type", value: "Shirt" },
        { label: "Color", value: "Ash" },
        { label: "Main Material", value: "Cotton Stylish and fashionable" },
        { label: "Gender", value: "Men" },
      ],
      descriptionProps: {
        description:
          "Specifications of Premium Quality Stylish Long Sleeve Casual Shirt For Man Brand No Brand SKU 212212768_BD-1263689825 Main Material Cotton.",
      },
    },
    productRatingProps: {
      titleProps: {
        title: "Ratings & Reviews of Premium Quality Stylish Long Sleeve Casual Shirt For Man",
      },
      summaryLayout: "bars-left",
      averageScore: "4.5/5",
      totalReviewsText: "150 Reviews",
      recommendedPercent: "90%",
      writeReviewButtonProps: { text: "Write a Review" },
      reviews: [
        {
          id: "fh-1",
          author: "Kristin Watson",
          date: "March 14, 2026",
          ratingProps: { rating: 5 },
          descriptionProps: {
            description:
              "You made it so simple. My new site is so much faster and easier to work with than my old site. I just choose the page, make the changes.",
          },
        },
        {
          id: "fh-2",
          author: "Jenny Wilson",
          date: "January 28, 2026",
          ratingProps: { rating: 5 },
          descriptionProps: {
            description:
              "You made it so simple. My new site is so much faster and easier to work with than my old site. I just choose the page, make the changes.",
          },
        },
      ],
      loadMoreText: "LOAD MORE REVIEWS",
    },
    productSimiliarProps: {
      variant: "sidebar",
      titleProps: { title: "Similar Products" },
      items: [
        {
          variant: "02",
          imageProps: { src: "/img/programmer-01.jpg", alt: "Blazer" },
          titleProps: { title: "Black Synthetic Casual Blazer For Man" },
          priceProps: { number: 1950 },
          ratingProps: { rating: 5 },
          cartButtonProps: {},
          favoriteButtonProps: {},
        },
        {
          variant: "02",
          imageProps: { src: "/img/programmer-02.jpg", alt: "Jumper" },
          titleProps: { title: "Full Sleeve High Neck Zipper Jumper" },
          priceProps: { number: 700 },
          ratingProps: { rating: 4 },
          cartButtonProps: {},
          favoriteButtonProps: {},
        },
        {
          variant: "02",
          imageProps: { src: "/img/ux-01.jpg", alt: "Sweater" },
          titleProps: { title: "Full Sleeve Cotton Sweater For Man" },
          priceProps: { number: 300 },
          ratingProps: { rating: 5 },
          cartButtonProps: {},
          favoriteButtonProps: {},
        },
      ],
      moreButtonProps: { text: "LOAD MORE" },
    },
  },

  "PageProductDetails · Showcase": {
    variant: "showcase",
    menuProps: {
      logoText: "amble",
      menus: [
        { textProps: { text: "Weight Loss" }, href: "#weight" },
        { textProps: { text: "Mental Health" }, href: "#mental" },
        { textProps: { text: "Hair Regrowth" }, href: "#hair" },
        { textProps: { text: "How It Works" }, href: "#how" },
        { textProps: { text: "Contact" }, href: "#contact" },
      ],
      buttonsProps: [{ text: "Login", variant: "outline" }],
    },
    productGalleryProps: {
      layout: "bottom-thumbs",
      imageProps: { src: "/img/programmer-02.jpg", alt: "Home Kit" },
      minFileProps: [
        { imageProps: { src: "/img/programmer-01.jpg", alt: "Vial 1" } },
        { imageProps: { src: "/img/programmer-02.jpg", alt: "Lifestyle" }, active: true },
        { imageProps: { src: "/img/ux-01.jpg", alt: "Vial 2" } },
      ],
    },
    productInfoProps: {
      ratingProps: { rating: 5 },
      reviewCountText: "4.4 · based on 4k+ Reviews",
      titleProps: { title: "Semaglutide Injection Home Kit" },
      featuresTitle: "",
      features: [
        "Science-backed metabolic support",
        "Boost insulin function and glucose regulation",
        "Reduce cardiovascular risk factors",
      ],
      calloutText:
        "This treatment requires an assessment of your medical history by a licensed provider.",
      buttonProps: { text: "Enroll Now — $179/mo" },
      secondaryButtonProps: { text: "See If You Are Eligible" },
      descriptionProps: {
        description:
          "A once-a-week prescription kit formulated with active ingredients and guided specialist follow-ups.",
      },
      accordionItems: [
        {
          title: "Details",
          content: "Includes monthly supply, home delivery, and ongoing care team check-ins.",
        },
        {
          title: "Potential side effects",
          content: "Mild nausea or digestive adjustment may occur during the initial titration phase.",
        },
        {
          title: "Shipping",
          content: "Temperature-controlled 2-day express shipping directly from partner pharmacies.",
        },
      ],
      jumpLinks: ["About Kit", "How It Works", "Why Choose Us", "Reviews", "Learn More"],
    },
    featureCardsProps: {
      sectionGridProps: {
        titleProps: { text: "How the program works" },
        descriptionProps: {
          text: "Get a personalized plan delivered right to your door from the comfort of home.",
        },
      },
      items: [
        {
          variant: "01",
          titleProps: { title: "Mimics natural hormones" },
          descriptionProps: {
            description: "Helps amplify the effects of natural satiety signals after meals.",
          },
          imageProps: { src: "/img/ux-01.jpg", alt: "Step 1" },
        },
        {
          variant: "01",
          titleProps: { title: "Slows digestion down" },
          descriptionProps: {
            description: "Helps stabilize blood sugar levels and sustain energy throughout the day.",
          },
          imageProps: { src: "/img/programmer-01.jpg", alt: "Step 2" },
        },
        {
          variant: "01",
          titleProps: { title: "Suppresses hunger signals" },
          descriptionProps: {
            description: "Directly supports balanced appetite regulation in the central nervous system.",
          },
          imageProps: { src: "/img/programmer-02.jpg", alt: "Step 3" },
        },
      ],
    },
  },

  "PageProductDetails · Classic": {
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
        description:
          "Concentrated botanical serum with Vitamin C and hyaluronic acid for deep hydration and radiant skin tone.",
      },
      sizesLabel: "Size/Volume",
      sizes: [
        { label: "30 ml", value: "30ml" },
        { label: "60 ml", value: "60ml" },
        { label: "80 ml", value: "80ml" },
        { label: "100 ml", value: "100ml" },
      ],
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
        description:
          "Dermatologist-tested formula suitable for all skin types. Free of parabens, sulfates, and artificial fragrances.",
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
          id: "bs-1",
          author: "Kristin Watson",
          verifiedText: "(Verified)",
          date: "1 month ago",
          headline: "Absolutely love this product!",
          ratingProps: { rating: 5 },
          descriptionProps: {
            description:
              "Leaves my skin feeling hydrated all day without any sticky residue. Noticeable glow after two weeks.",
          },
          mediaImages: [
            { src: "/img/ux-01.jpg", alt: "Review photo 1" },
            { src: "/img/programmer-01.jpg", alt: "Review photo 2" },
            { src: "/img/programmer-02.jpg", alt: "Review photo 3" },
          ],
        },
        {
          id: "bs-2",
          author: "Jenny Wilson",
          verifiedText: "(Verified)",
          date: "2 months ago",
          headline: "Perfect for my skincare routine!",
          ratingProps: { rating: 5 },
          descriptionProps: {
            description:
              "Pairs seamlessly with moisturizer and sunscreen. Gentle enough for sensitive skin.",
          },
        },
      ],
    },
  },

  "PageProductDetails · Minimal": {
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
        {
          variant: "01",
          imageProps: { src: "/img/programmer-01.jpg", alt: "Opulent" },
          titleProps: { title: "OPULENT" },
          priceProps: { number: 19 },
          ratingProps: { rating: 5 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/programmer-02.jpg", alt: "Grace" },
          titleProps: { title: "GRACE" },
          priceProps: { number: 13 },
          ratingProps: { rating: 4 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/ux-01.jpg", alt: "Velvet" },
          titleProps: { title: "VELVET" },
          priceProps: { number: 23 },
          ratingProps: { rating: 5 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/programmer-01.jpg", alt: "Iridescent" },
          titleProps: { title: "IRIDESCENT" },
          priceProps: { number: 25 },
          ratingProps: { rating: 4 },
        },
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
        {
          title: "Shop",
          links: [
            { label: "Skincare" },
            { label: "Facial" },
            { label: "Soap" },
            { label: "Candles" },
            { label: "Gifts" },
          ],
        },
        {
          title: "Help Desk",
          links: [
            { label: "Chat" },
            { label: "FAQ" },
            { label: "Shipping & Returns" },
            { label: "Contact" },
            { label: "Policies" },
          ],
        },
        {
          title: "Stores",
          links: [
            { label: "Manhattan" },
            { label: "Brooklyn" },
            { label: "Tokyo" },
            { label: "Jakarta" },
            { label: "Paris" },
          ],
        },
      ],
      bottomLinks: [{ label: "Licenses" }, { label: "Privacy" }, { label: "Terms" }],
    },
  },

  "PageProductDetails · Related Grid": {
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
        description:
          "Engineered mesh upper with responsive cushioning for all-day comfort and high-traction outsole durability.",
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
        {
          variant: "01",
          imageProps: { src: "/img/programmer-01.jpg", alt: "Product 1" },
          titleProps: { title: "Fusce Dolor Enim No" },
          descriptionProps: { description: "Clothing, Footwear" },
          priceProps: { number: 40 },
          ratingProps: { rating: 4 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/programmer-02.jpg", alt: "Product 2" },
          titleProps: { title: "Lacus Dignissim Posidonium" },
          descriptionProps: { description: "Accessories, Handbag" },
          priceProps: { number: 80 },
          ratingProps: { rating: 4 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/ux-01.jpg", alt: "Product 3" },
          titleProps: { title: "Fusce Tempor Volu" },
          descriptionProps: { description: "Backpacks & Travel" },
          priceProps: { number: 55 },
          ratingProps: { rating: 4 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/programmer-01.jpg", alt: "Product 4" },
          titleProps: { title: "Integer Eget Augue Audire" },
          descriptionProps: { description: "Apparel, Hoodies" },
          priceProps: { number: 100 },
          ratingProps: { rating: 4 },
        },
      ],
    },
  },

  "PageProductDetails · Catalog Specs": {
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
      sizes: [
        { label: "40.5", value: "40.5" },
        { label: "41", value: "41" },
        { label: "42", value: "42" },
        { label: "42.5", value: "42.5" },
        { label: "43", value: "43" },
      ],
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
        description:
          "Inspired by the original AJ1, the Air Jordan 1 Mid offers fans a chance to follow in the footsteps of greatness with classic court paneling and fresh color blocking.",
      },
    },
    productRatingProps: {
      titleProps: { title: "Customer Reviews (58)" },
      averageScore: "4.9/5",
      totalReviewsText: "58 verified buyers",
      reviews: [
        {
          id: "rz-1",
          author: "Daniel K.",
          date: "19 June 2026",
          ratingProps: { rating: 5 },
          descriptionProps: {
            description:
              "Authentic pair, leather quality is soft and durable, and the green colorway looks great in person.",
          },
        },
        {
          id: "rz-2",
          author: "Elena M.",
          date: "12 June 2026",
          ratingProps: { rating: 5 },
          descriptionProps: {
            description:
              "True to EU sizing. ankle support is very comfortable for long city walks.",
          },
        },
      ],
    },
    productSimiliarProps: {
      variant: "grid",
      columns: 5,
      titleProps: { title: "Recently Viewed Products" },
      items: [
        {
          variant: "01",
          imageProps: { src: "/img/programmer-01.jpg", alt: "Item 1" },
          titleProps: { title: "24\" Discovery Trail Bike" },
          priceProps: { number: 3570 },
          ratingProps: { rating: 5 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/programmer-02.jpg", alt: "Item 2" },
          titleProps: { title: "Inverter Welder SAB-258" },
          priceProps: { number: 1659 },
          ratingProps: { rating: 5 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/ux-01.jpg", alt: "Item 3" },
          titleProps: { title: "Sparkling Brut Reserve" },
          priceProps: { number: 89 },
          ratingProps: { rating: 4 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/programmer-01.jpg", alt: "Item 4" },
          titleProps: { title: "Reusable Sport Mask" },
          priceProps: { number: 129 },
          ratingProps: { rating: 5 },
        },
        {
          variant: "01",
          imageProps: { src: "/img/programmer-02.jpg", alt: "Item 5" },
          titleProps: { title: "Wireless Optical Mouse" },
          priceProps: { number: 279 },
          ratingProps: { rating: 5 },
        },
      ],
    },
  },

  "PageProductDetails · Fresh": {
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
        description:
          "Crisp, nutrient-rich broccoli harvested fresh from our farms and delivered directly to your door. Packed with vitamins C, K, and fiber, it's the perfect choice for healthy meals.",
      },
      highlights: [
        "100% Organic",
        "Harvested within 24 hours",
        "Great for boosting immunity & digestion",
      ],
    },
    productRatingProps: undefined,
    productSimiliarProps: {
      variant: "grid",
      titleProps: { title: "You may also like" },
      columns: 4,
      items: [
        {
          variant: "01",
          imageProps: { src: "/img/programmer-02.jpg", alt: "Tomatoes" },
          titleProps: { title: "Tomatoes" },
          descriptionProps: { description: "New Arrival · 2 days Left" },
          priceProps: { number: 2.2 },
          ratingProps: { rating: 5 },
          cartButtonProps: {},
        },
        {
          variant: "01",
          imageProps: { src: "/img/programmer-01.jpg", alt: "Cucumber" },
          titleProps: { title: "Cucumber" },
          descriptionProps: { description: "Best Seller · 2 days Left" },
          priceProps: { number: 1.5 },
          ratingProps: { rating: 5 },
          cartButtonProps: {},
        },
        {
          variant: "01",
          imageProps: { src: "/img/ux-01.jpg", alt: "Eggplant" },
          titleProps: { title: "Eggplant" },
          descriptionProps: { description: "Best Seller · 2 days Left" },
          priceProps: { number: 2.5 },
          ratingProps: { rating: 5 },
          cartButtonProps: {},
        },
        {
          variant: "01",
          imageProps: { src: "/img/programmer-02.jpg", alt: "Cauliflower" },
          titleProps: { title: "Cauliflower" },
          descriptionProps: { description: "New Arrival · 2 days Left" },
          priceProps: { number: 2.5 },
          ratingProps: { rating: 5 },
          cartButtonProps: {},
        },
      ],
    },
  },
};
