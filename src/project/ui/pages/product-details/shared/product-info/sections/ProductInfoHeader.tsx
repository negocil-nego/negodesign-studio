// @ts-nocheck
import Icon from "../../../../../atoms/icon/ui/Icon";
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription";
import CardPrice from "../../../../../molecules/card/shared/card-price/ui/CardPrice";
import CardRating from "../../../../../molecules/card/shared/card-rating/ui/CardRating";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle";
import type { ProductInfoHeaderProps } from "../data/types";

export default function ProductInfoHeader({
  category, badge, stockBadge, titleProps, subtitle, ratingProps, reviewCountText,
  priceProps, priceUnit, oldPrice, discountText, socialProofText, descriptionProps,
  featuresTitle = "Features:", features = [], calloutText,
}: ProductInfoHeaderProps) {
  return (
    <>
      <div className="space-y-2">
        {(category || badge || stockBadge) && (
          <div className="flex flex-wrap items-center justify-between gap-2">
            <div className="flex flex-wrap items-center gap-2">
              {category && <span className="text-xs font-medium uppercase tracking-wider text-blue-400">{category}</span>}
              {badge && <span className="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-emerald-300">{badge}</span>}
            </div>
            {stockBadge && <span className="rounded-md border border-blue-500/40 bg-blue-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-blue-300">{stockBadge}</span>}
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
    </>
  );
}
