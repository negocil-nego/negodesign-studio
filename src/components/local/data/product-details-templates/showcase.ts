import type { PageProductDetailsProps } from "../../../../project/ui/pages/product-details/data/types";

export const showcase: PageProductDetailsProps = {
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
    calloutText: "This treatment requires an assessment of your medical history by a licensed provider.",
    buttonProps: { text: "Enroll Now — $179/mo" },
    secondaryButtonProps: { text: "See If You Are Eligible" },
    descriptionProps: {
      description: "A once-a-week prescription kit formulated with active ingredients and guided specialist follow-ups.",
    },
    accordionItems: [
      { title: "Details", content: "Includes monthly supply, home delivery, and ongoing care team check-ins." },
      { title: "Potential side effects", content: "Mild nausea or digestive adjustment may occur during the initial titration phase." },
      { title: "Shipping", content: "Temperature-controlled 2-day express shipping directly from partner pharmacies." },
    ],
    jumpLinks: ["About Kit", "How It Works", "Why Choose Us", "Reviews", "Learn More"],
  },
  featureCardsProps: {
    sectionGridProps: {
      titleProps: { text: "How the program works" },
      descriptionProps: { text: "Get a personalized plan delivered right to your door from the comfort of home." },
    },
    items: [
      {
        variant: "01",
        titleProps: { title: "Mimics natural hormones" },
        descriptionProps: { description: "Helps amplify the effects of natural satiety signals after meals." },
        imageProps: { src: "/img/ux-01.jpg", alt: "Step 1" },
      },
      {
        variant: "01",
        titleProps: { title: "Slows digestion down" },
        descriptionProps: { description: "Helps stabilize blood sugar levels and sustain energy throughout the day." },
        imageProps: { src: "/img/programmer-01.jpg", alt: "Step 2" },
      },
      {
        variant: "01",
        titleProps: { title: "Suppresses hunger signals" },
        descriptionProps: { description: "Directly supports balanced appetite regulation in the central nervous system." },
        imageProps: { src: "/img/programmer-02.jpg", alt: "Step 3" },
      },
    ],
  },
};
