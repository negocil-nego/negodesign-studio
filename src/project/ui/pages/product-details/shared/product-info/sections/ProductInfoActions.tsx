// @ts-nocheck
import Icon from "../../../../../atoms/icon/ui/Icon";
import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton";
import CardCartButton from "../../../../../molecules/card/shared/card-cart-button/ui/CardCartButton";
import CardFavoriteButton from "../../../../../molecules/card/shared/card-favorite-button/ui/CardFavoriteButton";
import CardTags from "../../../../../molecules/card/shared/card-tags/ui/CardTags";
import ProductTabs from "../../product-tabs/ui/ProductTabs";
import type { ProductInfoActionsProps } from "../data/types";

export default function ProductInfoActions({
  buttonProps, secondaryButtonProps, cartButtonProps, favoriteButtonProps,
  tagsProps, trustBadges = [], metaItems = [], accordionItems = [], jumpLinks = [],
  inlineTabsProps,
}: ProductInfoActionsProps) {
  return (
    <>
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
    </>
  );
}
