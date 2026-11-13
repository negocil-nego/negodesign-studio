// @ts-nocheck
import { useState } from "react";
import type { ProductGalleryProps } from "../data/types";
import ProductGalleryBottom from "./gallery/ProductGalleryBottom";
import ProductGalleryGrid from "./gallery/ProductGalleryGrid";
import ProductGalleryLeft from "./gallery/ProductGalleryLeft";

export default function ProductGallery({
  layout = "bottom-thumbs",
  badge,
  showArrows = false,
  imageProps,
  videoProps,
  thumbnailVideoProps,
  minFileProps = [],
  galleryImages = [],
  className = "",
}: ProductGalleryProps) {
  const [activeIdx, setActiveIdx] = useState(0);

  const allThumbs =
    minFileProps.length > 0
      ? minFileProps
      : galleryImages.map((img, i) => ({
          imageProps: img,
          active: i === activeIdx,
        }));

  const currentImage = allThumbs[activeIdx]?.imageProps ??
    imageProps ?? {
      src: "/img/ux-01.jpg",
      alt: "Product preview",
    };

  const prevSlide = () => {
    if (!allThumbs.length) return;
    setActiveIdx((activeIdx - 1 + allThumbs.length) % allThumbs.length);
  };

  const nextSlide = () => {
    if (!allThumbs.length) return;
    setActiveIdx((activeIdx + 1) % allThumbs.length);
  };

  if (layout === "grid-gallery") {
    return (
      <ProductGalleryGrid
        currentImage={currentImage}
        allThumbs={allThumbs}
        activeIdx={activeIdx}
        badge={badge}
        className={className}
        onSelect={setActiveIdx}
      />
    );
  }

  if (layout === "left-thumbs") {
    return (
      <ProductGalleryLeft
        currentImage={currentImage}
        allThumbs={allThumbs}
        activeIdx={activeIdx}
        badge={badge}
        videoProps={videoProps}
        thumbnailVideoProps={thumbnailVideoProps}
        className={className}
        onSelect={setActiveIdx}
        onPrev={prevSlide}
        onNext={nextSlide}
      />
    );
  }

  return (
    <ProductGalleryBottom
      currentImage={currentImage}
      allThumbs={allThumbs}
      activeIdx={activeIdx}
      badge={badge}
      showArrows={showArrows}
      className={className}
      onSelect={setActiveIdx}
      onPrev={prevSlide}
      onNext={nextSlide}
    />
  );
}
