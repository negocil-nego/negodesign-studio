<script lang="ts">
  import ProductInfoHeader from "../sections/ProductInfoHeader.svelte";
  import ProductInfoOptions from "../sections/ProductInfoOptions.svelte";
  import ProductInfoActions from "../sections/ProductInfoActions.svelte";
  import type { ProductInfoProps } from "../data/types";

  let {
    category, badge, stockBadge, titleProps, subtitle, ratingProps, reviewCountText,
    priceProps, priceUnit, oldPrice, discountText, socialProofText, descriptionProps,
    featuresTitle, features, colorsLabel, colors, sizesLabel, sizes, purchaseOptions,
    showQuantity, initialQuantity = 1, stockCountText, subtotalText, deliveryText,
    urgencyText, calloutText, buttonProps, secondaryButtonProps, cartButtonProps,
    favoriteButtonProps, wishlistText, compareText, tagsProps, trustBadges, metaItems,
    accordionItems, jumpLinks, inlineTabsProps, className = "",
  }: ProductInfoProps = $props();

  // svelte-ignore state_referenced_locally
  let quantity = $state(initialQuantity);
  let selectedColor = $state("");
  let selectedSize = $state("");
  let selectedPurchase = $state("");
</script>

<div class={`space-y-5 ${className}`}>
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
    bind:quantity bind:selectedColor bind:selectedSize bind:selectedPurchase
    stockCountText={stockCountText} subtotalText={subtotalText}
    deliveryText={deliveryText} urgencyText={urgencyText}
  />
  <ProductInfoActions
    buttonProps={buttonProps} secondaryButtonProps={secondaryButtonProps}
    cartButtonProps={cartButtonProps} favoriteButtonProps={favoriteButtonProps}
    wishlistText={wishlistText} compareText={compareText} tagsProps={tagsProps}
    trustBadges={trustBadges} metaItems={metaItems} accordionItems={accordionItems}
    jumpLinks={jumpLinks} inlineTabsProps={inlineTabsProps}
  />
</div>
