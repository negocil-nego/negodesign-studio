// @ts-nocheck
import { useState } from "react";
import ProductInfoHeader from "../sections/ProductInfoHeader";
import ProductInfoOptions from "../sections/ProductInfoOptions";
import ProductInfoActions from "../sections/ProductInfoActions";
import type { ProductInfoProps } from "../data/types";

export default function ProductInfo({
  category, badge, stockBadge, titleProps, subtitle, ratingProps, reviewCountText,
  priceProps, priceUnit, oldPrice, discountText, socialProofText, descriptionProps,
  featuresTitle = "Features:", features = [], colorsLabel = "Colors", colors = [],
  sizesLabel = "Size", sizes = [], purchaseOptions = [], showQuantity = false,
  initialQuantity = 1, stockCountText, subtotalText, deliveryText, urgencyText,
  calloutText, buttonProps, secondaryButtonProps, cartButtonProps, favoriteButtonProps,
  tagsProps, trustBadges = [], metaItems = [], accordionItems = [], jumpLinks = [],
  inlineTabsProps, className = "",
}: ProductInfoProps) {
  const [quantity, setQuantity] = useState(initialQuantity);
  const [selectedColor, setSelectedColor] = useState(colors[0]?.value ?? "");
  const [selectedSize, setSelectedSize] = useState(sizes[0]?.value ?? "");
  const [selectedPurchase, setSelectedPurchase] = useState(purchaseOptions[0]?.id ?? "");

  return (
    <div className={`space-y-5 ${className}`}>
      <ProductInfoHeader
        category={category} badge={badge} stockBadge={stockBadge} titleProps={titleProps}
        subtitle={subtitle} ratingProps={ratingProps} reviewCountText={reviewCountText}
        priceProps={priceProps} priceUnit={priceUnit} oldPrice={oldPrice}
        discountText={discountText} socialProofText={socialProofText}
        descriptionProps={descriptionProps} featuresTitle={featuresTitle} features={features}
        calloutText={calloutText}
      />
      <ProductInfoOptions
        colorsLabel={colorsLabel} colors={colors} sizesLabel={sizesLabel} sizes={sizes}
        purchaseOptions={purchaseOptions} showQuantity={showQuantity}
        quantity={quantity} selectedColor={selectedColor} selectedSize={selectedSize}
        selectedPurchase={selectedPurchase} onQuantityChange={setQuantity}
        onSelectedColorChange={setSelectedColor} onSelectedSizeChange={setSelectedSize}
        onSelectedPurchaseChange={setSelectedPurchase} stockCountText={stockCountText}
        subtotalText={subtotalText} deliveryText={deliveryText} urgencyText={urgencyText}
      />
      <ProductInfoActions
        buttonProps={buttonProps} secondaryButtonProps={secondaryButtonProps}
        cartButtonProps={cartButtonProps} favoriteButtonProps={favoriteButtonProps}
        tagsProps={tagsProps} trustBadges={trustBadges} metaItems={metaItems}
        accordionItems={accordionItems} jumpLinks={jumpLinks} inlineTabsProps={inlineTabsProps}
      />
    </div>
  );
}
