// @ts-nocheck
import { useState } from "react";
import Icon from "../../../../../atoms/icon/ui/Icon";
import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton";
import CardCartButton from "../../../../../molecules/card/shared/card-cart-button/ui/CardCartButton";
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription";
import CardFavoriteButton from "../../../../../molecules/card/shared/card-favorite-button/ui/CardFavoriteButton";
import CardPrice from "../../../../../molecules/card/shared/card-price/ui/CardPrice";
import CardRating from "../../../../../molecules/card/shared/card-rating/ui/CardRating";
import CardTags from "../../../../../molecules/card/shared/card-tags/ui/CardTags";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle";
import ProductTabs from "../../product-tabs/ui/ProductTabs";
import type { ProductInfoProps } from "../data/types";

export default function ProductInfo({
  category,
  badge,
  stockBadge,
  titleProps,
  subtitle,
  ratingProps,
  reviewCountText,
  priceProps,
  priceUnit,
  oldPrice,
  discountText,
  socialProofText,
  descriptionProps,
  featuresTitle = "Features:",
  features = [],
  colorsLabel = "Colors",
  colors = [],
  sizesLabel = "Size",
  sizes = [],
  purchaseOptions = [],
  showQuantity = false,
  initialQuantity = 1,
  stockCountText,
  subtotalText,
  deliveryText,
  urgencyText,
  calloutText,
  buttonProps,
  secondaryButtonProps,
  cartButtonProps,
  favoriteButtonProps,
  tagsProps,
  trustBadges = [],
  metaItems = [],
  accordionItems = [],
  jumpLinks = [],
  inlineTabsProps,
  className = "",
}: ProductInfoProps) {
  const [quantity, setQuantity] = useState(initialQuantity);
  const [selectedColor, setSelectedColor] = useState(colors[0]?.value ?? "");
  const [selectedSize, setSelectedSize] = useState(sizes[0]?.value ?? "");
  const [selectedPurchase, setSelectedPurchase] = useState(purchaseOptions[0]?.id ?? "");

  return (
    <div className={`space-y-5 ${className}`}>
      <div className="space-y-2">
        {(category || badge || stockBadge) && (
          <div className="flex flex-wrap items-center justify-between gap-2">
            <div className="flex flex-wrap items-center gap-2">
              {category && (
                <span className="text-xs font-medium uppercase tracking-wider text-blue-400">
                  {category}
                </span>
              )}
              {badge && (
                <span className="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-emerald-300">
                  {badge}
                </span>
              )}
            </div>
            {stockBadge && (
              <span className="rounded-md border border-blue-500/40 bg-blue-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-blue-300">
                {stockBadge}
              </span>
            )}
          </div>
        )}
        {titleProps && <CardTitle {...titleProps} />}
        <div className="flex flex-wrap items-center gap-3 text-xs text-zinc-400">
          {subtitle && <span>{subtitle}</span>}
          {ratingProps && (
            <div className="flex items-center gap-2">
              <CardRating {...ratingProps} />
              <span className="font-semibold text-zinc-200">{ratingProps.rating}</span>
              {reviewCountText && <span>({reviewCountText})</span>}
            </div>
          )}
        </div>
      </div>

      {(priceProps || oldPrice || discountText) && (
        <div className="space-y-1">
          <div className="flex flex-wrap items-baseline gap-2.5">
            {priceProps && <CardPrice {...priceProps} />}
            {priceUnit && <span className="text-xs text-zinc-400">{priceUnit}</span>}
            {oldPrice && <span className="text-sm text-zinc-500 line-through">{oldPrice}</span>}
          </div>
          {discountText && <p className="text-xs font-medium text-emerald-400">{discountText}</p>}
        </div>
      )}

      {socialProofText && (
        <div className="inline-flex items-center gap-2 rounded-lg border border-blender-border bg-blender-surface px-3 py-1.5 text-xs text-zinc-300">
          <span>{socialProofText}</span>
        </div>
      )}

      {descriptionProps && <CardDescription {...descriptionProps} />}

      {features.length > 0 && (
        <div className="space-y-2">
          {featuresTitle && <h4 className="text-xs font-semibold text-zinc-200">{featuresTitle}</h4>}
          <ul className="space-y-1.5 text-xs text-zinc-300">
            {features.map((feat) => (
              <li key={feat} className="flex items-center gap-2">
                <span className="text-blue-400">✓</span>
                <span>{feat}</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {calloutText && (
        <div className="rounded-xl border border-blender-border bg-blender-surface/70 p-3 text-xs text-zinc-300">
          {calloutText}
        </div>
      )}

      {colors.length > 0 && (
        <div className="space-y-2">
          <span className="block text-xs font-semibold text-zinc-300">{colorsLabel}</span>
          <div className="flex flex-wrap items-center gap-2.5">
            {colors.map((c) => (
              <button
                key={c.value}
                type="button"
                onClick={() => setSelectedColor(c.value)}
                style={{ backgroundColor: c.hex }}
                className="flex size-7 items-center justify-center rounded-md border border-blender-border text-[10px] font-bold text-white"
              >
                {selectedColor === c.value ? "✓" : ""}
              </button>
            ))}
          </div>
        </div>
      )}

      {sizes.length > 0 && (
        <div className="space-y-2">
          <span className="block text-xs font-semibold text-zinc-300">{sizesLabel}</span>
          <div className="flex flex-wrap items-center gap-2">
            {sizes.map((s) => (
              <button
                key={s.value}
                type="button"
                onClick={() => setSelectedSize(s.value)}
                className={`rounded-lg border px-3 py-1.5 text-xs font-medium ${
                  selectedSize === s.value
                    ? "border-blue-500 bg-blue-600 text-white"
                    : "border-blender-border bg-blender-surface text-zinc-300"
                }`}
              >
                {s.priceLabel ? `${s.label} = ${s.priceLabel}` : s.label}
              </button>
            ))}
          </div>
        </div>
      )}

      {purchaseOptions.length > 0 && (
        <div className="space-y-2 rounded-xl border border-blender-border bg-blender-surface/50 p-3">
          {purchaseOptions.map((opt) => (
            <button
              key={opt.id}
              type="button"
              onClick={() => setSelectedPurchase(opt.id)}
              className="flex w-full items-center justify-between rounded-lg px-3 py-2 text-xs text-zinc-200"
            >
              <span>{opt.label}</span>
              <span className="font-bold">{opt.price}</span>
            </button>
          ))}
        </div>
      )}

      {showQuantity && (
        <div className="flex items-center gap-4">
          <div className="inline-flex items-center rounded-lg border border-blender-border bg-blender-surface">
            <button
              type="button"
              onClick={() => setQuantity(Math.max(1, quantity - 1))}
              className="px-3 py-1.5 text-xs font-bold text-zinc-300"
            >
              −
            </button>
            <span className="min-w-8 text-center text-xs font-semibold text-zinc-100">{quantity}</span>
            <button
              type="button"
              onClick={() => setQuantity(quantity + 1)}
              className="px-3 py-1.5 text-xs font-bold text-zinc-300"
            >
              +
            </button>
          </div>
          {stockCountText && <span className="text-xs text-zinc-400">{stockCountText}</span>}
        </div>
      )}

      {subtotalText && (
        <p className="text-xs text-zinc-300">
          Subtotal: <span className="font-bold text-zinc-100">{subtotalText}</span>
        </p>
      )}
      {(deliveryText || urgencyText) && (
        <div className="flex flex-wrap items-center gap-3 text-xs">
          {deliveryText && <span className="text-zinc-300">{deliveryText}</span>}
          {urgencyText && <span className="font-semibold text-rose-400">{urgencyText}</span>}
        </div>
      )}

      <div className="flex flex-wrap items-center gap-2.5 pt-1">
        {buttonProps && <CardButton {...buttonProps} />}
        {secondaryButtonProps && <CardButton variant="outline" {...secondaryButtonProps} />}
        {cartButtonProps && <CardCartButton {...cartButtonProps} />}
        {favoriteButtonProps && <CardFavoriteButton {...favoriteButtonProps} />}
      </div>

      {trustBadges.length > 0 && (
        <div className="grid gap-2 pt-1 sm:grid-cols-3">
          {trustBadges.map((b) => (
            <div key={b.text} className="flex items-center gap-2 rounded-lg border border-blender-border/70 bg-blender-surface/50 px-3 py-2 text-[11px] text-zinc-300">
              {b.icon && <Icon name={b.icon} className="size-4 text-blue-400" />}
              <span>{b.text}</span>
            </div>
          ))}
        </div>
      )}

      {tagsProps && <CardTags {...tagsProps} />}

      {metaItems.length > 0 && (
        <div className="space-y-1.5 border-t border-blender-border/60 pt-3 text-xs">
          {metaItems.map((meta) => (
            <p key={meta.label} className="text-zinc-400">
              <span className="font-semibold text-zinc-200">{meta.label}:</span>
              <span className="ml-1.5">{meta.value}</span>
            </p>
          ))}
        </div>
      )}

      {accordionItems.length > 0 && (
        <div className="space-y-2 pt-2">
          {accordionItems.map((acc) => (
            <details key={acc.title} open={acc.open} className="overflow-hidden rounded-xl border border-blender-border bg-blender-card">
              <summary className="cursor-pointer px-4 py-3 text-sm font-semibold text-zinc-100">
                {acc.title}
              </summary>
              <div className="border-t border-blender-border p-4 text-xs text-zinc-400">
                {acc.content}
              </div>
            </details>
          ))}
        </div>
      )}

      {jumpLinks.length > 0 && (
        <div className="flex flex-wrap gap-1.5 pt-1">
          {jumpLinks.map((link) => (
            <span key={link} className="rounded-full border border-blender-border bg-blender-surface px-2.5 py-1 text-[11px] text-zinc-300">
              ↓ {link}
            </span>
          ))}
        </div>
      )}

      {inlineTabsProps && (
        <div className="border-t border-blender-border pt-4">
          <ProductTabs {...inlineTabsProps} />
        </div>
      )}
    </div>
  );
}
